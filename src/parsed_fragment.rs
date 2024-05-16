use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;

enum Selectable<'a> {
    Doc(Html),
    Ref(ElementRef<'a>)
} 

/// this struct serves as an interface for lua code to call selector and text queries on this
/// element
pub struct ParsedFragment<'a> {
    fragment: Selectable<'a>,
}

impl<'a> ParsedFragment<'a> {
    pub fn from_doc(html: Html) -> Self {
        Self {
            fragment: Selectable::Doc(html)
        }
    }

    pub fn from_frag(element: ElementRef<'a>) -> Self {
        Self {
            fragment: Selectable::Ref(element)
        }
    }

    pub fn select(&'a self, selector: Selector) -> Vec<ParsedFragment> {
        match &self.fragment {
            Selectable::Doc(html) => {
                let select = html.select(&selector);
                let mut res = Vec::new();

                for element in select {
                    res.push(Self::from_frag(element))
                }
                res
            },
            Selectable::Ref(element) => {
                let select = element.select(&selector);
                let mut res = Vec::new();

                for element in select {
                    res.push(Self::from_frag(element))
                }
                res
            },
        }
    }

    pub fn text(&'a self) -> String {
        match &self.fragment {
            Selectable::Doc(html) => html.html(),
            Selectable::Ref(element) => element.text().collect::<Vec<_>>().join(""),
        }
    }
}
