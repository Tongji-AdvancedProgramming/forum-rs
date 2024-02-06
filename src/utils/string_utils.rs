pub trait StringUtilsExt {
    fn is_numberic(&self) -> bool;
}

impl<'a> StringUtilsExt for &'a str {
    fn is_numberic(&self) -> bool {
        self.chars().all(|ch| ch.is_numeric())
    }
}

impl StringUtilsExt for String {
    fn is_numberic(&self) -> bool {
        self.as_str().is_numberic()
    }
}
