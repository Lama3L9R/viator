use autolua::autolua;
use mlua::Table;

#[autolua(From, Into)]
pub struct DependencyDesc {
    pub name: String,
    pub version: Table,
    pub flags: Table
}