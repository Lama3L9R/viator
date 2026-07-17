use autolua::autolua;
use mlua::Function;
use crate::build::lua::target::Target;

pub mod target;
pub mod action;
pub mod dep;
pub mod frontmatter;

#[autolua(Into, From)]
pub struct ViatorFileLua {
    pub namespace: String,
    pub name: String,
    pub version: String,
    pub envCheck: Option<Vec<Function>>,
    pub targets: Vec<Target>,
}
