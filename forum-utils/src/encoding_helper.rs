use encoding_rs::GBK;
use std::borrow::Cow;

pub struct EncodingHelper;

impl EncodingHelper {
    pub fn gbk_guard(text: &str) -> bool {
        // 将字符串编码为GBK
        let (cow, _encoding_used, had_errors) = GBK.encode(text);

        // 如果在编码过程中遇到错误，则认为存在不兼容的字符
        if had_errors {
            return false;
        }

        // 解码回字符串，比较是否与原始字符串相同
        let decoded = GBK.decode(&cow).0;

        // 如果解码后的字符串与原始字符串不同，也认为存在不兼容的字符
        decoded == Cow::Borrowed(text)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gbk_guard() {
        assert_eq!(EncodingHelper::gbk_guard("你好"), true);
        assert_eq!(EncodingHelper::gbk_guard("你好👋"), false);
    }
}
