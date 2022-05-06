pub fn parse_ints(ints: Vec<String>) -> Vec<i32> {
    ints.into_iter().filter_map(|i| i.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_it_comply() {
        let ints = parse_ints(
            vec!["1", "2", "a", "3"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        println!("{:?}", ints);
        assert_eq!(&ints[..], [1, 2, 3]);
    }
}
