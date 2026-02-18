use std::fs::read_to_string;
use std::path::PathBuf;
use mlua::{Lua, Value};
use crate::build::lua::ViatorFile;
use crate::build::registry::Registry;
use crate::CliArgs;
use crate::lua::v::V;
use crate::utils::RBox;
use crate::build::lua::action::Action;

pub mod registry;
pub mod lua;

pub struct ViatorState {
    pub cli_args: CliArgs,
    pub lua: Lua,
    pub acts: Registry<Action>,
    pub viator_file: Option<RBox<ViatorFile>>,
}

pub type StateHandle = RBox<ViatorState>;
impl ViatorState {
    pub fn create(cli_args: CliArgs) -> anyhow::Result<StateHandle> {
        let state = RBox::new(ViatorState {
            cli_args,
            lua: Lua::new(),
            acts: Registry::new(),
            viator_file: None
        });

        V::new(state.clone()).attach(&state.lua);

        return Ok(state)
    }

    pub fn exec_lua(&self, file: PathBuf) -> anyhow::Result<Value> {
        let lua = read_to_string(file)?;

        Ok(self.exec_lua_code(lua)?)
    }

    pub fn exec_lua_code(&self, code: String) -> anyhow::Result<Value> {
        Ok(self.lua.load(code).eval::<Value>()?)

    }

    ///
    /// Loads a plugin Viator file, where the return value of the file is omitted
    ///
    pub fn load_plugin(&self, path: PathBuf) {

    }

    ///
    /// Loads a Viator file, where the return value will be parsed into ViatorFile
    ///
    pub fn load_script(&self, path: PathBuf) {

    }

    ///
    /// Run a pipeline described by the given ViatorFile
    ///
    pub fn execute_pipeline() {

    }

    ///
    /// Return the lua backend info
    ///
    pub fn get_lua_version(&self) -> String {
        let ver = self.lua.globals().get::<mlua::String>("_VERSION")
            .unwrap()
            .to_str().unwrap()
            .to_string();

        let jit = self.lua.globals().get::<mlua::Table>("jit");

        return if jit.is_ok() {
            let jit_ver = jit.unwrap().get::<mlua::String>("version").unwrap()
                .to_str().unwrap()
                .to_string();

            format!("{} ({})", ver, jit_ver)
        } else {
            ver
        }
    }
}