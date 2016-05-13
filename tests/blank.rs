// This file is auto-generated! Any changes to this file will be lost!
extern crate woothee;

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_blank() {
        let parser = Parser::new();

        match parser.parse(r#""#) {
            None => panic!(r#"invalid parse. """#),
            Some(result) => {
                assert_eq!(result.category, "UNKNOWN".to_string());
                assert_eq!(result.name, "UNKNOWN".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#""#) {
            None => panic!(r#"invalid parse. """#),
            Some(result) => {
                assert_eq!(result.category, "UNKNOWN".to_string());
                assert_eq!(result.name, "UNKNOWN".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"-"#) {
            None => panic!(r#"invalid parse. "-""#),
            Some(result) => {
                assert_eq!(result.category, "UNKNOWN".to_string());
                assert_eq!(result.name, "UNKNOWN".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
    }
}
