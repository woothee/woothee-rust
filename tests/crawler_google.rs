// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_crawler_google() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"#) {
            None => {
                panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)""#)
            }
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Googlebot");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Googlebot-Image/1.0"#) {
            None => panic!(r#"invalid parse. "Googlebot-Image/1.0""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Googlebot");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(
            r#"DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"#,
        ) {
            None => panic!(
                r#"invalid parse. "DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Googlebot Mobile");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"SAMSUNG-SGH-E250/1.0 Profile/MIDP-2.0 Configuration/CLDC-1.1 UP.Browser/6.2.3.3.c.1.101 (GUI) MMP/2.0 (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"#) {
            None => panic!(r#"invalid parse. "SAMSUNG-SGH-E250/1.0 Profile/MIDP-2.0 Configuration/CLDC-1.1 UP.Browser/6.2.3.3.c.1.101 (GUI) MMP/2.0 (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Googlebot Mobile");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"DoCoMo/2.0 SH905i(c100;TB;W24H16) (compatible; Mediapartners-Google/2.1; +http://www.google.com/bot.html)"#) {
            None => panic!(r#"invalid parse. "DoCoMo/2.0 SH905i(c100;TB;W24H16) (compatible; Mediapartners-Google/2.1; +http://www.google.com/bot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Google Mediapartners");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mediapartners-Google"#) {
            None => panic!(r#"invalid parse. "Mediapartners-Google""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Google Mediapartners");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser
            .parse(r#"Feedfetcher-Google; (+http://www.google.com/feedfetcher.html; feed-id=000000000000000000)"#)
        {
            None => panic!(
                r#"invalid parse. "Feedfetcher-Google; (+http://www.google.com/feedfetcher.html; feed-id=000000000000000000)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Google Feedfetcher");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"AppEngine-Google"#) {
            None => panic!(r#"invalid parse. "AppEngine-Google""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Google AppEngine");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (en-us) AppleWebKit/525.13 (KHTML, like Gecko; Google Web Preview) Version/3.1 Safari/525.13"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (en-us) AppleWebKit/525.13 (KHTML, like Gecko; Google Web Preview) Version/3.1 Safari/525.13""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Google Web Preview");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"FeedBurner/1.0 (http://www.FeedBurner.com)"#) {
            None => panic!(r#"invalid parse. "FeedBurner/1.0 (http://www.FeedBurner.com)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "Google FeedBurner");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; Android 5.0; SM-G920A) AppleWebKit (KHTML, like Gecko) Chrome Mobile Safari (compatible; AdsBot-Google-Mobile; +http://www.google.com/mobile/adsbot.html)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; Android 5.0; SM-G920A) AppleWebKit (KHTML, like Gecko) Chrome Mobile Safari (compatible; AdsBot-Google-Mobile; +http://www.google.com/mobile/adsbot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "AdsBot-Google-Mobile");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"AdsBot-Google (+http://www.google.com/adsbot.html)"#) {
            None => panic!(r#"invalid parse. "AdsBot-Google (+http://www.google.com/adsbot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "AdsBot-Google");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
    }
}
