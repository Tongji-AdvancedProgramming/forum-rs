use std::vec;

use markup5ever::interface::TreeSink;
use once_cell::sync::Lazy;
use scraper::{Html, Selector};

pub struct HtmlCleaner;

static SELECTORS_TO_REMOVE: Lazy<Vec<Selector>> = Lazy::new(|| {
    vec![
        "script", "style", "noscript", "svg", "img", "picture", "video", "audio", "iframe",
        "canvas", "map", "object", "pre", "code",
    ]
    .into_iter()
    .map(|tag| Selector::parse(tag).unwrap())
    .collect()
});

impl HtmlCleaner {
    pub fn html_to_text(html: &str) -> String {
        let mut document = Html::parse_document(html);
        for selector in SELECTORS_TO_REMOVE.iter() {
            for node in document.select(selector) {
                document.remove_from_parent(&node);
            }
        }

        document.root_element().text().collect::<Vec<_>>().join(" ")
    }
}
