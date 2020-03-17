// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_appliance() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (Nintendo 3DS; U; ; ja) Version/1.7455.JP"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Nintendo 3DS; U; ; ja) Version/1.7455.JP""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "Nintendo 3DS");
                assert_eq!(result.os, "Nintendo 3DS");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Opera/9.50 (Nintendo DSi; Opera/507; U; ja)"#) {
            None => panic!(r#"invalid parse. "Opera/9.50 (Nintendo DSi; Opera/507; U; ja)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Nintendo DSi");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "9.50");
            }
        }
        match parser.parse(r#"Opera/9.30 (Nintendo Wii; U; ; 3642; ja)"#) {
            None => panic!(r#"invalid parse. "Opera/9.30 (Nintendo Wii; U; ; 3642; ja)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "Opera");
                assert_eq!(result.os, "Nintendo Wii");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "9.30");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Nintendo WiiU) AppleWebKit/534.52 (KHTML, like Gecko) NX/2.1.0.8.21 NintendoBrowser/1.0.0.7494.US"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Nintendo WiiU) AppleWebKit/534.52 (KHTML, like Gecko) NX/2.1.0.8.21 NintendoBrowser/1.0.0.7494.US""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "Nintendo Wii U");
                assert_eq!(result.os, "Nintendo Wii U");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (PLAYSTATION 3; 1.00)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (PLAYSTATION 3; 1.00)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "PlayStation 3");
                assert_eq!(result.os, "PlayStation 3");
                assert_eq!(result.os_version, "1.00".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (PLAYSTATION 3 4.31) AppleWebKit/531.22.8 (KHTML, like Gecko)"#) {
            None => {
                panic!(r#"invalid parse. "Mozilla/5.0 (PLAYSTATION 3 4.31) AppleWebKit/531.22.8 (KHTML, like Gecko)""#)
            }
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "PlayStation 3");
                assert_eq!(result.os, "PlayStation 3");
                assert_eq!(result.os_version, "4.31".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (PlayStation 4 1.000) AppleWebKit/536.26 (KHTML, like Gecko)"#) {
            None => {
                panic!(r#"invalid parse. "Mozilla/5.0 (PlayStation 4 1.000) AppleWebKit/536.26 (KHTML, like Gecko)""#)
            }
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "PlayStation 4");
                assert_eq!(result.os, "PlayStation 4");
                assert_eq!(result.os_version, "1.000".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/4.0 (PSP (PlayStation Portable); 2.00)"#) {
            None => panic!(r#"invalid parse. "Mozilla/4.0 (PSP (PlayStation Portable); 2.00)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "PlayStation Portable");
                assert_eq!(result.os, "PlayStation Portable");
                assert_eq!(result.os_version, "2.00".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (PlayStation Vita 1.51) AppleWebKit/531.22.8 (KHTML, like Gecko) Silk/3.2"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (PlayStation Vita 1.51) AppleWebKit/531.22.8 (KHTML, like Gecko) Silk/3.2""#
            ),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "PlayStation Vita");
                assert_eq!(result.os, "PlayStation Vita");
                assert_eq!(result.os_version, "1.51".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "Xbox 360");
                assert_eq!(result.os, "Xbox 360");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                // NOTE: skip test now
                //assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Trident/6.0; Xbox; Xbox One)"#) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Trident/6.0; Xbox; Xbox One)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "Xbox One");
                assert_eq!(result.os, "Xbox One");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                // NOTE: skip test now
                //assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (DTV; TSBNetTV/TXXXXXXXXX.0203.ADD; like Gecko) NetFront/3.4 DTVNetBrowser/2.2 (000039;TXXXXXXXXX;0203;ADD) InettvBrowser/2.2 (000039;TXXXXXXXXX;0203;ADD) YahooDTV/1.1 (Video=0x03;Audio=0x01;Screen=0x01;Device=0x00;Remote=0x10)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (DTV; TSBNetTV/TXXXXXXXXX.0203.ADD; like Gecko) NetFront/3.4 DTVNetBrowser/2.2 (000039;TXXXXXXXXX;0203;ADD) InettvBrowser/2.2 (000039;TXXXXXXXXX;0203;ADD) YahooDTV/1.1 (Video=0x03;Audio=0x01;Screen=0x01;Device=0x00;Remote=0x10)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "InternetTVBrowser");
                assert_eq!(result.os, "DigitalTV");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(
            r#"Mozilla/5.0 (DTV; TVwithVideoPlayer) NetFront/4.1 InettvBrowser/2.2 (08001F;DTV04VSFC3;0001;0001)"#,
        ) {
            None => panic!(
                r#"invalid parse. "Mozilla/5.0 (DTV; TVwithVideoPlayer) NetFront/4.1 InettvBrowser/2.2 (08001F;DTV04VSFC3;0001;0001)""#
            ),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "InternetTVBrowser");
                assert_eq!(result.os, "DigitalTV");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (DTV; TSBNetTV/T45000006.0203.CDD; like Gecko) NetFront/3.4 DTVNetBrowser/2.2 (000039;T45011C06;0203;CDD) InettvBrowser/2.2 (000039;T45011C06;0203;CDD)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (DTV; TSBNetTV/T45000006.0203.CDD; like Gecko) NetFront/3.4 DTVNetBrowser/2.2 (000039;T45011C06;0203;CDD) InettvBrowser/2.2 (000039;T45011C06;0203;CDD)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "InternetTVBrowser");
                assert_eq!(result.os, "DigitalTV");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Standard; NF34SW/1.1; like Gecko) NetFront/3.4 InettvBrowser/2.2C (000087;IP03-01;0100;0000)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Standard; NF34SW/1.1; like Gecko) NetFront/3.4 InettvBrowser/2.2C (000087;IP03-01;0100;0000)""#),
            Some(result) => {
                assert_eq!(result.category, "appliance");
                assert_eq!(result.name, "InternetTVBrowser");
                assert_eq!(result.os, "DigitalTV");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN");
            }
        }
    }
}
