pub fn search<'a>(query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    content.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();

    content
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "rust";
        let content = "\
rust
Rust
brust
Safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["rust", "brust"],
            search(query, content).collect::<Vec<_>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let content = "\
rust
Rust
brust
Safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["rust", "Rust", "brust"],
            search_case_insensitive(query, content).collect::<Vec<_>>()
        );
    }
}
