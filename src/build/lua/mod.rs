use autolua::autolua;
use mlua::Function;
use crate::build::lua::target::Target;

pub mod target;
pub mod action;
pub mod dep;

#[autolua(Into, From)]
pub struct ViatorFile {
    ///
    /// Name of the project
    ///
    pub name: String,
    pub envCheck: Vec<Function>,
    pub targets: Vec<Target>,
}
