// This file is auto-generated! Any changes to this file will be lost!
extern crate woothee;

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_mobilephone_docomo() {
        let parser = Parser::new();

        match parser.parse(r#"DoCoMo/2.0 SH01A(c100;TB;W24H16)"#) {
            None => panic!(r#"invalid parse. "DoCoMo/2.0 SH01A(c100;TB;W24H16)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone".to_string());
                assert_eq!(result.name, "docomo".to_string());
                assert_eq!(result.os, "docomo".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "SH01A".to_string());
            }
        }
        match parser.parse(r#"DoCoMo/2.0 N07B(c500;TB;W24H16)"#) {
            None => panic!(r#"invalid parse. "DoCoMo/2.0 N07B(c500;TB;W24H16)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone".to_string());
                assert_eq!(result.name, "docomo".to_string());
                assert_eq!(result.os, "docomo".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "N07B".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; DoCoMo/1.0/D505i/c20/TB/W20H10; http://www.rcdtokyo.com/pc2m/)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; DoCoMo/1.0/D505i/c20/TB/W20H10; http://www.rcdtokyo.com/pc2m/)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone".to_string());
                assert_eq!(result.name, "docomo".to_string());
                assert_eq!(result.os, "docomo".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "D505i".to_string());
            }
        }
        match parser.parse(r#"DoCoMo/1.0/N505i/c20/TB/W24H12"#) {
            None => panic!(r#"invalid parse. "DoCoMo/1.0/N505i/c20/TB/W24H12""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone".to_string());
                assert_eq!(result.name, "docomo".to_string());
                assert_eq!(result.os, "docomo".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "N505i".to_string());
            }
        }
        match parser.parse(r#"Mozilla/4.08 (N905imyu_W;FOMA;c500;TB)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.08 (N905imyu_W;FOMA;c500;TB)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone".to_string());
                assert_eq!(result.name, "docomo".to_string());
                assert_eq!(result.os, "docomo".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "N905imyu_W".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (F02B;FOMA;like Gecko)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (F02B;FOMA;like Gecko)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone".to_string());
                assert_eq!(result.name, "docomo".to_string());
                assert_eq!(result.os, "docomo".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "F02B".to_string());
            }
        }
    }
}
