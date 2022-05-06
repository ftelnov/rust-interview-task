pub struct StringWrapper(String);

impl From<&str> for StringWrapper {
    fn from(raw: &str) -> Self {
        Self(raw.to_string())
    }
}

impl AsRef<str> for StringWrapper {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_it_compile() {
        assert_eq!(StringWrapper::from("string").as_ref(), "string");
    }
}
