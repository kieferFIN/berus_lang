pub(crate) fn str_from_iter<I: Iterator<Item=T>, T: ToString>(i: I, sep: &str) -> String {
    i.map(|t| t.to_string()).reduce(|s1, s2| s1 + sep + &s2).unwrap_or("".to_string())
}