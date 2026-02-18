
#[macro_export]
macro_rules! functional_struct {
    (pub $name:ident) => {
        pub struct $name;

        impl mlua::IntoLua for $name {
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                return Ok(mlua::Value::Table(lua.create_table()?));
            }
        }

        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                return Ok(Self {});
            }
        }
    };

    ($name:ident) => {
        struct $name;

        impl mlua::IntoLua for $name {
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                return Ok(mlua::Value::Table(lua.create_table()?));
            }
        }

        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                return Ok(Self {});
            }
        }
    };

}

#[macro_export]
macro_rules! empty_intolua {
    ($name:ident) => {
        impl mlua::IntoLua for $name {
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                return Ok(mlua::Value::Table(lua.create_table()?));
            }
        }
    };
}

#[macro_export]
macro_rules! empty_fromlua {
    ($name:ident) => {
        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                return Ok(Self {});
            }
        }
    };
}

#[macro_export]
macro_rules! empty_autolua {
    ($name:ident, Into) => {
        viator_utils::empty_intolua!($name);
    };

    ($name:ident, From) => {
        viator_utils::empty_fromlua!($name);
    };

    ($name:ident, From, Into) => {
        viator_utils::empty_fromlua!($name);
        viator_utils::empty_intolua!($name);
    };

    ($name:ident, Into, From) => {
        viator_utils::empty_fromlua!($name);
        viator_utils::empty_intolua!($name);
    }
}
