mod parsed_fragment;

use std::fs::File;
use std::io::Read;

use mlua::Lua;

use parsed_fragment::ParsedFragment;
use scraper::{Html, Selector};

fn get_html(_: &Lua, url: std::string::String) -> mlua::Result<ParsedFragment> {
    let html = reqwest::blocking::get(url).unwrap().text().unwrap().to_string();

    let parsed_fragment = Html::parse_document(&html);

    Ok(ParsedFragment::new(parsed_fragment.root_element()))
}

pub fn search_with_term(term: &str) {
    // call lua code that gets url
    
    let lua = Lua::new();

    let f = lua.create_function(get_html).unwrap();

    lua.globals().set("Get_HTML", f).unwrap();

    let mut program = String::new();
    File::open("main.lua").unwrap().read_to_string(&mut program).unwrap();
    
    lua.load(&program).exec().unwrap();

    let search: mlua::Function = lua.globals().get("Search").unwrap();
    let html: mlua::AnyUserData = search.call(term).unwrap();
    println!("{}", html.borrow::<ParsedFragment>().unwrap().text);
    // let html: mlua::Integer = search.call(term).unwrap();
    // println!("{}", html);

}
