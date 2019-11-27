// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_pc_lowpriority() {
        let parser = Parser::new();

        match parser.parse(r#"Sleipnir/2.9.9"#) {
            None => panic!(r#"invalid parse. "Sleipnir/2.9.9""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Sleipnir");
                assert_eq!(result.os, "Windows UNKNOWN Ver");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "2.9.9");
            }
        }
    }
}
