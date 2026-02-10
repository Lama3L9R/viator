use anyhow::anyhow;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Fields, ItemStruct, Token, Type};
use syn::parse::{Parse, ParseStream};
use viator_utils::deduce_enum;
use crate::{gen_maybe_wrapper};

macro_rules! field_has_attr {
    (PureTag, $field: ident, $text: literal) => {
        $field.attrs.iter().any(|x| {
            if let syn::Meta::Path(path) = &x.meta {
                return path.is_ident($text)
            }

            return false
        })
    };

    (WithParam, $field: ident, $text: literal) => {
        $field.attrs.iter().any(|x| {
            if let syn::Meta::List(list) = &x.meta {
                return list.path.is_ident($text)
            }

            return false
        })
    };
}

macro_rules! drop_attr {
    ($field: ident, $text: literal) => {
        $field.attrs = $field.attrs.clone().into_iter().filter(|attr| {
            match &attr.meta {
                syn::Meta::Path(path) => !path.is_ident($text),
                syn::Meta::List(list) => !list.path.is_ident($text),
                _ => true,
            }
        }).collect();
    };
}

pub struct MatrixLikeField {
    pub(crate) ident: Ident,
    pub(crate) set_ident: Option<Ident>,
    pub(crate) get_ident: Option<Ident>,

    ///
    /// Not supported for now. Preserved for future use.
    ///
    pub(crate) new_ident: Option<Ident>
}

pub struct MatrixTagArg {
    ident: Ident,
    eq_tok: Token![=],
    expr: Ident
}

impl Parse for MatrixTagArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let eq_tok = input.parse()?;
        let expr = input.parse()?;

        Ok(Self { ident, eq_tok, expr })
    }
}

pub struct MatrixTag {
    args: Vec<MatrixTagArg>,
}

impl Parse for MatrixTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<MatrixTagArg> = Vec::new();

        while !input.is_empty() {
            let param = input.parse::<MatrixTagArg>();

            if let Ok(param) = param {
                params.push(param);
            }

            if let Err(_) = input.parse::<Token![,]>() {
                break;
            }
        }

        return Ok(Self {
            args: params
        })
    }
}

pub struct StructInfo {
    pub(crate) stt: ItemStruct,
    pub(crate) target_fields: Vec<Ident>,
    pub(crate) skipped_fields: Vec<Ident>,
    pub(crate) mat_like_fields: Vec<MatrixLikeField>
}

pub fn transform_struct(mut target: ItemStruct) -> anyhow::Result<StructInfo> {
    let input_fields = if let Fields::Named(named) = &mut target.fields {
        named
    } else {
        return Err(anyhow!("Only full struct is supported!"))
    };

    let mut regular_fields: Vec<Ident> = Vec::new();
    let mut skipped_fields: Vec<Ident> = Vec::new();
    let mut mat_like_fields: Vec<MatrixLikeField> = Vec::new();

    for field in &mut input_fields.named {
        if field_has_attr!(PureTag, field, "skip") {

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
        } else if field_has_attr!(WithParam, field, "matrix") {
            let attr = field.attrs.iter().find(|x| {
                if let syn::Meta::List(list) = &x.meta {
                    return list.path.is_ident("matrix");
                }
                return false
            }).unwrap();

            let matrix_attr = deduce_enum!(&attr.meta, syn::Meta::List);
            let args = syn::parse::<MatrixTag>(matrix_attr.tokens.clone().into())?;

            let mut get_ident = Option::None;
            let mut set_ident = Option::None;
            let mut new_ident = Option::None;

            args.args.iter().for_each(|it| {
                match it.ident.to_string().as_str() {
                    "get" => get_ident = Some(it.expr.clone()),
                    "set" => set_ident = Some(it.expr.clone()),
                    "new" => new_ident = Some(it.expr.clone()),

                    _ => { }
                }
            });

            mat_like_fields.push(MatrixLikeField {
                ident: field.ident.clone().unwrap(),
                set_ident,
                get_ident,
                new_ident
            })
        } else {
            regular_fields.push(field.ident.clone().unwrap());
        }
    }

    return Ok(StructInfo {
        stt: target,
        target_fields: regular_fields,
        skipped_fields,
        mat_like_fields,
    })
}