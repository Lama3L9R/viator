use mlua::{IntoLua, Lua, Value};

mod lresolver;

pub enum MaybeLua <T> {
    Lua(Value),
    Rs(T)
}

impl <T> MaybeLua<T> {
    pub fn unwrap_lua(self) -> Value {
        if let MaybeLua::Lua(x) = self {
            return x;
        }

        unreachable!();
    }

    pub fn unwrap_rs(self) -> T {
        if let MaybeLua::Rs(x) = self {
            return x;
        }
        unreachable!();
    }
}

impl <T: IntoLua> MaybeLua<T> {
    pub fn unwrap_or_into_lua(self, lua: &Lua) -> Value {
        match self {
            MaybeLua::Rs(x) => x.into_lua(lua).unwrap(),
            MaybeLua::Lua(x) => x.into_lua(lua).unwrap(),
        }
    }
}