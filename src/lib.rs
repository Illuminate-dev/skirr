use std::fs::File;
use std::io::Read;

use mlua::Lua;


pub fn test_lua_function() {
    let lua = Lua::new();

    let mut program = String::new();
    File::open("main.lua").unwrap().read_to_string(&mut program).unwrap();

    lua.load(&program).exec().unwrap();

    let v: mlua::Function = lua.globals().get("Search").unwrap();
    let x: mlua::Table = v.call("test").unwrap();
    println!("{:?}", x);
}
