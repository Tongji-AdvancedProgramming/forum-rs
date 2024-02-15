use std::vec;

use once_cell::sync::Lazy;
use regex::Regex;
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

static BLANK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s{2,}").unwrap());

impl HtmlCleaner {
    pub fn html_to_text(html: &str) -> String {
        let mut document = Html::parse_document(html);
        let ids_to_remove = SELECTORS_TO_REMOVE
            .iter()
            .flat_map(|s| document.select(s).map(|node| node.id()))
            .collect::<Vec<_>>();

        for id in ids_to_remove {
            document.tree.get_mut(id).unwrap().detach();
        }

        let result = document.root_element().text().collect::<Vec<_>>().join(" ");
        BLANK_REGEX.replace_all(&result, " ").into_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_html_to_text() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <h1>Test</h1>
                    <p>Test</p>
                    <script>alert('test');</script>
                    <style>body { color: red; }</style>
                    <noscript><p>Test</p></noscript>
                    <svg><text>Test</text></svg>
                    <img src="test.png" alt="Test">
                    <picture><source srcset="test.png" type="image/png"></picture>
                    <video><source src="test.mp4" type="video/mp4"></video>
                    <audio><source src="test.mp3" type="audio/mpeg"></audio>
                    <iframe src="test.html"></iframe>
                    <canvas></canvas>
                    <map></map>
                    <object></object>
                    <pre>Test</pre>
                    <code>Test</code>
                </body>
            </html>
        "#;

        let text = HtmlCleaner::html_to_text(html);
        println!("{}", text);
    }
}
