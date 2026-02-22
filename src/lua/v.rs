
use mlua::{Error, FromLua, IntoLua, MultiValue, ObjectLike, UserDataFields, UserDataMethods};
use autolua::{autolua, bindlua};
use mlua::Lua;
use crate::build::StateHandle;

bindlua! {
    #[autolua(From)]
    pub lua V2D {
        lua x: u32
        lua y: u32

        lua operator fn add(v1: V2D) -> V2D {
            return Ok(V2D {
                x: this.x + v1.x,
                y: this.y + v1.y,
            })
        }

        lua static operator fn sub(v1: V2D, v2: V2D) -> V2D {
            return Ok(V2D {
                x: v1.x - v2.x,
                y: v1.y - v2.y,
            })
        }

        lua operator fn toString() -> String {
            return Ok(format!("Vector2D [{}, {}]", this.x, this.y))
        }
    }
}

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

        lua static fn newVector2D(x: u32, y: u32) -> V2D {
            return Ok(V2D { x, y })
        }

        lua static fn require(pkg: String) -> String {
            println!("Alright");

            return Ok(String::from("hello"));
        }

        lua static fn depend(test: String) -> String {
            println!("depend: {}", test);

            return Ok(format!("こんばんは {}", test));
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
