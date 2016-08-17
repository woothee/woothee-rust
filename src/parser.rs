use std::collections::HashMap;
use regex::Regex;
use dataset;
use woothee::VALUE_UNKNOWN;

lazy_static! {
    static ref RX_CHROME_PATTERN: Regex = Regex::new(r"(?:Chrome|CrMo|CriOS)/([.0-9]+)").unwrap();
    static ref RX_DOCOMO_VERSION_PATTERN: Regex = Regex::new(r#"DoCoMo/[.0-9]+[ /]([^- /;()"']+)"#).unwrap();
    static ref RX_VIVALDI_PATTERN: Regex = Regex::new(r"Vivaldi/([.0-9]+)").unwrap();
    static ref RX_FIREFOX_PATTERN: Regex = Regex::new(r"Firefox/([.0-9]+)").unwrap();
    static ref RX_FIREFOX_OS_PATTERN: Regex = Regex::new(r"^Mozilla/[.0-9]+ \((?:Mobile|Tablet);(?:.*;)? rv:([.0-9]+)\) Gecko/[.0-9]+ Firefox/[.0-9]+$").unwrap();
    static ref RX_FIREFOX_IOS_PATTERN: Regex = Regex::new(r"FxiOS/([.0-9]+)").unwrap();
    static ref RX_FOMA_VERSION_PATTERN: Regex = Regex::new(r"\(([^;)]+);FOMA;").unwrap();
    static ref RX_JIG_PATTERN: Regex = Regex::new(r"jig browser[^;]+; ([^);]+)").unwrap();
    static ref RX_KDDI_PATTERN: Regex = Regex::new(r#"KDDI-([^- /;()"']+)"#).unwrap();
    static ref RX_MAYBE_RSS_PATTERN: Regex = Regex::new(r"(?i)rss(?:reader|bar|[-_ /;()]|[ +]*/)").unwrap();
    static ref RX_MAYBE_CRAWLER_PATTERN: Regex = Regex::new(r"(?i)(?:bot|crawler|spider)(?:[-_ ./;@()]|$)").unwrap();
    static ref RX_MAYBE_FEED_PARSER_PATTERN: Regex = Regex::new(r"(?i)(?:feed|web) ?parser").unwrap();
    static ref RX_MAYBE_WATCHDOG_PATTERN: Regex = Regex::new(r"(?i)watch ?dog").unwrap();
    static ref RX_MSEDGE_PATTERN: Regex = Regex::new(r"Edge/([.0-9]+)").unwrap();
    static ref RX_MSIE_PATTERN: Regex = Regex::new(r"MSIE ([.0-9]+);").unwrap();
    static ref RX_OPERA_VERSION_PATTERN1: Regex = Regex::new(r"Version/([.0-9]+)").unwrap();
    static ref RX_OPERA_VERSION_PATTERN2: Regex = Regex::new(r"Opera[/ ]([.0-9]+)").unwrap();
    static ref RX_OPERA_VERSION_PATTERN3: Regex = Regex::new(r"OPR/([.0-9]+)").unwrap();
    static ref RX_SAFARI_PATTERN: Regex = Regex::new(r"Version/([.0-9]+)").unwrap();
    static ref RX_SOFTBANK_PATTERN: Regex = Regex::new(r"(?:SoftBank|Vodafone|J-PHONE)/[.0-9]+/([^ /;()]+)").unwrap();
    static ref RX_TRIDENT_PATTERN: Regex = Regex::new(r"Trident/([.0-9]+);").unwrap();
    static ref RX_TRIDENT_VERSION_PATTERN: Regex = Regex::new(r" rv:([.0-9]+)").unwrap();
    static ref RX_IEMOBILE_PATTERN: Regex = Regex::new(r"IEMobile/([.0-9]+);").unwrap();
    static ref RX_WILLCOM_PATTERN: Regex = Regex::new(r"(?:WILLCOM|DDIPOCKET);[^/]+/([^ /;()]+)").unwrap();
    static ref RX_WINDOWS_VERSION_PATTERN: Regex = Regex::new(r"Windows ([ .a-zA-Z0-9]+)[;\\)]").unwrap();
    static ref RX_WIN_PHONE: Regex = Regex::new(r"^Phone(?: OS)? ([.0-9]+)").unwrap();
    static ref RX_WEBVIEW_PATTERN: Regex = Regex::new(r"iP(hone;|ad;|od) .*like Mac OS X").unwrap();
    static ref RX_WEBVIEW_VERSION_PATTERN: Regex = Regex::new(r"Version/([.0-9]+)").unwrap();
    static ref RX_PPC_OS_VERSION: Regex = Regex::new(r"rv:(\d+\.\d+\.\d+)").unwrap();
    static ref RX_FREEBSD_OS_VERSION: Regex = Regex::new(r"FreeBSD ([^;\)]+);").unwrap();
    static ref RX_CHROMEOS_OS_VERSION: Regex = Regex::new(r"CrOS ([^\)]+)\)").unwrap();
    static ref RX_ANDROIDOS_OS_VERSION: Regex = Regex::new(r"Android[- ](\d+\.\d+(?:\.\d+)?)").unwrap();
    static ref RX_PSP_OS_VERSION: Regex = Regex::new(r"PSP \(PlayStation Portable\); ([.0-9]+)\)").unwrap();
    static ref RX_PS3_OS_VERSION: Regex = Regex::new(r"PLAYSTATION 3;? ([.0-9]+)\)").unwrap();
    static ref RX_PSVITA_OS_VERSION: Regex = Regex::new(r"PlayStation Vita ([.0-9]+)\)").unwrap();
    static ref RX_PS4_OS_VERSION: Regex = Regex::new(r"PlayStation 4 ([.0-9]+)\)").unwrap();
    static ref RX_BLACKBERRY10_OS_VERSION: Regex = Regex::new(r"BB10(?:.+)Version/([.0-9]+) ").unwrap();
    static ref RX_BLACKBERRY_OS_VERSION: Regex = Regex::new(r"BlackBerry(?:\d+)/([.0-9]+) ").unwrap();

    static ref RE_OSX_IPHONE_OS_VERSION: Regex = Regex::new(r"; CPU(?: iPhone)? OS (\d+_\d+(?:_\d+)?) like Mac OS X").unwrap();
    static ref RE_OSX_OS_VERSION: Regex = Regex::new(r"Mac OS X (10[._]\d+(?:[._]\d+)?)(?:\)|;)").unwrap();
    static ref RX_HTTP_CLIENT: Regex = Regex::new(r"^(?:Apache-HttpClient/|Jakarta Commons-HttpClient/|Java/)").unwrap();
    static ref RX_HTTP_CLIENT_OTHER: Regex = Regex::new(r"[- ]HttpClient(/|$)").unwrap();
    static ref RX_PHP: Regex = Regex::new(r"^(?:PHP|WordPress|CakePHP|PukiWiki|PECL::HTTP)(?:/| |$)").unwrap();
    static ref RX_PEAR: Regex = Regex::new(r"(?:PEAR HTTP_Request|HTTP_Request)(?: class|2)").unwrap();
    static ref RX_MAYBE_CRAWLER_OTHER: Regex = Regex::new(r"(?:Rome Client |UnwindFetchor/|ia_archiver |Summify |PostRank/)").unwrap();
}

#[derive(Debug, Default)]
pub struct WootheeResult<'a> {
    pub name: &'a str,
    pub category: &'a str,
    pub os: &'a str,
    pub os_version: String,
    pub browser_type: &'a str,
    pub version: String,
    pub vendor: &'a str,
}

impl<'a> WootheeResult<'a> {
    pub fn new() -> WootheeResult<'a> {
        WootheeResult {
            name: VALUE_UNKNOWN,
            category: VALUE_UNKNOWN,
            os: VALUE_UNKNOWN,
            os_version: VALUE_UNKNOWN.to_string(),
            browser_type: VALUE_UNKNOWN,
            version: VALUE_UNKNOWN.to_string(),
            vendor: VALUE_UNKNOWN,
        }
    }

    fn populate_with(&mut self, ds: &WootheeResult<'a>) {
        if !ds.name.is_empty() {
            self.name = ds.name;
        }

        if !ds.category.is_empty() {
            self.category = ds.category.clone();
        }

        if !ds.os.is_empty() {
            self.os = ds.os.clone();
        }

        if !ds.browser_type.is_empty() {
            self.browser_type = ds.browser_type.clone();
        }

        if !ds.version.is_empty() {
            self.version = ds.version.clone();
        }

        if !ds.vendor.is_empty() {
            self.vendor = ds.vendor.clone();
        }
    }
}

