// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_mobilephone_misc() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (jig browser core; SH03B)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (jig browser core; SH03B)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "jig browser");
                assert_eq!(result.os, "jig");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "SH03B");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (jig browser9i 1.5.0; F10B; 2004)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (jig browser9i 1.5.0; F10B; 2004)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "jig browser");
                assert_eq!(result.os, "jig");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "F10B");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (jig browser9)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (jig browser9)""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "jig browser");
                assert_eq!(result.os, "jig");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"emobile/1.0.0 (H11T; like Gecko; Wireless) NetFront/3.4"#) {
            None => panic!(r#"invalid parse. "emobile/1.0.0 (H11T; like Gecko; Wireless) NetFront/3.4""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "emobile");
                assert_eq!(result.os, "emobile");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"IAC/1.0 (H31IA;like Gecko;OpenBrowser) WWW Browser/ver1.0"#) {
            None => panic!(r#"invalid parse. "IAC/1.0 (H31IA;like Gecko;OpenBrowser) WWW Browser/ver1.0""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "emobile");
                assert_eq!(result.os, "emobile");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (H11T; like Gecko; OpenBrowser) NetFront/3.4"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (H11T; like Gecko; OpenBrowser) NetFront/3.4""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "emobile");
                assert_eq!(result.os, "emobile");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Huawei/1.0/H12HW/B000 Browser/Obigo-Browser/Q04A"#) {
            None => panic!(r#"invalid parse. "Huawei/1.0/H12HW/B000 Browser/Obigo-Browser/Q04A""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "emobile");
                assert_eq!(result.os, "emobile");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (SymbianOS/9.2; U; Series60/3.1 NokiaE66-1/500.21.009; Profile/MIDP-2.0 Configuration/CLDC-1.1 ) AppleWebKit/413 (KHTML, like Gecko) Safari/413"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (SymbianOS/9.2; U; Series60/3.1 NokiaE66-1/500.21.009; Profile/MIDP-2.0 Configuration/CLDC-1.1 ) AppleWebKit/413 (KHTML, like Gecko) Safari/413""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "Safari");
                assert_eq!(result.os, "SymbianOS");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (en-us) AppleWebKit/534.14 (KHTML, like Gecko; Google Wireless Transcoder) Chrome/9.0.597 Safari/534.14"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (en-us) AppleWebKit/534.14 (KHTML, like Gecko; Google Wireless Transcoder) Chrome/9.0.597 Safari/534.14""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "Mobile Transcoder");
                assert_eq!(result.os, "Mobile Transcoder");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "Google");
            }
        }
        match parser
            .parse(r#"Mozilla/5.0 (compatible; livedoor-Mobile-Gateway/0.02; +http://p.m.livedoor.com/help.html)"#)
        {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (compatible; livedoor-Mobile-Gateway/0.02; +http://p.m.livedoor.com/help.html)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "Mobile Transcoder");
                assert_eq!(result.os, "Mobile Transcoder");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "livedoor");
            }
        }
    }
}
