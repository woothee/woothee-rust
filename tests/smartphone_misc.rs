// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_smartphone_misc() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (Mobile; rv:18.0) Gecko/18.0 Firefox/18.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Mobile; rv:18.0) Gecko/18.0 Firefox/18.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Firefox OS");
                // NOTE: skip test now
                //assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "18.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Tablet; rv:26.0) Gecko/18.0 Firefox/26.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Tablet; rv:26.0) Gecko/18.0 Firefox/26.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Firefox OS");
                assert_eq!(result.os_version, "26.0".to_string());
                assert_eq!(result.version, "26.0");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 7.0; Windows Phone OS 7.5; Trident/3.1; IEMobile/7.0; FujitsuToshibaMobileCommun; IS12T; KDDI)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 7.0; Windows Phone OS 7.5; Trident/3.1; IEMobile/7.0; FujitsuToshibaMobileCommun; IS12T; KDDI)""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows Phone OS");
                assert_eq!(result.os_version, "7.5".to_string());
                assert_eq!(result.version, "7.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows Phone 8.1; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 930) like Gecko"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows Phone 8.1; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 930) like Gecko""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows Phone OS");
                assert_eq!(result.os_version, "8.1".to_string());
                assert_eq!(result.version, "11.0");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 6.0; Windows CE; IEMobile 7.7) S12HT"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 6.0; Windows CE; IEMobile 7.7) S12HT""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows CE");
                // NOTE: skip test now
                //assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "6.0");
            }
        }
        match parser
            .parse(r#"Opera/9.80 (BlackBerry; Opera Mini/6.5.27548/26.1305; U; ja) Presto/2.8.119 Version/10.54"#)
        {
            None => panic!(
                r#"invalid parse. "Opera/9.80 (BlackBerry; Opera Mini/6.5.27548/26.1305; U; ja) Presto/2.8.119 Version/10.54""#
            ),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "BlackBerry");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "10.54");
            }
        }
        match parser.parse(r#"BlackBerry9700/5.0.0.1014 Profile/MIDP-2.1 Configuration/CLDC-1.1 VendorID/220"#) {
            None => panic!(
                r#"invalid parse. "BlackBerry9700/5.0.0.1014 Profile/MIDP-2.1 Configuration/CLDC-1.1 VendorID/220""#
            ),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "BlackBerry");
                assert_eq!(result.os_version, "5.0.0.1014".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (BB10; Touch) AppleWebKit/537.35+ (KHTML, like Gecko) Version/10.3.1.2243 Mobile Safari/537.35+"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (BB10; Touch) AppleWebKit/537.35+ (KHTML, like Gecko) Version/10.3.1.2243 Mobile Safari/537.35+""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Safari");
                assert_eq!(result.os, "BlackBerry 10");
                assert_eq!(result.os_version, "10.3.1.2243".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
    }
}
