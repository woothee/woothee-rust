// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_crawler_nonmajor() {
        let parser = Parser::new();

        match parser.parse(r#"emBot-GalaBuzz/Nutch-1.0 (http://emining.jp/; em@galabuzz.jp)"#) {
            None => panic!(r#"invalid parse. "emBot-GalaBuzz/Nutch-1.0 (http://emining.jp/; em@galabuzz.jp)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows; U; Windows NT 6.0; en-US; aggregator VocusBot 0.4; +http://www.vocus.com/vnhs.html)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows; U; Windows NT 6.0; en-US; aggregator VocusBot 0.4; +http://www.vocus.com/vnhs.html)""#),
            Some(result) => {
                assert_eq!(result.category, "pc");
                assert_eq!(result.name, "UNKNOWN");
                assert_eq!(result.os, "Windows Vista");
                // NOTE: skip test now
                //assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; Ezooms/1.0; ezooms.bot@gmail.com)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Ezooms/1.0; ezooms.bot@gmail.com)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; Rakutenbot/1.0; +http://dynamic.rakuten.co.jp/bot.html)"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (compatible; Rakutenbot/1.0; +http://dynamic.rakuten.co.jp/bot.html)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Flamingo_SearchEngine (+http://www.flamingosearch.com/bot)"#) {
            None => panic!(r#"invalid parse. "Flamingo_SearchEngine (+http://www.flamingosearch.com/bot)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#""mapion-news-bot/1.0 (http://www.mapion.co.jp/news/)""#) {
            None => panic!(r#"invalid parse. ""mapion-news-bot/1.0 (http://www.mapion.co.jp/news/)"""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; MJ12bot/v1.4.0; http://www.majestic12.co.uk/bot.php?+)"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (compatible; MJ12bot/v1.4.0; http://www.majestic12.co.uk/bot.php?+)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; TweetmemeBot/2.11; +http://tweetmeme.com/)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; TweetmemeBot/2.11; +http://tweetmeme.com/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(
            r#"Mozilla/5.0 (compatible; PaperLiBot/2.1; http://support.paper.li/entries/20023257-what-is-paper-li)"#,
        ) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (compatible; PaperLiBot/2.1; http://support.paper.li/entries/20023257-what-is-paper-li)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"SearQuBot/SearQuBot v1.0"#) {
            None => panic!(r#"invalid parse. "SearQuBot/SearQuBot v1.0""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; ADJUSTbot/2.0; +http://www.ad-just.jp/)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; ADJUSTbot/2.0; +http://www.ad-just.jp/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"FTRF: Friendly robot/1.3"#) {
            None => panic!(r#"invalid parse. "FTRF: Friendly robot/1.3""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"kizasi-spider/1.0 (+http://kizasi.jp/)"#) {
            None => panic!(r#"invalid parse. "kizasi-spider/1.0 (+http://kizasi.jp/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"BlogramCrawler/1.0.1(+http://blogram.jp/)"#) {
            None => panic!(r#"invalid parse. "BlogramCrawler/1.0.1(+http://blogram.jp/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"www2.apserver.net ASP-Ranker Feed Crawler"#) {
            None => panic!(r#"invalid parse. "www2.apserver.net ASP-Ranker Feed Crawler""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Rome Client (http://tinyurl.com/64t5n)"#) {
            None => panic!(r#"invalid parse. "Rome Client (http://tinyurl.com/64t5n)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"UnwindFetchor/1.0 (+http://www.gnip.com/)"#) {
            None => panic!(r#"invalid parse. "UnwindFetchor/1.0 (+http://www.gnip.com/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"ia_archiver (+http://www.alexa.com/site/help/webmasters; crawler@alexa.com)"#) {
            None => panic!(
                r#"invalid parse. "ia_archiver (+http://www.alexa.com/site/help/webmasters; crawler@alexa.com)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Summify (Summify/1.0.1; +http://summify.com)"#) {
            None => panic!(r#"invalid parse. "Summify (Summify/1.0.1; +http://summify.com)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"PostRank/2.0 (postrank.com; 1 subscribers)"#) {
            None => panic!(r#"invalid parse. "PostRank/2.0 (postrank.com; 1 subscribers)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"cloudforecastbot"#) {
            None => panic!(r#"invalid parse. "cloudforecastbot""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser
            .parse(r#"SimplePie/1.3-dev (Feed Parser; http://simplepie.org; Allow like Gecko) Build/20111118194710"#)
        {
            None => panic!(
                r#"invalid parse. "SimplePie/1.3-dev (Feed Parser; http://simplepie.org; Allow like Gecko) Build/20111118194710""#
            ),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Rainmeter WebParser plugin"#) {
            None => panic!(r#"invalid parse. "Rainmeter WebParser plugin""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Data-Hotel-Watchdog/1.1"#) {
            None => panic!(r#"invalid parse. "Data-Hotel-Watchdog/1.1""#),
            Some(result) => {
                assert_eq!(result.category, "crawler");
                assert_eq!(result.name, "misc crawler");
                assert_eq!(result.os, "UNKNOWN");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
    }
}
