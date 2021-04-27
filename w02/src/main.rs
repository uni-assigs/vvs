pub fn is_valid(identifier: &str) -> bool {
    // Make sure the identifier has at least a character and at most 6 
    match identifier.len() {
        1..=6 => {}
        _ => return false,
    }

    // The first character has to be a letter
    let first_char = identifier.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        return false;
    }

    // Make sure the rest of the identifier is alphanumeric
    identifier
        .chars()
        .map(char::is_alphanumeric)
        .all(|b| b == true)
}

#[cfg(test)]
mod silly_pascal_tests {
    use super::is_valid;

    #[test]
    fn valid_identifier_lengths() {
        let identifiers = vec!["i", "ij", "abc", "resu", "abcde", "monads"];

        let is_valid: Vec<bool> = identifiers.into_iter().map(is_valid).collect();

        assert_eq!(is_valid.iter().all(|b| *b == true), true)
    }

    #[test]
    fn invalid_identifier_lengths() {
        let identifiers = vec!["", "results", "mysql_query", "abcdefg", "parametric_polymorphism"];

        let is_valid: Vec<bool> = identifiers.into_iter().map(is_valid).collect();

        assert_eq!(is_valid.iter().all(|b| *b == false), true)
    }

    #[test]
    fn invalid_identifier_first_char() {
        let identifiers = vec!["2abc", "3res", "4nbc"];

        let is_valid: Vec<bool> = identifiers.into_iter().map(is_valid).collect();

        assert_eq!(is_valid.iter().all(|b| *b == false), true)
    }

    #[test]
    fn valid_alphanumeric_identifiers() {
        let identifiers = vec!["joão2", "res34", "mbcd45"];

        let is_valid: Vec<bool> = identifiers.into_iter().map(is_valid).collect();

        assert_eq!(is_valid.iter().all(|b| *b == true), true)
    }

    #[test]
    fn alphanumeric_identifiers_too_long() {
        let identifiers = vec![
            "funcionário2", "produto3", "empresa4"
        ];

        let is_valid: Vec<bool> = identifiers.into_iter().map(is_valid).collect();

        assert_eq!(is_valid.iter().all(|b| *b == false), true)
    }

    #[test]
    fn invalid_alphanumeric_identifiers() {
        let identifiers = vec![
            "func1@", "ab.c3", "emp5!a", "abc2>", "abc4*"
        ];

        let is_valid: Vec<bool> = identifiers.into_iter().map(is_valid).collect();

        assert_eq!(is_valid.iter().all(|b| *b == false), true)
    }
}

fn main() {
    dbg!(is_valid("identifier"));
    dbg!(is_valid("abc2"));
}
