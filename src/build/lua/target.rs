use autolua::autolua;
use mlua::Function;
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