// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_smartphone_android() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (Linux; U; Android 2.3.5; ja-jp; ISW11F Build/FGK500) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; U; Android 2.3.5; ja-jp; ISW11F Build/FGK500) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Safari");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "2.3.5".to_string());
                assert_eq!(result.version, "4.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; U; Android 3.1; ja-jp; L-06C Build/HMJ37) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; U; Android 3.1; ja-jp; L-06C Build/HMJ37) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Safari");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "3.1".to_string());
                assert_eq!(result.version, "4.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; U; Android-4.0.3; en-us; Galaxy Nexus Build/IML74K) AppleWebKit/535.7 (KHTML, like Gecko) CrMo/16.0.912.75 Mobile Safari/535.7"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; U; Android-4.0.3; en-us; Galaxy Nexus Build/IML74K) AppleWebKit/535.7 (KHTML, like Gecko) CrMo/16.0.912.75 Mobile Safari/535.7""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Chrome");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "4.0.3".to_string());
                assert_eq!(result.version, "16.0.912.75");
            }
        }
        match parser.parse(r#"Opera/9.80 (Android; Opera Mini/6.5.27452/26.1305; U; ja) Presto/2.8.119 Version/10.54"#)
        {
            None => panic!(
                r#"invalid parse. "Opera/9.80 (Android; Opera Mini/6.5.27452/26.1305; U; ja) Presto/2.8.119 Version/10.54""#
            ),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "10.54");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 4.2.2; SO-01F Build/14.1.H.1.281) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.166 Mobile Safari/537.36 OPR/20.0.1396.73172"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 4.2.2; SO-01F Build/14.1.H.1.281) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.166 Mobile Safari/537.36 OPR/20.0.1396.73172""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "4.2.2".to_string());
                assert_eq!(result.version, "20.0.1396.73172");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Android; Mobile; rv:14.0) Gecko/14.0 Firefox/14.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Android; Mobile; rv:14.0) Gecko/14.0 Firefox/14.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "14.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Android 11; Mobile; rv:93.0) Gecko/93.0 Firefox/93.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Android 11; Mobile; rv:93.0) Gecko/93.0 Firefox/93.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "11".to_string());
                assert_eq!(result.version, "93.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Android; Tablet; rv:14.0) Gecko/14.0 Firefox/14.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Android; Tablet; rv:14.0) Gecko/14.0 Firefox/14.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "14.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 8.0; Pixel XL Build/OPP3.170518.006) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.0 Mobile Safari/537.36 EdgA/41.1.35.1"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 8.0; Pixel XL Build/OPP3.170518.006) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.0 Mobile Safari/537.36 EdgA/41.1.35.1""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Edge");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "8.0".to_string());
                assert_eq!(result.version, "41.1.35.1");
            }
        }
        match parser.parse(r#"Dalvik/1.4.0 (Linux; U; Android 2.3.4; SBM009SH Build/S0008)"#) {
            None => panic!(r#"invalid parse. "Dalvik/1.4.0 (Linux; U; Android 2.3.4; SBM009SH Build/S0008)""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "2.3.4".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"LDNReader/2.0.1 (Android)"#) {
            None => panic!(r#"invalid parse. "LDNReader/2.0.1 (Android)""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 5.1.1; Nexus 5 Build/LMY48B; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/43.0.2357.65 Mobile Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 5 Build/LMY48B; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/43.0.2357.65 Mobile Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Webview");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "5.1.1".to_string());
                assert_eq!(result.version, "4.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 9; SM-N960F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.105 Mobile Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 9; SM-N960F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.105 Mobile Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "Chrome");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "9".to_string());
                assert_eq!(result.version, "72.0.3626.105");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 5.0.2; SAMSUNG SM-G925F Build/LRX22G) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/4.0 Chrome/44.0.2403.133 Mobile Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 5.0.2; SAMSUNG SM-G925F Build/LRX22G) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/4.0 Chrome/44.0.2403.133 Mobile Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone");
                assert_eq!(result.name, "SamsungBrowser");
                assert_eq!(result.os, "Android");
                assert_eq!(result.os_version, "5.0.2".to_string());
                assert_eq!(result.version, "4.0");
            }
        }
    }
}
