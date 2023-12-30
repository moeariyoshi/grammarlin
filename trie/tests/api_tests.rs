#[cfg(test)]
mod test {
    use trie::Trie;

    //tests should only use api calls, as all other internals are private
    //if we want to test private methods, tests must be written within lib.rs itself

    #[test]
    fn basic_prefix_search() {
        let mut wordlist: Vec<String> = Vec::new();
        wordlist.push("aa".to_owned());
        wordlist.push("aa".to_owned());
        wordlist.push("aa".to_owned());

        wordlist.push("ac".to_owned());
        wordlist.push("ac".to_owned());
        wordlist.push("ac".to_owned());

        wordlist.push("aaa".to_owned());
        wordlist.push("aaa".to_owned());

        wordlist.push("ab".to_owned());

        wordlist.push("ae".to_owned());

        let mytrie = Trie::new(wordlist);
        assert!(
            vec![
                "a".to_owned(),
                "c".to_owned(),
                "aa".to_owned(),
                "b".to_owned(),
                "e".to_owned(),
            ] == mytrie.get_suggestions("a".to_owned(), 5)
                || vec![
                    "a".to_owned(),
                    "c".to_owned(),
                    "aa".to_owned(),
                    "e".to_owned(),
                    "b".to_owned(),
                ] == mytrie.get_suggestions("a".to_owned(), 5)
        );
        assert!(mytrie.get_suggestions("b".to_owned(), 5).is_empty());
        assert_eq!(mytrie.get_suggestions("ac".to_owned(), 10), vec!["".to_owned()]);
        assert!(mytrie.get_suggestions("a".to_owned(), 0).is_empty());
        assert!(
            vec![
                "aa".to_owned(),
                "ac".to_owned(),
                "aaa".to_owned(),
                "ab".to_owned(),
                "ae".to_owned(),
            ] == mytrie.get_suggestions("".to_owned(), 5)
                || vec![
                    "aa".to_owned(),
                    "ac".to_owned(),
                    "aaa".to_owned(),
                    "ae".to_owned(),
                    "ab".to_owned(),
                ] == mytrie.get_suggestions("".to_owned(), 5)
        );
    }

    #[test]
    fn uppercase_prefix_search() {
        let mut wordlist: Vec<String> = Vec::new();
        wordlist.push("aa".to_owned());
        wordlist.push("aa".to_owned());
        wordlist.push("aa".to_owned());

        wordlist.push("ac".to_owned());
        wordlist.push("ac".to_owned());
        wordlist.push("ac".to_owned());

        wordlist.push("aaa".to_owned());
        wordlist.push("aaa".to_owned());

        wordlist.push("ab".to_owned());

        wordlist.push("ae".to_owned());

        let mytrie = Trie::new(wordlist);

        assert!(
            vec![
                "a".to_owned(),
                "c".to_owned(),
                "aa".to_owned(),
                "b".to_owned(),
                "e".to_owned(),
            ] == mytrie.get_suggestions("A".to_owned(), 5)
                || vec![
                    "a".to_owned(),
                    "c".to_owned(),
                    "aa".to_owned(),
                    "e".to_owned(),
                    "b".to_owned(),
                ] == mytrie.get_suggestions("A".to_owned(), 5)
        );
    }
}
