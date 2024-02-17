use std::borrow::Cow;
use encoding_rs::GBK;

pub struct EncodingHelper;

impl EncodingHelper {
    pub fn utf2gbk(text: &str) -> Cow<str> {
        GBK.decode(text.as_bytes()).0
    }
}
