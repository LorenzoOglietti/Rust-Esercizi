#[cfg(test)]
mod tests {
    use Esercizio1::anagrammi::anagrammi::sono_anagrammi;

    #[test]
    fn test_sono_anagrammi() {
        assert_eq!(sono_anagrammi("listen", "silent"), true);
        assert_eq!(sono_anagrammi("test", "sert"), false);
        assert_eq!(sono_anagrammi("atto", "otTa"), true);

        assert_eq!(sono_anagrammi("", ""), true);
        assert_eq!(sono_anagrammi("a", "a"), true);

        assert_eq!(sono_anagrammi("abcdefg", "gfedcba"), true);

        assert_eq!(sono_anagrammi("abc", "abcd"), false);
        assert_eq!(sono_anagrammi("abcd", "abc"), false);

        assert_eq!(sono_anagrammi("ABCD", "abcd"), true)
    }
}