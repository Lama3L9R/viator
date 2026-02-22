use std::fs::read_to_string;
use std::path::PathBuf;
use anyhow::anyhow;
use autolua::autolua;
use mlua::{FromLua, Lua, Value};
use viator_utils::lua::hashbrown::HashbrownMap;
use crate::build::lua::ViatorFile;
use crate::build::registry::Registry;
use crate::CliArgs;
use crate::lua::v::V;
use crate::utils::RBox;
use crate::build::lua::action::Action;

pub mod registry;
pub mod lua;

#[autolua(Into, From)]
pub struct BuildContext {
    pub files: HashbrownMap<String, Value>,

}

impl BuildContext {
    fn new() -> BuildContext {
        BuildContext {
            files: HashbrownMap::new(),
        }
    }
}

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
    pub fn load_script(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let result = ViatorFile::from_lua(self.exec_lua(path)?, &self.lua)?;

        self.viator_file = Some(RBox::new(result));

        return Ok(());
    }

    ///
    /// Run a pipeline described by the given ViatorFile
    ///
    pub fn execute_pipeline(&self, name: String) -> anyhow::Result<BuildContext> {
        let context = BuildContext::new();

        if self.viator_file.is_none() {
            return Err(anyhow!("Viator file not yet loaded"))
        }

        let target = self.viator_file.as_ref().unwrap().targets.iter().find(|it| {
            it.name == name
        });

        if let None = target {
            return Err(anyhow!("Target not found"))
        }

        let final_ctx = target.unwrap().run(self, context)?;

        return Ok(final_ctx);
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

    ///
    /// Get lua as a ref of vm handle
    ///
    pub fn lua(&self) -> &Lua { &self.lua }
}