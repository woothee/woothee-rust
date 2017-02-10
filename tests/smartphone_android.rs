// This file is auto-generated! Any changes to this file will be lost!
extern crate woothee;

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_smartphone_android() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (Linux; U; Android 2.3.5; ja-jp; ISW11F Build/FGK500) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; U; Android 2.3.5; ja-jp; ISW11F Build/FGK500) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "2.3.5".to_string());
                assert_eq!(result.version, "4.0".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; U; Android 3.1; ja-jp; L-06C Build/HMJ37) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; U; Android 3.1; ja-jp; L-06C Build/HMJ37) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "3.1".to_string());
                assert_eq!(result.version, "4.0".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; U; Android-4.0.3; en-us; Galaxy Nexus Build/IML74K) AppleWebKit/535.7 (KHTML, like Gecko) CrMo/16.0.912.75 Mobile Safari/535.7"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; U; Android-4.0.3; en-us; Galaxy Nexus Build/IML74K) AppleWebKit/535.7 (KHTML, like Gecko) CrMo/16.0.912.75 Mobile Safari/535.7""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Chrome".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "4.0.3".to_string());
                assert_eq!(result.version, "16.0.912.75".to_string());
            }
        }
        match parser.parse(r#"Opera/9.80 (Android; Opera Mini/6.5.27452/26.1305; U; ja) Presto/2.8.119 Version/10.54"#) {
            None => panic!(r#"invalid parse. "Opera/9.80 (Android; Opera Mini/6.5.27452/26.1305; U; ja) Presto/2.8.119 Version/10.54""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Opera".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "10.54".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 4.2.2; SO-01F Build/14.1.H.1.281) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.166 Mobile Safari/537.36 OPR/20.0.1396.73172"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 4.2.2; SO-01F Build/14.1.H.1.281) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.166 Mobile Safari/537.36 OPR/20.0.1396.73172""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Opera".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "4.2.2".to_string());
                assert_eq!(result.version, "20.0.1396.73172".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Android; Mobile; rv:14.0) Gecko/14.0 Firefox/14.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Android; Mobile; rv:14.0) Gecko/14.0 Firefox/14.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Firefox".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "14.0".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Android; Tablet; rv:14.0) Gecko/14.0 Firefox/14.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Android; Tablet; rv:14.0) Gecko/14.0 Firefox/14.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Firefox".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "14.0".to_string());
            }
        }
        match parser.parse(r#"Dalvik/1.4.0 (Linux; U; Android 2.3.4; SBM009SH Build/S0008)"#) {
            None => panic!(r#"invalid parse. "Dalvik/1.4.0 (Linux; U; Android 2.3.4; SBM009SH Build/S0008)""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "UNKNOWN".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "2.3.4".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"LDNReader/2.0.1 (Android)"#) {
            None => panic!(r#"invalid parse. "LDNReader/2.0.1 (Android)""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "UNKNOWN".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 5.1.1; Nexus 5 Build/LMY48B; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/43.0.2357.65 Mobile Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 5 Build/LMY48B; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/43.0.2357.65 Mobile Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Webview".to_string());
                assert_eq!(result.os, "Android".to_string());
                assert_eq!(result.os_version, "5.1.1".to_string());
                assert_eq!(result.version, "4.0".to_string());
            }
        }
    }
}
