// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_misc() {
        let parser = Parser::new();

        match parser.parse(r#"AppleSyndication/56.1"#) {
            None => panic!(r#"invalid parse. "AppleSyndication/56.1""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "Safari RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; Google Desktop/5.9.1005.12335; http://desktop.google.com/)"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (compatible; Google Desktop/5.9.1005.12335; http://desktop.google.com/)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "Google Desktop");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Windows-RSS-Platform/2.0 (MSIE 9.0; Windows NT 6.0)"#) {
            None => panic!(r#"invalid parse. "Windows-RSS-Platform/2.0 (MSIE 9.0; Windows NT 6.0)""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "Windows RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"RssBar/1.29 (RssBar for unDonut 1.35)"#) {
            None => panic!(r#"invalid parse. "RssBar/1.29 (RssBar for unDonut 1.35)""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"MagpieRSS/0.61 (+http://magpierss.sf.net)"#) {
            None => panic!(r#"invalid parse. "MagpieRSS/0.61 (+http://magpierss.sf.net)""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"gooRSSreader3.7 - build20090410"#) {
            None => panic!(r#"invalid parse. "gooRSSreader3.7 - build20090410""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Fenrir Headline-Reader Plugin"#) {
            None => panic!(r#"invalid parse. "Fenrir Headline-Reader Plugin""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"jsRSS++/3.15"#) {
            None => panic!(r#"invalid parse. "jsRSS++/3.15""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"cococ/1.06"#) {
            None => panic!(r#"invalid parse. "cococ/1.06""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "RSSReader");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Wget/1.12 (linux-gnu)"#) {
            None => panic!(r#"invalid parse. "Wget/1.12 (linux-gnu)""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "wget");
            }
        }
        match parser.parse(r#"Apache-HttpClient/UNAVAILABLE (java 1.4)"#) {
            None => panic!(r#"invalid parse. "Apache-HttpClient/UNAVAILABLE (java 1.4)""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "Java");
            }
        }
        match parser.parse(r#"livedoor HttpClient"#) {
            None => panic!(r#"invalid parse. "livedoor HttpClient""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "Java");
            }
        }
        match parser.parse(r#"Jakarta Commons-HttpClient/3.0"#) {
            None => panic!(r#"invalid parse. "Jakarta Commons-HttpClient/3.0""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "Java");
            }
        }
        match parser.parse(r#"Java/1.5.0_17"#) {
            None => panic!(r#"invalid parse. "Java/1.5.0_17""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "Java");
            }
        }
        match parser.parse(r#"IE6.0,Java(TM) 2 Runtime Environment, Standard Edition"#) {
            None => panic!(r#"invalid parse. "IE6.0,Java(TM) 2 Runtime Environment, Standard Edition""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "Java");
            }
        }
        match parser.parse(r#"libwww-perl/5.835"#) {
            None => panic!(r#"invalid parse. "libwww-perl/5.835""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "perl");
            }
        }
        match parser.parse(r#"WWW-Mechanize/1.64"#) {
            None => panic!(r#"invalid parse. "WWW-Mechanize/1.64""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "perl");
            }
        }
        match parser.parse(r#"LWP::Simple/5.800"#) {
            None => panic!(r#"invalid parse. "LWP::Simple/5.800""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "perl");
            }
        }
        match parser.parse(r#"LWP LDMusicNews::LDNewsAPI"#) {
            None => panic!(r#"invalid parse. "LWP LDMusicNews::LDNewsAPI""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "perl");
            }
        }
        match parser.parse(r#"lwp-trivial/1.41"#) {
            None => panic!(r#"invalid parse. "lwp-trivial/1.41""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "perl");
            }
        }
        match parser.parse(r#"Ruby"#) {
            None => panic!(r#"invalid parse. "Ruby""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "ruby");
            }
        }
        match parser.parse(r#"feedzirra http://github.com/pauldix/feedzirra/tree/master"#) {
            None => panic!(r#"invalid parse. "feedzirra http://github.com/pauldix/feedzirra/tree/master""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "ruby");
            }
        }
        match parser.parse(r#"Typhoeus - https://github.com/typhoeus/typhoeus"#) {
            None => panic!(r#"invalid parse. "Typhoeus - https://github.com/typhoeus/typhoeus""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "ruby");
            }
        }
        match parser.parse(r#"Python-urllib/1.16"#) {
            None => panic!(r#"invalid parse. "Python-urllib/1.16""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "python");
            }
        }
        match parser.parse(r#"Twisted PageGetter"#) {
            None => panic!(r#"invalid parse. "Twisted PageGetter""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "python");
            }
        }
        match parser.parse(r#"PHP/5.2.13"#) {
            None => panic!(r#"invalid parse. "PHP/5.2.13""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"PHP"#) {
            None => panic!(r#"invalid parse. "PHP""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"PEAR HTTP_Request class"#) {
            None => panic!(r#"invalid parse. "PEAR HTTP_Request class""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"HTTP_Request2/2.1.1 (http://pear.php.net/package/http_request2) PHP/5.3.10-1ubuntu3.6"#) {
            None => panic!(
                r#"invalid parse. "HTTP_Request2/2.1.1 (http://pear.php.net/package/http_request2) PHP/5.3.10-1ubuntu3.6""#
            ),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"PECL::HTTP/1.7.4 (PHP/5.4.4)"#) {
            None => panic!(r#"invalid parse. "PECL::HTTP/1.7.4 (PHP/5.4.4)""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"WordPress/3.2.1; http://www.painlog.jp"#) {
            None => panic!(r#"invalid parse. "WordPress/3.2.1; http://www.painlog.jp""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"CakePHP"#) {
            None => panic!(r#"invalid parse. "CakePHP""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"PukiWiki/1.4.6"#) {
            None => panic!(r#"invalid parse. "PukiWiki/1.4.6""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; PEAR HTTP_Request class;)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; PEAR HTTP_Request class;)""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "php");
            }
        }
        match parser.parse(r#"curl/7.19.7 (x86_64-redhat-linux-gnu) libcurl/7.19.7 NSS/3.19.1 Basic ECC zlib/1.2.3 libidn/1.18 libssh2/1.4.2"#) {
            None => panic!(r#"invalid parse. "curl/7.19.7 (x86_64-redhat-linux-gnu) libcurl/7.19.7 NSS/3.19.1 Basic ECC zlib/1.2.3 libidn/1.18 libssh2/1.4.2""#),
            Some(result) => {
                assert_eq!(result.category, "misc");
                assert_eq!(result.name, "HTTP Library");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "curl");
            }
        }
    }
}
