// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_blank() {
        let parser = Parser::new();

        match parser.parse(r#""#) {
            None => panic!(r#"invalid parse. """#),
            Some(result) => {
                assert_eq!(result.category, "UNKNOWN");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#""#) {
            None => panic!(r#"invalid parse. """#),
            Some(result) => {
                assert_eq!(result.category, "UNKNOWN");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"-"#) {
            None => panic!(r#"invalid parse. "-""#),
            Some(result) => {
                assert_eq!(result.category, "UNKNOWN");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
    }
}
