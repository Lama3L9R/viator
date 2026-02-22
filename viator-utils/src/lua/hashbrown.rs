#![cfg(feature = "hashbrown")]

use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use anyhow::anyhow;
use mlua::{FromLua, IntoLua, Lua};
use mlua::prelude::{LuaResult, LuaValue};
use crate::deduce_enum;

#[derive(Debug, Clone)]
pub struct HashbrownMap<K, V>(hashbrown::HashMap<K, V>)
    where K: Eq + Hash;

impl <K: Eq + Hash, V> HashbrownMap<K, V> {
    pub fn new() -> HashbrownMap<K, V> {
        HashbrownMap(hashbrown::HashMap::new())
    }
}

impl <K: Eq + Hash, V> Deref for HashbrownMap<K, V> {
    type Target = hashbrown::HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <K: Eq + Hash, V> DerefMut for HashbrownMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <K: Eq + Hash, V> Into<hashbrown::HashMap<K, V>> for HashbrownMap<K, V> {
    fn into(self) -> hashbrown::HashMap<K, V> {
        return self.0;
    }
}

impl <K: Eq + Hash, V> From<hashbrown::HashMap<K, V>> for HashbrownMap<K, V> {
    fn from(hashmap: hashbrown::HashMap<K, V>) -> HashbrownMap<K, V> {
        return HashbrownMap(hashmap);
    }
}

impl <K: FromLua + Eq + Hash, V: FromLua> FromLua for HashbrownMap<K, V> {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        if matches!(&value, LuaValue::Table(_)) {
            return Err(anyhow!("Lua type mismatch! Expected Table to convert to HashbrownMap!").into())
        }

        let mut hash: hashbrown::HashMap<K, V> = hashbrown::HashMap::new();
        let value = deduce_enum!(&value, LuaValue::Table);

        value.for_each(|key, value| {
            hash.insert(key, value);

            Ok(())
        })?;


        return Ok(Self(hash));
    }
}

impl <K: IntoLua + Eq + Hash + Clone, V: IntoLua + Clone> IntoLua for HashbrownMap<K, V> {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let tbl = lua.create_table()?;

        self.iter().for_each(|(k, v)| {
            tbl.set(k.clone(), v.clone()).unwrap();
        });

        Ok(LuaValue::Table(tbl))
    }
}

impl <K: Eq + Hash, V: Eq + Hash> Default for HashbrownMap<K, V> {
    fn default() -> Self {
        HashbrownMap::new()
    }
}