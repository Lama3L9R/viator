use proc_macro::TokenStream;
use anyhow::anyhow;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Fields, Ident, ItemStruct, Meta, Token, Type};
use syn::parse::{Parse, ParseStream};

type TokStream = proc_macro2::TokenStream;

macro_rules! field_has_attr {
    ($field: ident, $text: literal) => {
        $field.attrs.iter().any(|x| {
            if let Meta::Path(path) = &x.meta {
                return path.is_ident("skip")
            }

            return false
        })
    };
}

macro_rules! drop_attr {
    ($field: ident, $text: literal) => {
        $field.attrs = $field.attrs.clone().into_iter().filter(|attr| {
            if let Meta::Path(path) = &attr.meta {
                !path.is_ident($text)
            } else {
                false
            }
        }).collect();
    };
}

struct StructInfo<> {
    stt: ItemStruct,
    target_fields: Vec<Ident>,
    skipped_fields: Vec<Ident>,
}

struct AutoLuaArgs {
    params: Vec<String>
}

impl Parse for AutoLuaArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params = Vec::new();

        while !input.is_empty() {
            let param = input.parse::<Ident>();
            if let Ok(param) = param {
                params.push(param.to_string());
            }

            if let Err(_) = input.parse::<Token![,]>() {
                break;
            }
        }

        Ok(Self { params })
    }
}

///
/// Auto generate IntoLua and/or FromLua
///
/// ```
/// #[autolua(Into, RefInto, From)]
/// struct MyLuaData {
///     number: u32,
///     str: String,
///
///     #[skip]
///     skip_me: SomeOtherStuff,
///
///     dont_deluaify: mlua::Value,
///     keep_the_original_taste: mlua::Table
/// }
/// ```
///
/// - `Into`: will generate IntoLua implementation on MyLuaData.
/// - `RefInto`: will generate IntoLua implementation on &MyLuaData with all fields Clone-ed
/// - `From`: will generate FromLua implementation strictly from a Table type
///
/// Note: if `#[skip]` is used on a field, the type of the field will
/// be transformed into `viator_utils::MaybeValue<T>`,
/// which impl Deref, and will panic if null is being used / deref-ed.
///
#[proc_macro_attribute]
pub fn autolua(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AutoLuaArgs);

    // We do nothing if nothing to auto impl
    if args.params.is_empty() {
        return input;
    }

    let mut into = false;
    let mut from = false;
    let mut ref_into = false;

    for param in &args.params {
        match param.as_str() {
            "Into" => into = true,
            "From" => from = true,
            "RefInto" => ref_into = true,

            _ => {}
        }
    }

    let input = parse_macro_input!(input as ItemStruct);
    let info = transform_struct(input).unwrap();
    let mut tok_stream: TokStream = recreate_struct(&info).unwrap();

    if from {
        let from_toks = gen_from_lua(&info).unwrap();

        tok_stream = quote! {
            #tok_stream
            #from_toks
        }
    }

    if into {
        let into_toks = gen_into_lua(&info).unwrap();

        tok_stream = quote! {
            #tok_stream
            #into_toks
        }
    }

    if ref_into {
        let ref_to_toks = gen_into_lua_ref(&info).unwrap();

        tok_stream = quote! {
            #tok_stream
            #ref_to_toks
        }
    }

    return tok_stream.into()
}

fn gen_maybe_wrapper(ttype: TokenStream) -> TokStream {
    let ttype: TokStream = ttype.into();

    return quote! { viator_utils::maybe_value::MaybeValue<#ttype> };
}

fn transform_struct(mut target: ItemStruct) -> anyhow::Result<StructInfo> {
    let input_fields = if let Fields::Named(named) = &mut target.fields {
        named
    } else {
        return Err(anyhow!("Only full struct is supported!"))
    };

    let mut regular_fields: Vec<Ident> = Vec::new();
    let mut skipped_fields: Vec<Ident> = Vec::new();

    for field in &mut input_fields.named {
        if field_has_attr!(field, "skip") {

            // Add MaybeValue wrapper to original type
            field.ty = Type::Verbatim(gen_maybe_wrapper(field.ty.clone().into_token_stream().into()));

            // Drop processed attribute (which is undefined to rustc)
            drop_attr!(field, "skip");
            drop_attr!(field, "hidden_彩蛋哦");

            // Random quote from quote.lama.icu
            //
            // English (Translated by Gemini 3 Fast):
            //      This unfamiliar place is but a fragment of the scenery,
            //      a fleeting station in the soul's long pilgrimage;
            //      the road that lies ahead remains, as it ever was, an interrogation of the infinite.
            //
            // for whoever wish to include this in your prog:
            // #[此一处陌生的地方_不过是心魂之旅中的一处景观_一次际遇_未来的路途一样还是无限之问]
            drop_attr!(field, "此一处陌生的地方_不过是心魂之旅中的一处景观_一次际遇_未来的路途一样还是无限之问");

            skipped_fields.push(field.ident.clone().unwrap());
        } else {
            regular_fields.push(field.ident.clone().unwrap());
        }
    }

    return Ok(StructInfo {
        stt: target,
        target_fields: regular_fields,
        skipped_fields
    })
}

fn recreate_struct(target: &StructInfo) -> anyhow::Result<TokStream> {
    let stt = &target.stt;

    return Ok(quote! {
        #stt
    })
}

fn gen_from_lua(target: &StructInfo) -> anyhow::Result<TokStream> {
    let name = target.stt.ident.clone();

    let regular_fields = target.target_fields.iter().map(|it| {
        quote! {
            #it: table.get(stringify!(#it))?,
        }
    }).collect::<TokStream>();

    let skipped_fields = target.skipped_fields.iter().map(|it| {
        quote! {
            #it: viator_utils::maybe!(null),
        }
    }).collect::<TokStream>();

    let implementation: TokStream = quote! {
        impl FromLua for #name {
            fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
                return match value {
                    mlua::Value::Table(table) => {
                        Ok(
                            Self {
                                #regular_fields
                                #skipped_fields
                            }
                        )
                    }

                    _ => {
                        Err(anyhow!("Unable to convert such value into {} struct", stringify!(#name)).into())
                    }
                }
            }
        }
    }.into();

    Ok(quote! {
        #implementation
    })
}

fn gen_into_lua(target: &StructInfo) -> anyhow::Result<TokStream> {
    let name = target.stt.ident.clone();

    let combined_fields = target.target_fields.iter().map(|it| {
        quote! {
            tbl.set(stringify!(#it), self.#it)?;
        }
    }).collect::<TokStream>();

    return Ok(quote! {
        impl mlua::IntoLua for #name {
            fn into_lua(self, lua: &mlua::Lua) -> Result<mlua::Value, mlua::Error> {
                let tbl = lua.create_table()?;

                #combined_fields

                return Ok(mlua::Value::Table(tbl));
            }
        }
    })
}

fn gen_into_lua_ref(target: &StructInfo) -> anyhow::Result<TokStream> {
    let name = target.stt.ident.clone();

    let combined_fields = target.target_fields.iter().map(|it| {
        quote! {
            tbl.set(stringify!(#it), self.#it.clone())?;
        }
    }).collect::<TokStream>();

    return Ok(quote! {
        impl mlua::IntoLua for &#name {
            fn into_lua(self, lua: &mlua::Lua) -> Result<mlua::Value, mlua::Error> {
                let tbl = lua.create_table()?;

                #combined_fields

                return Ok(mlua::Value::Table(tbl));
            }
        }
    })
}