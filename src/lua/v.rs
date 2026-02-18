
use mlua::IntoLua;
use autolua::bindlua;
use mlua::Lua;
use mlua::prelude::LuaResult;
use crate::build::StateHandle;

bindlua! {
    pub lua V {
        lua registry: VRegistry
        lua utils: VUtils

        pub fn attach(self, lua: &Lua) {
            lua.globals().set("V", self).expect("Failed to attach helper struct V to lua globals");
        }

        pub fn new(state: StateHandle) -> V {
            return V {
                registry: VRegistry::new(state.clone()),
                utils: VUtils::new(state),
            }
        }

        lua fn require() -> LuaResult<String> {
            println!("Alright");

            return Ok(String::from("hello"));
        }

        lua fn depend() {

        }

        lua fn dependDyn() {

        }

        lua fn dependStatic() {

        }

        lua fn action() {

        }

        lua fn linker() {

        }

        lua fn compiler() {

        }

        lua fn info() {

        }

        lua fn warning() {

        }

        lua fn error() {

        }
    }

    #[derive(Clone)]
    pub lua VRegistry {
        pub fn new(state: StateHandle) -> VRegistry {
            return VRegistry {

            }
        }
    }

    #[derive(Clone)]
    pub lua VUtils {
        pub fn new(state: StateHandle) -> VUtils {
            return VUtils {

            }
        }
    }
}