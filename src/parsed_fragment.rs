use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;

use mlua::UserData;


/// this struct serves as an interface for lua code to call selector and text queries on this
/// element
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

