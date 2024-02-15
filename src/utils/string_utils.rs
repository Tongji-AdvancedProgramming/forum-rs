pub trait StringUtilsExt {
    fn is_numeric(&self) -> bool;

    fn abbreviate(&self, len: usize) -> String;
}

impl<'a> StringUtilsExt for &'a str {
    fn is_numeric(&self) -> bool {
        self.chars().all(|ch| ch.is_numeric())
    }

    fn abbreviate(&self, len: usize) -> String {
        if self.len() > len {
            String::from(&self[..len]) + "..."
        } else {
            String::from(*self)
        }
    }
}

impl StringUtilsExt for String {
    fn is_numeric(&self) -> bool {
        self.as_str().is_numeric()
    }

    fn abbreviate(&self, len: usize) -> String {
        self.as_str().abbreviate(len)
    }
}
