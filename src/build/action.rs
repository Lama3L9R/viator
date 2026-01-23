use crate::ViatorState;
use crate::build::context::BuildContext;
use anyhow::{Error, anyhow};
use hashbrown::HashMap;
use mlua::{Function, IntoLua, Lua, LuaNativeFn, MultiValue, Value};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

type NativeFunction =
    fn(viator: Box<ViatorState>, ctx: Rc<Box<BuildContext>>, val: mlua::Value) -> Result<(), Error>;

enum MaybeNativeFunction {
    Native(NativeFunction),
    Lua(Function),
}

impl MaybeNativeFunction {
    fn unwrap_lua(&self) -> Result<&Function, Error> {
        if let MaybeNativeFunction::Lua(f) = self {
            Ok(f)
        } else {
            Err(anyhow!("Unwrapped a native function into a lua function"))
        }
    }

    fn unwrap_native(&self) -> Result<&NativeFunction, Error> {
        if let MaybeNativeFunction::Native(f) = self {
            Ok(&f)
        } else {
            Err(anyhow!("Unwrapped a lua function into a native function"))
        }
    }
}

pub struct Action {
    pub name: String,
    pub handler: MaybeNativeFunction,
}

pub struct ConfigBoundAction {
    pub name: Option<String>,
    pub act: Arc<Action>,
    pub value: Value,
}

impl ConfigBoundAction {
    fn call(
        &self,
        viator: Box<ViatorState>,
        ctx: Box<BuildContext>,
        args: &Value,
    ) -> Result<(), Error> {
        let mut values = MultiValue::new();

        values.push_front(ctx.into_lua(&viator.lua)?);
        values.push_front(args.clone());

        self.act.handler.unwrap_lua()?.call::<()>(values)?;

        Ok(())
    }

    fn call_directly(
        &self,
        viator: Box<ViatorState>,
        ctx: Rc<Box<BuildContext>>,
        args: &Value,
    ) -> Result<(), Error> {
        self.act.handler.unwrap_native()?(viator, ctx, args.clone())
    }
}

impl Action {
    fn bound(self: Arc<Action>, config: Value) -> ConfigBoundAction {
        ConfigBoundAction {
            name: None,
            act: self,
            value: config,
        }
    }

    fn bound_with_name(self: Arc<Action>, name: String, config: Value) -> ConfigBoundAction {
        ConfigBoundAction {
            name: Some(name),
            act: self,
            value: config,
        }
    }
}
