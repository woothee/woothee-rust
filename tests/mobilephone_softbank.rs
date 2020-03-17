// This file is auto-generated! Any changes to this file will be lost!

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_mobilephone_softbank() {
        let parser = Parser::new();

        match parser.parse(r#"SoftBank/1.0/841SH/SHJ001/SN000000000000000 Browser/NetFront/3.5 Profile/MIDP-2.0 Configuration/CLDC-1.1"#) {
            None => panic!(r#"invalid parse. "SoftBank/1.0/841SH/SHJ001/SN000000000000000 Browser/NetFront/3.5 Profile/MIDP-2.0 Configuration/CLDC-1.1""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "SoftBank Mobile");
                assert_eq!(result.os, "SoftBank");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "841SH");
            }
        }
        match parser.parse(r#"SoftBank/2.0/004SH/SHJ001/SN000000000000000 Browser/NetFront/3.5 Profile/MIDP-2.0 Configuration/CLDC-1.1"#) {
            None => panic!(r#"invalid parse. "SoftBank/2.0/004SH/SHJ001/SN000000000000000 Browser/NetFront/3.5 Profile/MIDP-2.0 Configuration/CLDC-1.1""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "SoftBank Mobile");
                assert_eq!(result.os, "SoftBank");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "004SH");
            }
        }
        match parser.parse(r#"SoftBank/2.0/944SH/SHJ001/SN000000000000000 Browser/NetFront/3.5 Profile/MIDP-2.0 Configuration/CLDC-1.1"#) {
            None => panic!(r#"invalid parse. "SoftBank/2.0/944SH/SHJ001/SN000000000000000 Browser/NetFront/3.5 Profile/MIDP-2.0 Configuration/CLDC-1.1""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "SoftBank Mobile");
                assert_eq!(result.os, "SoftBank");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "944SH");
            }
        }
        match parser.parse(r#"SoftBank/1.0/821SC/SCJ001/SN000000000000000 Flash/Flash-Lite/2.0"#) {
            None => panic!(r#"invalid parse. "SoftBank/1.0/821SC/SCJ001/SN000000000000000 Flash/Flash-Lite/2.0""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "SoftBank Mobile");
                assert_eq!(result.os, "SoftBank");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "821SC");
            }
        }
        match parser
            .parse(r#"Vodafone/1.0/V905SH/SHJ001 Browser/VF-NetFront/3.3 Profile/MIDP-2.0 Configuration/CLDC-1.1"#)
        {
            None => panic!(
                r#"invalid parse. "Vodafone/1.0/V905SH/SHJ001 Browser/VF-NetFront/3.3 Profile/MIDP-2.0 Configuration/CLDC-1.1""#
            ),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "SoftBank Mobile");
                assert_eq!(result.os, "SoftBank");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "V905SH");
            }
        }
        match parser.parse(r#"Vodafone/1.0/V804SH/SHJ001/SN0000000000000 Browser/UP.Browser/7.0.2.1 Profile/MIDP-2.0 Configuration/CLDC-1.1 Ext-J-Profile/JSCL-1.3.2 Ext-V-Profile/VSCL-2.0.0"#) {
            None => panic!(r#"invalid parse. "Vodafone/1.0/V804SH/SHJ001/SN0000000000000 Browser/UP.Browser/7.0.2.1 Profile/MIDP-2.0 Configuration/CLDC-1.1 Ext-J-Profile/JSCL-1.3.2 Ext-V-Profile/VSCL-2.0.0""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "SoftBank Mobile");
                assert_eq!(result.os, "SoftBank");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "V804SH");
            }
        }
        match parser.parse(r#"J-PHONE/4.0/J-T51/SNXXXXXXXXX000000000 TS/1.00 Profile/MIDP-1.0 Configuration/CLDC-1.0 Ext-Profile/JSCL-1.1.0"#) {
            None => panic!(r#"invalid parse. "J-PHONE/4.0/J-T51/SNXXXXXXXXX000000000 TS/1.00 Profile/MIDP-1.0 Configuration/CLDC-1.0 Ext-Profile/JSCL-1.1.0""#),
            Some(result) => {
                assert_eq!(result.category, "mobilephone");
                assert_eq!(result.name, "SoftBank Mobile");
                assert_eq!(result.os, "SoftBank");
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "J-T51");
            }
        }
    }
}
