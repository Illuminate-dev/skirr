mod parsed_fragment;
pub mod gui;

use std::fs::File;
use std::io::Read;

use mlua::{Lua, Integer};

use parsed_fragment::ParsedFragment;
use scraper::Html;

const NAME: &str = "Skirr";

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
    File::open("scripts/quotes.lua").unwrap().read_to_string(&mut program).unwrap();
    
    lua.load(&program).exec().unwrap();

    let search: mlua::Function = lua.globals().get("Search").unwrap();
    let display: mlua::Function = lua.globals().get("Display").unwrap();

    let quote: mlua::Table = search.call(term).unwrap();

    for pair in quote.pairs::<Integer, mlua::Table>() {
        let (_key, value) = pair.unwrap();
        
        let display_map: mlua::Table = display.call(value).unwrap();

        for pair in display_map.pairs::<String, String>() {
            let (key, value) = pair.unwrap();
            println!("{}: {}", key, value);
        }
    }
    
    // let html: mlua::Integer = search.call(term).unwrap();
    // println!("{}", html);

}
