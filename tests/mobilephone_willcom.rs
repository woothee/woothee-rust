// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_mobilephone_willcom() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/3.0(WILLCOM;TOSHIBA/WX320T/2;1/1/C128) NetFront/3.4"#) {
            None => panic!(r#"invalid parse. "Mozilla/3.0(WILLCOM;TOSHIBA/WX320T/2;1/1/C128) NetFront/3.4""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "WILLCOM");
                assert_eq!(result.os, "WILLCOM");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "WX320T");
            }
        }
        match parser.parse(r#"Mozilla/3.0(WILLCOM;SANYO/WX310SA/2;1/1/C128) NetFront/3.3"#) {
            None => panic!(r#"invalid parse. "Mozilla/3.0(WILLCOM;SANYO/WX310SA/2;1/1/C128) NetFront/3.3""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "WILLCOM");
                assert_eq!(result.os, "WILLCOM");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "WX310SA");
            }
        }
        match parser.parse(r#"Mozilla/3.0(WILLCOM;KYOCERA/WX331K/2;1.0.8.13.000000/0.1/C100) Opera 7.2 EX"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/3.0(WILLCOM;KYOCERA/WX331K/2;1.0.8.13.000000/0.1/C100) Opera 7.2 EX""#
            ),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "WILLCOM");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "WX331K");
            }
        }
        match parser.parse(r#"Mozilla/3.0(DDIPOCKET;JRC/AH-J3001V,AH-J3002V/1.0/0100/c50)CNF/2.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/3.0(DDIPOCKET;JRC/AH-J3001V,AH-J3002V/1.0/0100/c50)CNF/2.0""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "WILLCOM");
                assert_eq!(result.os, "WILLCOM");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "AH-J3001V,AH-J3002V");
            }
        }
        match parser.parse(r#"Mozilla/3.0(DDIPOCKET;KYOCERA/AH-K3001V/1.4.1.67.000000/0.1/C100) Opera 7.0"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/3.0(DDIPOCKET;KYOCERA/AH-K3001V/1.4.1.67.000000/0.1/C100) Opera 7.0""#
            ),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "WILLCOM");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "AH-K3001V");
            }
        }
    }
}
