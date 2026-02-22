use autolua::autolua;
use mlua::{Function, IntoLuaMulti, Lua, MultiValue, Value};
use crate::build::{BuildContext, StateHandle};

#[autolua(From, Into)]
pub struct Action {
    pub name: String,
    pub handler: Function
}

#[autolua(From, Into)]
pub struct ConfigBoundAction {
    pub name: Option<String>,
    pub config: mlua::Value,
    pub act: Action
}

impl ConfigBoundAction {
    pub fn invoke(&self, ctx: &Value) -> anyhow::Result<()> {
        let mut multi = MultiValue::new();

        multi.push_back(ctx.clone());
        multi.push_back(self.config.clone());

        return Ok(self.act.handler.call::<()>(multi)?);
    }
}