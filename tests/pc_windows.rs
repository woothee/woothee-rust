// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_pc_windows() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; InfoPath.1)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0; InfoPath.1)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 2000");
                assert_eq!(result.os_version, "NT 5.0".to_string());
                assert_eq!(result.version, "6.0");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322)"#) {
            None => {
                panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322)""#)
            }
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "7.0");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; 2004/11/08; GoogleT5; .NET CLR 1.1.4322; .NET CLR 2.0.50727; InfoPath.1; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; 2004/11/08; GoogleT5; .NET CLR 1.1.4322; .NET CLR 2.0.50727; InfoPath.1; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "8.0");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1) ; .NET CLR 2.0.50727; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729; .NET4.0C; BOIE8;ENUSMSCOM)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1) ; .NET CLR 2.0.50727; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729; .NET4.0C; BOIE8;ENUSMSCOM)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "8.0");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.0; Trident/4.0)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.0; Trident/4.0)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows Vista");
                assert_eq!(result.os_version, "NT 6.0".to_string());
                assert_eq!(result.version, "8.0");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 7");
                assert_eq!(result.os_version, "NT 6.1".to_string());
                assert_eq!(result.version, "8.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 7");
                assert_eq!(result.os_version, "NT 6.1".to_string());
                assert_eq!(result.version, "9.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Win64; x64; Trident/6.0)"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Win64; x64; Trident/6.0)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 8");
                assert_eq!(result.os_version, "NT 6.2".to_string());
                assert_eq!(result.version, "10.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.1; Trident/7.0; BOIE9;JAJP; rv:11.0) like Gecko"#) {
            None => {
                panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.1; Trident/7.0; BOIE9;JAJP; rv:11.0) like Gecko""#)
            }
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 7");
                assert_eq!(result.os_version, "NT 6.1".to_string());
                assert_eq!(result.version, "11.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.3; Trident/7.0; rv:11.0) like Gecko"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.3; Trident/7.0; rv:11.0) like Gecko""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 8.1");
                assert_eq!(result.os_version, "NT 6.3".to_string());
                assert_eq!(result.version, "11.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.1; Trident/7.0; MALCJS; rv:11.0) like Gecko"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.1; Trident/7.0; MALCJS; rv:11.0) like Gecko""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 7");
                assert_eq!(result.os_version, "NT 6.1".to_string());
                assert_eq!(result.version, "11.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.1; Win64; x64; Trident/7.0; MASPJS; rv:11.0) like Gecko"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Windows NT 6.1; Win64; x64; Trident/7.0; MASPJS; rv:11.0) like Gecko""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 7");
                assert_eq!(result.os_version, "NT 6.1".to_string());
                assert_eq!(result.version, "11.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; Touch; rv:11.0) like Gecko"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; Touch; rv:11.0) like Gecko""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 8.1");
                assert_eq!(result.os_version, "NT 6.3".to_string());
                assert_eq!(result.version, "11.0");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.3; Win64; x64; Trident/7.0; Touch; rv:11.0) like Gecko"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Windows NT 6.3; Win64; x64; Trident/7.0; Touch; rv:11.0) like Gecko""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows 8.1");
                assert_eq!(result.os_version, "NT 6.3".to_string());
                assert_eq!(result.version, "11.0");
            }
        }
        match parser.parse(r#"Mozilla/4.78 [ja] (Win98; U)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.78 [ja] (Win98; U)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Windows 98");
                assert_eq!(result.os_version, "98".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534+ (KHTML, like Gecko)"#) {
            None => {
                panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534+ (KHTML, like Gecko)""#)
            }
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Windows 7");
                assert_eq!(result.os_version, "NT 6.1".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.10240"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.10240""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Edge");
                assert_eq!(result.os, "Windows 10");
                assert_eq!(result.os_version, "NT 10.0".to_string());
                // NOTE: skip test now
                //assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.48 Safari/537.36 Edg/74.1.96.24"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.48 Safari/537.36 Edg/74.1.96.24""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Edge");
                assert_eq!(result.os, "Windows 10");
                assert_eq!(result.os_version, "NT 10.0".to_string());
                // NOTE: skip test now
                //assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/525.13 (KHTML, like Gecko) Chrome/0.2.149.27 Safari/525.13"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/525.13 (KHTML, like Gecko) Chrome/0.2.149.27 Safari/525.13""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Chrome");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "0.2.149.27");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.0; rv:9.0.1) Gecko/20100101 Firefox/9.0.1"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.0; rv:9.0.1) Gecko/20100101 Firefox/9.0.1""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Windows Vista");
                assert_eq!(result.os_version, "NT 6.0".to_string());
                assert_eq!(result.version, "9.0.1");
            }
        }
        match parser.parse(
            r#"Mozilla/5.0 (Windows NT 5.1) AppleWebKit/534.52.7 (KHTML, like Gecko) Version/5.1.2 Safari/534.52.7"#,
        ) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/534.52.7 (KHTML, like Gecko) Version/5.1.2 Safari/534.52.7""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Safari");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "5.1.2");
            }
        }
        match parser.parse(r#"Opera/9.52 (Windows NT 5.1; U; ja)"#) {
            None => panic!(r#"invalid parse. "Opera/9.52 (Windows NT 5.1; U; ja)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "9.52");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/48.0.2564.88 Safari/537.36 Vivaldi/1.0.380.2"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/48.0.2564.88 Safari/537.36 Vivaldi/1.0.380.2""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Vivaldi");
                assert_eq!(result.os, "Windows 10");
                assert_eq!(result.os_version, "NT 10.0".to_string());
                assert_eq!(result.version, "1.0.380.2");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.117 Safari/537.36 OPR/20.0.1387.64"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.117 Safari/537.36 OPR/20.0.1387.64""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Windows 8.1");
                assert_eq!(result.os_version, "NT 6.3".to_string());
                assert_eq!(result.version, "20.0.1387.64");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; .NET CLR 2.0.50727; InfoPath.1) Sleipnir/2.8.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322; .NET CLR 2.0.50727; InfoPath.1) Sleipnir/2.8.0""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "6.0");
            }
        }
        match parser
            .parse(r#"Mozilla/5.0 (Windows; U; Windows NT 6.0; ja-JP; rv:1.4) Gecko/20030624 Netscape/7.1 (ax)"#)
        {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Windows; U; Windows NT 6.0; ja-JP; rv:1.4) Gecko/20030624 Netscape/7.1 (ax)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Windows Vista");
                assert_eq!(result.os_version, "NT 6.0".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 5.1; rv:8.0) Gecko/20111105 Thunderbird/8.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 5.1; rv:8.0) Gecko/20111105 Thunderbird/8.0""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Windows XP");
                assert_eq!(result.os_version, "NT 5.1".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 YaBrowser/18.1.1.839 Yowser/2.5 Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 YaBrowser/18.1.1.839 Yowser/2.5 Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Yandex Browser");
                assert_eq!(result.os, "Windows 7");
                assert_eq!(result.os_version, "NT 6.1".to_string());
                assert_eq!(result.version, "18.1.1.839");
            }
        }
    }
}
