// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_pc_misc() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_5_4; ja-jp) AppleWebKit/525.18 (KHTML, like Gecko) Version/3.1.2 Safari/525.20.1"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_5_4; ja-jp) AppleWebKit/525.18 (KHTML, like Gecko) Version/3.1.2 Safari/525.20.1""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Safari");
                assert_eq!(result.os, "Mac OSX");
                assert_eq!(result.os_version, "10.5.4".to_string());
                assert_eq!(result.version, "3.1.2");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/27.0.1453.93 Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/27.0.1453.93 Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Chrome");
                assert_eq!(result.os, "Mac OSX");
                assert_eq!(result.os_version, "10.7.5".to_string());
                assert_eq!(result.version, "27.0.1453.93");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.7; rv:21.0) Gecko/20100101 Firefox/21.0"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.7; rv:21.0) Gecko/20100101 Firefox/21.0""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Mac OSX");
                assert_eq!(result.os_version, "10.7".to_string());
                assert_eq!(result.version, "21.0");
            }
        }
        match parser.parse(r#"Opera/9.80 (Macintosh; Intel Mac OS X 10.8.3) Presto/2.12.388 Version/12.15"#) {
            None => panic!(
                r#"invalid parse. "Opera/9.80 (Macintosh; Intel Mac OS X 10.8.3) Presto/2.12.388 Version/12.15""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Mac OSX");
                assert_eq!(result.os_version, "10.8.3".to_string());
                assert_eq!(result.version, "12.15");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.154 Safari/537.36 OPR/20.0.1387.82"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.154 Safari/537.36 OPR/20.0.1387.82""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Mac OSX");
                assert_eq!(result.os_version, "10.9.2".to_string());
                assert_eq!(result.version, "20.0.1387.82");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.94 Safari/537.36 Vivaldi/1.1.453.52"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.94 Safari/537.36 Vivaldi/1.1.453.52""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Vivaldi");
                assert_eq!(result.os, "Mac OSX");
                assert_eq!(result.os_version, "10.11.4".to_string());
                assert_eq!(result.version, "1.1.453.52");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.50 (KHTML, like Gecko) Version/5.1 Instapaper/4.0 (+http://www.instapaper.com/)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.50 (KHTML, like Gecko) Version/5.1 Instapaper/4.0 (+http://www.instapaper.com/)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Mac OSX");
                // NOTE: skip test now
                //assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(
            r#"Mozilla/5.0 (Macintosh; U; PPC Mac OS X 10.5; ja-JP-mac; rv:1.9.1.19) Gecko/20110420 SeaMonkey/2.0.14"#,
        ) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Macintosh; U; PPC Mac OS X 10.5; ja-JP-mac; rv:1.9.1.19) Gecko/20110420 SeaMonkey/2.0.14""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Mac OSX");
                assert_eq!(result.os_version, "10.5".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Macintosh; U; PPC; en-US; mimic; rv:9.2.1) (mimic Gecko/20100722 Firefox/3.6.8) Classilla/CFM"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Macintosh; U; PPC; en-US; mimic; rv:9.2.1) (mimic Gecko/20100722 Firefox/3.6.8) Classilla/CFM""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Mac OS Classic");
                assert_eq!(result.os_version, "9.2.1".to_string());
                assert_eq!(result.version, "3.6.8");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (compatible; MSIE 5.17; Mac_PowerPC)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (compatible; MSIE 5.17; Mac_PowerPC)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Internet Explorer");
                assert_eq!(result.os, "Mac OS Classic");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "5.17");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Ubuntu; X11; Linux i686; rv:9.0.1) Gecko/20100101 Firefox/9.0.1"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (Ubuntu; X11; Linux i686; rv:9.0.1) Gecko/20100101 Firefox/9.0.1""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "Linux");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "9.0.1");
            }
        }
        match parser.parse(
            r#"Mozilla/5.0 (X11; Linux i686) AppleWebKit/535.7 (KHTML, like Gecko) Chrome/16.0.912.75 Safari/535.7"#,
        ) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (X11; Linux i686) AppleWebKit/535.7 (KHTML, like Gecko) Chrome/16.0.912.75 Safari/535.7""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Chrome");
                assert_eq!(result.os, "Linux");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "16.0.912.75");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/45.0.2454.99 Safari/537.36 Vivaldi/1.0.321.3"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/45.0.2454.99 Safari/537.36 Vivaldi/1.0.321.3""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Vivaldi");
                assert_eq!(result.os, "Linux");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "1.0.321.3");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.2.24) Gecko cXense"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.2.24) Gecko cXense""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Linux");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (X11; U; Linux i686; ja-JP; rv:1.8.1.23) Gecko/20090910 SeaMonkey/1.1.18"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (X11; U; Linux i686; ja-JP; rv:1.8.1.23) Gecko/20090910 SeaMonkey/1.1.18""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Linux");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.0.3) Gecko/2008092814 Iceweasel/3.0.3 (Debian-3.0.3-3)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.0.3) Gecko/2008092814 Iceweasel/3.0.3 (Debian-3.0.3-3)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Linux");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (X11; FreeBSD amd64; rv:8.0) Gecko/20100101 Firefox/8.0"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (X11; FreeBSD amd64; rv:8.0) Gecko/20100101 Firefox/8.0""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Firefox");
                assert_eq!(result.os, "BSD");
                // NOTE: skip test now
                //assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "8.0");
            }
        }
        match parser.parse(r#"Opera/9.80 (X11; FreeBSD 8.2-RELEASE-p3 amd64; U; ja) Presto/2.9.168 Version/11.52"#) {
            None => panic!(
                r#"invalid parse. "Opera/9.80 (X11; FreeBSD 8.2-RELEASE-p3 amd64; U; ja) Presto/2.9.168 Version/11.52""#
            ),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "BSD");
                assert_eq!(result.os_version, "8.2-RELEASE-p3 amd64".to_string());
                assert_eq!(result.version, "11.52");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (X11; CrOS x86_64 5116.115.4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.152 Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (X11; CrOS x86_64 5116.115.4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.152 Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Chrome");
                assert_eq!(result.os, "ChromeOS");
                assert_eq!(result.os_version, "x86_64 5116.115.4".to_string());
                assert_eq!(result.version, "33.0.1750.152");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 YaBrowser/17.11.1.1087 (beta) Yowser/2.5 Safari/537.36"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 YaBrowser/17.11.1.1087 (beta) Yowser/2.5 Safari/537.36""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "Yandex Browser");
                assert_eq!(result.os, "Linux");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "17.11.1.1087");
            }
        }
    }
}