#[derive(Default)]
pub struct Parser<'a> {
    agent_dataset: HashMap<&'a str, WootheeResult<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser { agent_dataset: dataset::get_default_dataset() }
    }

    pub fn parse(&self, agent: &str) -> Option<WootheeResult> {
        let mut result = WootheeResult::new();
        if agent == "" || agent == "-" {
            return Some(result);
        }

        if self.try_crawler(agent, &mut result) {
            return Some(result);
        }

        if self.try_browser(agent, &mut result) {
            self.try_os(agent, &mut result);
            return Some(result);
        }

        if self.try_mobilephone(agent, &mut result) {
            return Some(result);
        }

        if self.try_appliance(agent, &mut result) {
            return Some(result);
        }

        if self.try_misc(agent, &mut result) {
            return Some(result);
        }

        if self.try_os(agent, &mut result) {
            return Some(result);
        }

        if self.try_rare_cases(agent, &mut result) {
            return Some(result);
        }

        None
    }

    fn populate_dataset<'b>(&'b self, result: &mut WootheeResult<'b>, label: &str) -> bool {
        match self.lookup_dataset(label) {
            Some(ds) => {
                result.populate_with(ds);
                true
            }
            None => false,
        }
    }

    fn lookup_dataset(&self, label: &str) -> Option<&WootheeResult> {
        self.agent_dataset.get(label)
    }

    pub fn try_crawler<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if self.challenge_google(agent, result) {
            return true;
        }

        if self.challenge_crawlers(agent, result) {
            return true;
        }

        false
    }

    fn try_os<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if self.challenge_windows(agent, result) {
            return true;
        }

        if self.challenge_osx(agent, result) {
            return true;
        }

        if self.challenge_linux(agent, result) {
            return true;
        }

        if self.challenge_smartphone(agent, result) {
            return true;
        }

        if self.challenge_mobilephone(agent, result) {
            return true;
        }

        if self.challenge_appliance(agent, result) {
            return true;
        }

        if self.challenge_misc_os(agent, result) {
            return true;
        }

        false
    }

    fn try_browser<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if self.challenge_msie(agent, result) {
            return true;
        }

        if self.challenge_ms_edge(agent, result) {
            return true;
        }

        if self.challenge_vivaldi(agent, result) {
            return true;
        }

        if self.challenge_firefox_ios(agent, result) {
            return true;
        }

        if self.challenge_safari_chrome(agent, result) {
            return true;
        }

        if self.challenge_firefox(agent, result) {
            return true;
        }

        if self.challenge_opera(agent, result) {
            return true;
        }

        if self.challenge_webview(agent, result) {
            return true;
        }

        false
    }

    fn try_mobilephone<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if self.challenge_docomo(agent, result) {
            return true;
        }

        if self.challenge_au(agent, result) {
            return true;
        }

        if self.challenge_softbank(agent, result) {
            return true;
        }

        if self.challenge_willcom(agent, result) {
            return true;
        }

        if self.challenge_misc_mobilephone(agent, result) {
            return true;
        }

        false
    }

    fn try_appliance<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if self.challenge_playstation(agent, result) {
            return true;
        }

        if self.challenge_nintendo(agent, result) {
            return true;
        }

        if self.challenge_digital_tv(agent, result) {
            return true;
        }

        false
    }
    fn try_misc<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if self.challenge_desktop_tools(agent, result) {
            return true;
        }

        false
    }

    fn try_rare_cases<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if self.challenge_smartphone_patterns(agent, result) {
            return true;
        }

        if self.challenge_sleipnir(agent, result) {
            return true;
        }

        if self.challenge_http_library(agent, result) {
            return true;
        }

        if self.challenge_maybe_rss_reader(agent, result) {
            return true;
        }

        if self.challenge_maybe_crawler(agent, result) {
            return true;
        }

        false
    }

    fn challenge_google<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Google") {
            return false;
        }

        if agent.contains("compatible; Googlebot") {
            if agent.contains("compatible; Googlebot-Mobile") {
                return self.populate_dataset(result, "GoogleBotMobile");
            }
            return self.populate_dataset(result, "GoogleBot");
        }

        if agent.contains("Googlebot-Image/") {
            return self.populate_dataset(result, "GoogleBot");
        }

        if agent.contains("Mediapartners-Google") &&
           (agent.contains("compatible; Mediapartners-Google") || agent == "Mediapartners-Google") {
            return self.populate_dataset(result, "GoogleMediaPartners");
        }

        if agent.contains("Feedfetcher-Google;") {
            return self.populate_dataset(result, "GoogleFeedFetcher");
        }

        if agent.contains("AppEngine-Google") {
            return self.populate_dataset(result, "GoogleAppEngine");
        }

        if agent.contains("Google Web Preview") {
            return self.populate_dataset(result, "GoogleWebPreview");
        }

        false
    }

    fn challenge_crawlers<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if agent.contains("Yahoo") || agent.contains("help.yahoo.co.jp/help/jp/") ||
           agent.contains("listing.yahoo.co.jp/support/faq/") {
            if agent.contains("compatible; Yahoo! Slurp") {
                return self.populate_dataset(result, "YahooSlurp");
            } else if agent.contains("YahooFeedSeekerJp") || agent.contains("YahooFeedSeekerBetaJp") ||
               agent.contains("crawler (http://listing.yahoo.co.jp/support/faq/") ||
               agent.contains("crawler (http://help.yahoo.co.jp/help/jp/") ||
               agent.contains("Y!J-BRZ/YATSHA crawler") ||
               agent.contains("Y!J-BRY/YATSH crawler") {
                return self.populate_dataset(result, "YahooJP");
            } else if agent.contains("Yahoo Pipes") {
                return self.populate_dataset(result, "YahooPipes");
            }
        }

        if agent.contains("msnbot") {
            return self.populate_dataset(result, "msnbot");
        }

        if agent.contains("bingbot") && agent.contains("compatible; bingbot") {
            return self.populate_dataset(result, "bingbot");
        }

        if agent.contains("BingPreview") {
            return self.populate_dataset(result, "BingPreview")
        }

        if agent.contains("Baidu") &&
           (agent.contains("compatible; Baiduspider") || agent.contains("Baiduspider+") ||
            agent.contains("Baiduspider-image+")) {
            return self.populate_dataset(result, "Baiduspider");
        }

        if agent.contains("Yeti") && agent.contains("http://help.naver.com/robots") {
            return self.populate_dataset(result, "Yeti");
        }

        if agent.contains("FeedBurner/") {
            return self.populate_dataset(result, "FeedBurner");
        }

        if agent.contains("facebookexternalhit") {
            return self.populate_dataset(result, "facebook");
        }

        if agent.contains("Twitterbot/") {
            return self.populate_dataset(result, "twitter");
        }

        if agent.contains("ichiro") &&
           (agent.contains("http://help.goo.ne.jp/door/crawler.html") ||
            agent.contains("compatible; ichiro/mobile goo;")) {
            return self.populate_dataset(result, "goo");
        }

        if agent.contains("gooblogsearch/") {
            return self.populate_dataset(result, "goo");
        }

        if agent.contains("Apple-PubSub") {
            return self.populate_dataset(result, "ApplePubSub");
        }

        if agent.contains("(www.radian6.com/crawler)") {
            return self.populate_dataset(result, "radian6");
        }

        if agent.contains("Genieo/") {
            return self.populate_dataset(result, "Genieo");
        }

        if agent.contains("labs.topsy.com/butterfly/") {
            return self.populate_dataset(result, "topsyButterfly");
        }

        if agent.contains("rogerbot/1.0 (http://www.seomoz.org/dp/rogerbot") {
            return self.populate_dataset(result, "rogerbot");
        }

        if agent.contains("compatible; AhrefsBot/") {
            return self.populate_dataset(result, "AhrefsBot");
        }

        if agent.contains("livedoor FeedFetcher") || agent.contains("Fastladder FeedFetcher") {
            return self.populate_dataset(result, "livedoorFeedFetcher");
        }

        if agent.contains("Hatena Antenna") || agent.contains("Hatena Pagetitle Agent") ||
           agent.contains("Hatena Diary RSS") {
            return self.populate_dataset(result, "Hatena");
        }

        if agent.contains("mixi-check") || agent.contains("mixi-crawler") ||
           agent.contains("mixi-news-crawler") {
            return self.populate_dataset(result, "mixi");
        }

        if agent.contains("Indy Library") && agent.contains("compatible; Indy Library") {
            return self.populate_dataset(result, "IndyLibrary");
        }

        false
    }

    fn challenge_msie<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("compatible; MSIE") && !agent.contains("Trident/") &&
           !agent.contains("IEMobile") {
            return false;
        }

        let mut version = VALUE_UNKNOWN;
        let re_msie_caps = RX_MSIE_PATTERN.captures(agent);
        let re_trident_caps = RX_TRIDENT_PATTERN.captures(agent);
        let re_trident_ver_caps = RX_TRIDENT_VERSION_PATTERN.captures(agent);
        let re_ie_mobile_caps = RX_IEMOBILE_PATTERN.captures(agent);

        if re_msie_caps.is_some() {
            version = re_msie_caps.unwrap().at(1).unwrap();
        } else if re_trident_caps.is_some() && re_trident_ver_caps.is_some() {
            version = re_trident_ver_caps.unwrap().at(1).unwrap();
        } else if re_ie_mobile_caps.is_some() {
            version = re_ie_mobile_caps.unwrap().at(1).unwrap();
        }

        if !self.populate_dataset(result, "MSIE") {
            return false;
        }

        result.version = version.to_string();

        true
    }

    fn challenge_ms_edge<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !RX_MSEDGE_PATTERN.is_match(agent) {
            return false;
        };

        if !self.populate_dataset(result, "Edge") {
            return false;
        }

        true
    }

    fn challenge_vivaldi<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        match RX_VIVALDI_PATTERN.captures(agent) {
            Some(caps) => {
                result.version = caps.at(1).unwrap().to_string();
            }
            None => {
                return false;
            }
        };

        if !self.populate_dataset(result, "Vivaldi") {
            return false;
        }

        true
    }

    fn challenge_firefox_ios<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        match RX_FIREFOX_IOS_PATTERN.captures(agent) {
            Some(caps) => {
                result.version = caps.at(1).unwrap().to_string();
            }
            None => {
                return false;
            }
        };

        if !self.populate_dataset(result, "Firefox") {
            return false;
        }

        true
    }

    fn challenge_safari_chrome<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Safari/") {
            return false;
        }

        if RX_CHROME_PATTERN.is_match(agent) {
            if RX_OPERA_VERSION_PATTERN3.is_match(agent) {
                let version = match RX_OPERA_VERSION_PATTERN3.captures(agent) {
                    Some(caps) => caps.at(1).unwrap(),
                    None => "",
                };
                if !self.populate_dataset(result, "Opera") {
                    return false;
                }
                if !version.is_empty() {
                    result.version = version.to_string();
                }
                return true;
            }

            if !self.populate_dataset(result, "Chrome") {
                return false;
            }

            let version = match RX_CHROME_PATTERN.captures(agent) {
                Some(caps) => caps.at(1).unwrap(),
                None => "",
            };
            if !version.is_empty() {
                result.version = version.to_string();
            }
            return true;
        }

        let version = match RX_SAFARI_PATTERN.captures(agent) {
            Some(caps) => caps.at(1).unwrap(),
            None => VALUE_UNKNOWN,
        };

        if !self.populate_dataset(result, "Safari") {
            return false;
        }

        result.version = version.to_string();

        true
    }

    fn challenge_firefox<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Firefox/") {
            return false;
        }

        let version = match RX_FIREFOX_PATTERN.captures(agent) {
            Some(caps) => caps.at(1).unwrap(),
            None => VALUE_UNKNOWN,
        };

        if !self.populate_dataset(result, "Firefox") {
            return false;
        }

        result.version = version.to_string();

        true
    }

    fn challenge_opera<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Opera") {
            return false;
        }

        let version = match RX_OPERA_VERSION_PATTERN1.captures(agent) {
            Some(caps) => caps.at(1).unwrap(),
            None => {
                match RX_OPERA_VERSION_PATTERN2.captures(agent) {
                    Some(caps2) => caps2.at(1).unwrap(),
                    None => VALUE_UNKNOWN,
                }
            }
        };

        if !self.populate_dataset(result, "Opera") {
            return false;
        }

        result.version = version.to_string();

        true
    }

    fn challenge_webview<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !RX_WEBVIEW_PATTERN.is_match(agent) || agent.contains("Safari/") {
            return false;
        }

        if !self.populate_dataset(result, "Webview") {
            return false;
        }

        let version = match RX_WEBVIEW_VERSION_PATTERN.captures(agent) {
            Some(caps) => caps.at(1).unwrap(),
            None => "",
        };
        if !version.is_empty() {
            result.version = version.to_string();
        }

        true
    }

    fn challenge_docomo<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("DoCoMo") && !agent.contains(";FOMA;") {
            return false;
        }

        let mut version = VALUE_UNKNOWN;
        let docomo_caps = RX_DOCOMO_VERSION_PATTERN.captures(agent);
        let foma_caps = RX_FOMA_VERSION_PATTERN.captures(agent);
        if docomo_caps.is_some() {
            version = docomo_caps.unwrap().at(1).unwrap();
        } else if foma_caps.is_some() {
            version = foma_caps.unwrap().at(1).unwrap();
        }

        if !self.populate_dataset(result, "docomo") {
            return false;
        }
        result.version = version.to_string();

        true
    }

    fn challenge_au<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("KDDI-") {
            return false;
        }

        let mut version = VALUE_UNKNOWN;
        let caps = RX_KDDI_PATTERN.captures(agent);
        if caps.is_some() {
            version = caps.unwrap().at(1).unwrap();
        }

        if !self.populate_dataset(result, "au") {
            return false;
        }
        result.version = version.to_string();

        true
    }

    fn challenge_softbank<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("SoftBank") && !agent.contains("Vodafone") &&
           !agent.contains("J-PHONE") {
            return false;
        }

        let mut version = VALUE_UNKNOWN;
        let caps = RX_SOFTBANK_PATTERN.captures(agent);
        if caps.is_some() {
            version = caps.unwrap().at(1).unwrap();
        }

        if !self.populate_dataset(result, "SoftBank") {
            return false;
        }
        result.version = version.to_string();

        true
    }

    fn challenge_willcom<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("WILLCOM") && !agent.contains("DDIPOCKET") {
            return false;
        }

        let mut version = VALUE_UNKNOWN;
        let caps = RX_WILLCOM_PATTERN.captures(agent);
        if caps.is_some() {
            version = caps.unwrap().at(1).unwrap();
        }

        if !self.populate_dataset(result, "willcom") {
            return false;
        }
        result.version = version.to_string();

        true
    }

    fn challenge_misc_mobilephone<'b>(&'b self,
                                      agent: &str,
                                      result: &mut WootheeResult<'b>)
                                      -> bool {
        if agent.contains("jig browser") {
            if !self.populate_dataset(result, "jig") {
                return false;
            }

            let caps = RX_JIG_PATTERN.captures(agent);
            if caps.is_some() {
                result.version = caps.unwrap().at(1).unwrap().to_string();
            }
            return true;
        }

        if agent.contains("emobile/") || agent.contains("OpenBrowser") ||
           agent.contains("Browser/Obigo-Browser") {
            if !self.populate_dataset(result, "emobile") {
                return false;
            }
            return true;
        }

        if agent.contains("SymbianOS") {
            if !self.populate_dataset(result, "SymbianOS") {
                return false;
            }
            return true;
        }

        if agent.contains("Hatena-Mobile-Gateway/") {
            if !self.populate_dataset(result, "MobileTranscoder") {
                return false;
            }
            result.version = "Hatena".to_string();
            return true;
        }

        if agent.contains("livedoor-Mobile-Gateway/") {
            if !self.populate_dataset(result, "MobileTranscoder") {
                return false;
            }
            result.version = "livedoor".to_string();
            return true;
        }

        false
    }

    fn challenge_playstation<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        let mut os_version = "";

        let d = if agent.contains("PSP (PlayStation Portable)") {
            os_version = match RX_PSP_OS_VERSION.captures(agent) {
                Some(caps) => caps.at(1).unwrap(),
                None => "",
            };
            self.lookup_dataset("PSP")
        } else if agent.contains("PlayStation Vita") {
            os_version = match RX_PSVITA_OS_VERSION.captures(agent) {
                Some(caps) => caps.at(1).unwrap(),
                None => "",
            };
            self.lookup_dataset("PSVita")
        } else if agent.contains("PLAYSTATION 3 ") || agent.contains("PLAYSTATION 3;") {
            os_version = match RX_PS3_OS_VERSION.captures(agent) {
                Some(caps) => caps.at(1).unwrap(),
                None => "",
            };
            self.lookup_dataset("PS3")
        } else if agent.contains("PlayStation 4 ") {
            os_version = match RX_PS4_OS_VERSION.captures(agent) {
                Some(caps) => caps.at(1).unwrap(),
                None => "",
            };
            self.lookup_dataset("PS4")
        } else {
            None
        };

        if d.is_none() {
            return false;
        }
        let data = d.unwrap();

        result.populate_with(data);

        if !os_version.is_empty() {
            result.os_version = os_version.to_string();
        }

        true
    }

    fn challenge_nintendo<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if agent.contains("Nintendo 3DS;") {
            return self.populate_dataset(result, "Nintendo3DS");
        }

        if agent.contains("Nintendo DSi;") {
            return self.populate_dataset(result, "NintendoDSi");
        }

        if agent.contains("Nintendo Wii;") {
            return self.populate_dataset(result, "NintendoWii");
        }

        if agent.contains("(Nintendo WiiU)") {
            return self.populate_dataset(result, "NintendoWiiU");
        }

        false
    }

    fn challenge_digital_tv<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if agent.contains("InettvBrowser/") {
            return self.populate_dataset(result, "DigitalTV");
        }

        false
    }

    fn challenge_windows<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Windows") {
            return false;
        }

        if agent.contains("Xbox") {
            if agent.contains("Xbox; Xbox One)") {
                return self.populate_dataset(result, "XboxOne");
            }
            return self.populate_dataset(result, "Xbox360");
        }

        let mut w = self.lookup_dataset("Win");
        if w.is_none() {
            return false;
        }
        let mut win = w.unwrap();

        let caps = RX_WINDOWS_VERSION_PATTERN.captures(agent);
        if caps.is_none() {
            result.category = win.category.clone();
            result.os = win.name.clone();
            return true;
        }

        let mut version = caps.unwrap().at(1).unwrap();
        w = match version {
            "NT 10.0" => self.lookup_dataset("Win10"),
            "NT 6.3" => self.lookup_dataset("Win8.1"),
            "NT 6.2" => self.lookup_dataset("Win8"),
            "NT 6.1" => self.lookup_dataset("Win7"),
            "NT 6.0" => self.lookup_dataset("WinVista"),
            "NT 5.1" => self.lookup_dataset("WinXP"),
            "NT 5.0" => self.lookup_dataset("Win2000"),
            "NT 4.0" => self.lookup_dataset("WinNT4"),
            "98" => self.lookup_dataset("Win98"),
            "95" => self.lookup_dataset("Win95"),
            "CE" => self.lookup_dataset("WinCE"),
            _ => {
                let caps = RX_WIN_PHONE.captures(version);
                if caps.is_some() {
                    version = caps.unwrap().at(1).unwrap();
                    self.lookup_dataset("WinPhone")
                } else {
                    None
                }
            }
        };

        if w.is_none() {
            return false;
        }
        win = w.unwrap();

        result.category = win.category.clone();
        result.os = win.name.clone();
        if !version.is_empty() {
            result.os_version = version.to_string();
        }

        true
    }

    fn challenge_osx<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Mac OS X") {
            return false;
        }

        let mut d = self.lookup_dataset("OSX");
        if d.is_none() {
            return false;
        }
        let mut data = d.unwrap();
        let mut version = String::new();

        if agent.contains("like Mac OS X") {
            if agent.contains("iPhone;") {
                d = self.lookup_dataset("iPhone");
            } else if agent.contains("iPad;") {
                d = self.lookup_dataset("iPad");
            } else if agent.contains("iPod") {
                d = self.lookup_dataset("iPod");
            }
            if d.is_none() {
                return false;
            }
            data = d.unwrap();

            let caps = RE_OSX_IPHONE_OS_VERSION.captures(agent);
            if caps.is_some() {
                let v = caps.unwrap().at(1).unwrap();
                version = v.replace("_", ".");
            }
        } else {
            let caps = RE_OSX_OS_VERSION.captures(agent);
            if caps.is_some() {
                let v = caps.unwrap().at(1).unwrap();
                version = v.replace("_", ".");
            }
        }

        result.category = data.category.clone();
        result.os = data.name.clone();
        if !version.is_empty() {
            result.os_version = version.to_string();
        }

        true
    }

    fn challenge_linux<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Linux") {
            return false;
        }

        let mut os_version = String::new();
        let d = if agent.contains("Android") {
            let caps = RX_ANDROIDOS_OS_VERSION.captures(agent);
            if caps.is_some() {
                os_version = caps.unwrap().at(1).unwrap().to_string();
            }
            self.lookup_dataset("Android")
        } else {
            self.lookup_dataset("Linux")
        };

        if d.is_none() {
            return false;
        }
        let data = d.unwrap();

        result.category = data.category.clone();
        result.os = data.name.clone();
        if !os_version.is_empty() {
            result.os_version = os_version;
        }

        true
    }

    fn challenge_smartphone<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        let mut os_version = "";

        let mut d = if agent.contains("iPhone") {
            self.lookup_dataset("iPhone")
        } else if agent.contains("iPad") {
            self.lookup_dataset("iPad")
        } else if agent.contains("iPod") {
            self.lookup_dataset("iPod")
        } else if agent.contains("Android") {
            self.lookup_dataset("Android")
        } else if agent.contains("CFNetwork") {
            self.lookup_dataset("iOS")
        } else if agent.contains("BB10") {
            let caps = RX_BLACKBERRY10_OS_VERSION.captures(agent);
            if caps.is_some() {
                os_version = caps.unwrap().at(1).unwrap();
            }
            result.version = VALUE_UNKNOWN.to_string();
            self.lookup_dataset("BlackBerry10")
        } else if agent.contains("BlackBerry") {
            let caps = RX_BLACKBERRY_OS_VERSION.captures(agent);
            if caps.is_some() {
                os_version = caps.unwrap().at(1).unwrap();
            }
            self.lookup_dataset("BlackBerry")
        } else {
            None
        };

        let f = self.lookup_dataset("Firefox");
        if f.is_none() {
            return false;
        }
        let firefox = f.unwrap();

        if result.name == firefox.name {
            // Firefox OS specific pattern
            // http://lawrencemandel.com/2012/07/27/decision-made-firefox-os-user-agent-string/
            // https://github.com/woothee/woothee/issues/2
            let caps = RX_FIREFOX_OS_PATTERN.captures(agent);
            if caps.is_some() {
                let c = caps.unwrap();
                if c.len() > 1 {
                    d = self.lookup_dataset("FirefoxOS");
                    os_version = c.at(1).unwrap();
                }
            }
        }

        if d.is_none() {
            return false;
        }
        let data = d.unwrap();

        result.category = data.category.clone();
        result.os = data.name.clone();
        if !os_version.is_empty() {
            result.os_version = os_version.to_string();
        }

        true
    }

    fn challenge_mobilephone<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if agent.contains("KDDI-") {
            let caps = RX_KDDI_PATTERN.captures(agent);
            if caps.is_some() {
                let term = caps.unwrap().at(1).unwrap();
                let d = self.lookup_dataset("au");
                if d.is_none() {
                    return false;
                }
                let data = d.unwrap();
                result.category = data.category.clone();
                result.os = data.os.clone();
                result.version = term.to_string();
                return true;
            }
        }

        if agent.contains("WILLCOM") || agent.contains("DDIPOCKET") {
            let caps = RX_WILLCOM_PATTERN.captures(agent);
            if caps.is_some() {
                let term = caps.unwrap().at(1).unwrap();
                let d = self.lookup_dataset("willcom");
                if d.is_none() {
                    return false;
                }
                let data = d.unwrap();
                result.category = data.category.clone();
                result.os = data.os.clone();
                result.version = term.to_string();
                return true;
            }
        }

        if agent.contains("SymbianOS") {
            let d = self.lookup_dataset("SymbianOS");
            if d.is_none() {
                return false;
            }
            let data = d.unwrap();
            result.category = data.category.clone();
            result.os = data.os.clone();
            return true;
        }

        if agent.contains("Google Wireless Transcoder") {
            if !self.populate_dataset(result, "MobileTranscoder") {
                return false;
            }
            result.version = "Google".to_string();
        }

        if agent.contains("Naver Transcoder") {
            if !self.populate_dataset(result, "MobileTranscoder") {
                return false;
            }
            result.version = "Naver".to_string();
        }

        false
    }

    fn challenge_desktop_tools<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if agent.contains("AppleSyndication/") {
            return self.populate_dataset(result, "SafariRSSReader");
        }

        if agent.contains("compatible; Google Desktop/") {
            return self.populate_dataset(result, "GoogleDesktop");
        }

        if agent.contains("Windows-RSS-Platform") {
            return self.populate_dataset(result, "WindowsRSSReader");
        }

        false
    }

    fn challenge_smartphone_patterns<'b>(&'b self,
                                         agent: &str,
                                         result: &mut WootheeResult<'b>)
                                         -> bool {
        if agent.contains("CFNetwork/") {
            // This is like iPhone, but only Category and Name are filled
            let d = self.lookup_dataset("iOS");
            if d.is_none() {
                return false;
            }
            let data = d.unwrap();

            result.os = data.name.clone();
            result.category = data.category.clone();
            return true;
        }

        false
    }

    fn challenge_sleipnir<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if !agent.contains("Sleipnir/") {
            return false;
        }

        let version = match Regex::new(r"Sleipnir/([.0-9]+)").unwrap().captures(agent) {
            Some(caps) => caps.at(1).unwrap(),
            None => VALUE_UNKNOWN,
        };

        self.populate_dataset(result, "Sleipnir");

        let w = self.lookup_dataset("Win");
        if w.is_none() {
            return false;
        }
        let win = w.unwrap();

        result.version = version.to_string();
        result.category = win.category.clone();
        result.os = win.name.clone();

        true
    }

    fn challenge_http_library<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        // TODO: wip
        let mut version = "";

        if RX_HTTP_CLIENT.is_match(agent) || RX_HTTP_CLIENT_OTHER.is_match(agent) ||
           agent.contains("Java(TM) 2 Runtime Environment,") {
            version = "Java";
        } else if agent.starts_with("Wget/") {
            version = "wget";
        } else if agent.starts_with("libwww-perl") || agent.starts_with("WWW-Mechanize") ||
           agent.starts_with("LWP::Simple") || agent.starts_with("LWP ") ||
           agent.starts_with("lwp-trivial") {
            version = "perl";
        } else if agent.starts_with("Ruby") || agent.starts_with("feedzirra") ||
           agent.starts_with("Typhoeus") {
            version = "ruby"
        } else if agent.starts_with("Python-urllib/") || agent.starts_with("Twisted ") {
            version = "python";
        } else if RX_PHP.is_match(agent) || RX_PEAR.is_match(agent) {
            version = "php";
        }

        if version.is_empty() {
            return false;
        }

        if !self.populate_dataset(result, "HTTPLibrary") {
            return false;
        }
        result.version = version.to_string();

        true
    }

    fn challenge_maybe_rss_reader<'b>(&'b self,
                                      agent: &str,
                                      result: &mut WootheeResult<'b>)
                                      -> bool {
        if RX_MAYBE_RSS_PATTERN.is_match(agent) ||
           agent.to_lowercase().contains("headline-reader") || agent.contains("cococ/") {
            return self.populate_dataset(result, "VariousRSSReader");
        }

        false
    }

    fn challenge_maybe_crawler<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if RX_MAYBE_CRAWLER_PATTERN.is_match(agent) || RX_MAYBE_CRAWLER_OTHER.is_match(agent) ||
           agent.contains("ASP-Ranker Feed Crawler") ||
           RX_MAYBE_FEED_PARSER_PATTERN.is_match(agent) ||
           RX_MAYBE_WATCHDOG_PATTERN.is_match(agent) {
            return self.populate_dataset(result, "VariousCrawler");
        }

        false
    }

    fn challenge_appliance<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        if agent.contains("Nintendo DSi;") {
            let d = self.lookup_dataset("NintendoDSi");
            if d.is_none() {
                return false;
            }
            let data = d.unwrap();
            result.category = data.category.clone();
            result.os = data.os.clone();
            return true;
        }

        if agent.contains("Nintendo Wii;") {
            let d = self.lookup_dataset("NintendoWii");
            if d.is_none() {
                return false;
            }
            let data = d.unwrap();
            result.category = data.category.clone();
            result.os = data.os.clone();
            return true;
        }

        false
    }

    fn challenge_misc_os<'b>(&'b self, agent: &str, result: &mut WootheeResult<'b>) -> bool {
        let d = if agent.contains("(Win98;") {
            result.os_version = "98".to_string();
            self.lookup_dataset("Win98")
        } else if agent.contains("Macintosh; U; PPC;") || agent.contains("Mac_PowerPC") {
            let caps = RX_PPC_OS_VERSION.captures(agent);
            if caps.is_some() {
                result.os_version = caps.unwrap().at(1).unwrap().to_string();
            }
            self.lookup_dataset("MacOS")
        } else if agent.contains("X11; FreeBSD ") {
            let caps = RX_FREEBSD_OS_VERSION.captures(agent);
            if caps.is_some() {
                result.os_version = caps.unwrap().at(1).unwrap().to_string();
            }
            self.lookup_dataset("BSD")
        } else if agent.contains("X11; CrOS ") {
            let caps = RX_CHROMEOS_OS_VERSION.captures(agent);
            if caps.is_some() {
                result.os_version = caps.unwrap().at(1).unwrap().to_string();
            }
            self.lookup_dataset("ChromeOS")
        } else {
            None
        };

        if d.is_none() {
            return false;
        }
        let data = d.unwrap();

        result.category = data.category.clone();
        result.os = data.name.clone();

        true
    }
}

#[cfg(test)]
mod tests {
    use parser::Parser;

    #[test]
    fn test_parse_smoke() {
        let parser = Parser::new();
        match parser.parse("Mozilla/5.0 (Mobile; rv:18.0) Gecko/18.0 Firefox/18.0") {
            Some(result) => assert_eq!(result.category, "smartphone".to_string()),
            None => panic!("invalid"),
        }
    }
}
