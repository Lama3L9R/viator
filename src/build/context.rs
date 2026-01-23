use std::any::Any;

use hashbrown::{HashMap, HashSet};
use mlua::serde::Serializer;
use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildContext {
    ///
    /// Contextual files, where key is file type, and value is files
    ///
    pub files: HashMap<String, HashSet<String>>,
}

impl FromLua for BuildContext {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

impl IntoLua for BuildContext {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        lua.to_value(&self)
    }
}
