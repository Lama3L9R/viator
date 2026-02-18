use autolua::autolua;
use mlua::Function;

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