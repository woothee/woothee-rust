// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_mobilephone_au() {
        let parser = Parser::new();

        match parser.parse(r#"KDDI-TS3V UP.Browser/6.2_7.2.7.1.K.6.210 (GUI) MMP/2.0"#) {
            None => panic!(r#"invalid parse. "KDDI-TS3V UP.Browser/6.2_7.2.7.1.K.6.210 (GUI) MMP/2.0""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "au by KDDI");
                assert_eq!(result.os, "au");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "TS3V");
            }
        }
        match parser.parse(r#"KDDI-CA3H UP.Browser/6.2_7.2.7.1.K.5.207 (GUI) MMP/2.0"#) {
            None => panic!(r#"invalid parse. "KDDI-CA3H UP.Browser/6.2_7.2.7.1.K.5.207 (GUI) MMP/2.0""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "au by KDDI");
                assert_eq!(result.os, "au");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "CA3H");
            }
        }
        match parser.parse(r#"Mozilla/5.0 Opera/9.5 (KDDI-SH3F; BREW; Opera Mobi; U; ja) Presto/2.2.1"#) {
            None => {
                panic!(r#"invalid parse. "Mozilla/5.0 Opera/9.5 (KDDI-SH3F; BREW; Opera Mobi; U; ja) Presto/2.2.1""#)
            }
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "au");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "SH3F");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 6.0; KDDI-CA3B) Opera 8.60 [ja]"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 6.0; KDDI-CA3B) Opera 8.60 [ja]""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "au");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "CA3B");
            }
        }
    }
}
