use autolua::autolua;
use mlua::{FromLua, Function, IntoLua, Lua};
use crate::build::{BuildContext, StateHandle, ViatorState};
use crate::build::lua::action::ConfigBoundAction;
use crate::build::lua::dep::DependencyDesc;

#[autolua(Into, From)]
pub struct SanitizerSettings {
    pub address: Option<bool>,
}

#[autolua(Into, From)]
pub struct Target {
    pub name: String,
    pub dependencies: Vec<DependencyDesc>,
    pub extraFlagsHandler: Function,
    pub pipeline: Vec<ConfigBoundAction>,

    pub debug: Option<bool>,
    pub optimization: Option<u8>,
    pub sanitizers: Option<SanitizerSettings>
}

impl Target {
    pub fn run(&self, state: &ViatorState, ctx: BuildContext) -> anyhow::Result<BuildContext> {
        let val = ctx.into_lua(state.lua())?;

        for act in &self.pipeline {
            act.invoke(&val)?;
        }

        Ok(BuildContext::from_lua(val, state.lua())?)
    }
}