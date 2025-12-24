use mlua::{Lua, Result as LuaResult};
use std::fs;
use std::process::Command;

use crate::metrics::Metrics;

pub struct LuaEngine {
    lua: Lua,
}

impl LuaEngine {
    pub fn new() -> LuaResult<Self> {
        let lua = Lua::new();

        lua.globals().set(
            "notify",
            lua.create_function(|_, msg: String| {
                println!("⚠ ALERT: {}", msg);
                Ok(())
            })?,
        )?;

        lua.globals().set(
            "run",
            lua.create_function(|_, cmd: String| {
                let _ = Command::new("sh").arg("-c").arg(cmd).spawn();
                Ok(())
            })?,
        )?;

        Ok(Self { lua })
    }

    pub fn execute(&self, metrics: &Metrics) -> LuaResult<()> {
        let globals = self.lua.globals();

        let cpu = self.lua.create_table()?;
        cpu.set("usage", metrics.cpu)?;
        globals.set("cpu", cpu)?;

        let mem = self.lua.create_table()?;
        mem.set("used", metrics.memory_used)?;
        mem.set("total", metrics.memory_total)?;
        mem.set(
            "used_percent",
            (metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0,
        )?;
        globals.set("mem", mem)?;

        for script in load_lua_rules("lua/rules") {
            let _ = self.lua.load(&script).exec();
        }

        Ok(())
    }
}

fn load_lua_rules(dir: &str) -> Vec<String> {
    let mut scripts = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                if let Ok(content) = fs::read_to_string(&path) {
                    scripts.push(content);
                }
            }
        }
    }

    scripts
}
