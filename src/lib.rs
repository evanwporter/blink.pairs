use mlua::prelude::*;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, MutexGuard};

use buffer::ParsedBuffer;

pub mod buffer;

static PARSED_BUFFERS: LazyLock<Mutex<HashMap<usize, ParsedBuffer>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn buffers<'a>() -> MutexGuard<'a, HashMap<usize, ParsedBuffer>> {
    match PARSED_BUFFERS.lock() {
        Ok(lock) => lock,
        Err(_) => {
            PARSED_BUFFERS.clear_poison();
            let mut lock = PARSED_BUFFERS.lock().expect("reset poisoned parser cache");
            *lock = HashMap::new();
            lock
        }
    }
}

fn parse_buffer(_lua: &Lua, (bufnr, filetype, lines): (usize, String, Vec<String>)) -> LuaResult<bool> {
    if !ParsedBuffer::supports_filetype(&filetype) {
        buffers().remove(&bufnr);
        return Ok(false);
    }
    let lines = lines.iter().map(String::as_str).collect::<Vec<_>>();
    buffers().insert(bufnr, ParsedBuffer::parse(&lines));
    Ok(true)
}

fn get_match_at(
    _lua: &Lua,
    (bufnr, row, col): (usize, usize, usize),
) -> LuaResult<Option<buffer::KeywordMatch>> {
    Ok(buffers().get(&bufnr).and_then(|buffer| buffer.match_at(row, col)))
}

#[mlua::lua_module(skip_memory_check)]
fn sv_matchit(lua: &Lua) -> LuaResult<LuaTable> {
    std::panic::set_hook(Box::new(|_| {}));
    let exports = lua.create_table()?;
    exports.set("parse_buffer", lua.create_function(parse_buffer)?)?;
    exports.set("get_match_at", lua.create_function(get_match_at)?)?;
    Ok(exports)
}
