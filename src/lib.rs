//! # minigrep
//! 
//! `minigrep` provides a minimal super-fast implementation of `grep` with iterators.

/// Search the query in the content.
/// 
/// # Example
/// 
/// ```
/// let query = "rust";
/// let content = "rust is good.
/// Rust is good.";
/// 
/// assert_eq!(vec!["rust is good."], minigrep::search(query, content).collect::<Vec<_>>());
/// ```
/// 
/// # Panics
/// 
/// These are the scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.
/// 
/// # Errors
/// 
/// If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so that they can write code to handle the different kinds of errors in different ways.
/// 
/// # Safety
/// 
/// If the function is unsafe to call (we discuss unsafety in Chapter 20), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.
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
