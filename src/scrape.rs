use std::fs::File;
use std::io::Read;

use mlua::{Lua, Integer};

use scraper::Html;

use scraper::Selector;
use scraper::ElementRef;

use mlua::UserData;

pub struct ParsedFragment {
    pub text: String,
    pub html: String,
}

impl ParsedFragment {
    pub fn new(element: ElementRef) -> Self {
        Self {
            text: element.text().collect::<String>(),
            html: element.html()
        }
    }

    pub fn select(&self, selector: String) -> Vec<ParsedFragment> {
        let frag = Html::parse_fragment(&self.html);
        let selector = Selector::parse(&selector).unwrap();
        let select = frag.select(&selector);
        let mut res = Vec::new();

        for element in select {
            res.push(Self::new(element));
        }
        res
    }
}

impl UserData for ParsedFragment {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("text", |_, this| Ok(this.text.clone()))
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("select", |_, this, selector: String| {
            Ok(this.select(selector))
        });
    }
}

pub struct Entry {
    text_map: Vec<(String, String)>,
}

impl Entry {
    pub fn from_map(text_map: Vec<(String, String)>) -> Self {
        Self {
            text_map
        }
    }

    pub fn main_text(&self) -> &str {
        &self.text_map.iter().find(|(k, _)| k == "main_text").unwrap().1
    }
}

fn get_html(_: &Lua, url: std::string::String) -> mlua::Result<ParsedFragment> {
    let html = reqwest::blocking::get(url).unwrap().text().unwrap().to_string();

    let parsed_fragment = Html::parse_document(&html);

    Ok(ParsedFragment::new(parsed_fragment.root_element()))
}

pub fn prepare_lua(lua: &mut Lua, script: &str) {
    let f = lua.create_function(get_html).unwrap();

    lua.globals().set("Get_HTML", f).unwrap();

    let mut program = String::new();
    File::open(script).unwrap().read_to_string(&mut program).unwrap();
    
    lua.load(&program).exec().unwrap();
}


/// search for a term using the provided lua script and return a vector of entries each consisting
/// of a key value pair
pub fn search_with_term(script: &str, term: &str) -> Vec<Entry> {
    // call lua code that gets url

    let mut lua = Lua::new();
    prepare_lua(&mut lua, script);
    
    let search: mlua::Function = lua.globals().get("Search").unwrap();
    let display: mlua::Function = lua.globals().get("Display").unwrap();

    let quote: mlua::Table = search.call(term).unwrap();

    let mut res = Vec::new();

    for pair in quote.pairs::<Integer, mlua::Table>() {
        let mut entry = Vec::new();

        let (_key, value) = pair.unwrap();
        
        let display_map: mlua::Table = display.call(value).unwrap();


        for pair in display_map.pairs::<String, String>() {
            let (key, value) = pair.unwrap();
            entry.push((key, value));
        }
        res.push(Entry::from_map(entry));
    }

    res
}
