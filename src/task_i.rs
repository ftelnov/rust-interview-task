pub struct StringWrapper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_it_compile() {
        assert_eq!(StringWrapper::from("string").as_ref(), "string");
    }
}
