use std::{collections::HashMap, fs};

use reqwest::header::{HeaderMap, HeaderName, ACCEPT_ENCODING, TE};
use reqwest::header::{
    ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, ORIGIN, PRAGMA, REFERER, USER_AGENT,
};

use once_cell::sync::Lazy;

pub static HEADER_MANAGER: Lazy<HeaderManager> = Lazy::new(|| HeaderManager::load());

pub struct HeaderManager {
    headers: HashMap<String, String>,
}

//header manager requires a  well configured headers.json file in the root directory of the project
impl HeaderManager {
    pub fn load() -> Self {
        let contents = fs::read_to_string("./headers.json").expect(
            "💀 http header request file not found. expected headers. to be in .../warheads/headers.json",
        );

        let headers: HashMap<String, String> = serde_json::from_str(&contents).expect("💀 failed to parse headers.json as json. check the configuration in .../warheads/headers.json");

        assert!(!headers.is_empty(), "💀 headers.json is empty");
        assert!(
            headers.contains_key("User-Agent"),
            "💀 headers.json is missing User-Agent"
        );
        assert!(
            headers.contains_key("Ocp-Apim-Subscription-Key"),
            "💀 headers.json is missing User-Agent"
        );
        assert!(
            headers.contains_key("x-postal-code"),
            "💀 headers.json is missing User-Agent"
        );

        Self { headers }
    }

    pub fn history_request_headers(&self) -> HeaderMap {
        let mut headers = HeaderManager::default();
        headers.insert(ACCEPT, ("*/*").parse().unwrap());
        headers.insert(
            ACCEPT_LANGUAGE,
            ("en-US,en;q=0.9,de;q=0.8").parse().unwrap(),
        );

        headers.insert(
            USER_AGENT,
            self.headers.get("User-Agent").unwrap().parse().unwrap(),
        );

        headers
    }

    pub fn gamecard_request_headers(&self) -> HeaderMap {
        let mut headers = HeaderManager::default();
        headers.insert(ACCEPT, ("application/json").parse().unwrap());
        headers.insert(ACCEPT_LANGUAGE, ("en-US,en;q=0.5").parse().unwrap());

        headers.insert(
            ACCEPT_ENCODING,
            ("gzip, deflate, br, zstd").parse().unwrap(),
        );

        headers.insert(TE, ("trailers").parse().unwrap());

        headers.insert(
            USER_AGENT,
            self.headers.get("User-Agent").unwrap().parse().unwrap(),
        );

        headers.insert(
            "Ocp-Apim-Subscription-Key".parse::<HeaderName>().unwrap(),
            self.headers
                .get("Ocp-Apim-Subscription-Key")
                .unwrap()
                .parse()
                .unwrap(),
        );

        headers.insert(
            "x-postal-code".parse::<HeaderName>().unwrap(),
            self.headers.get("x-postal-code").unwrap().parse().unwrap(),
        );

        headers
    }

    fn default() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(CONNECTION, ("keep-alive").parse().unwrap());
        headers.insert(ORIGIN, ("https://www.nba.com").parse().unwrap());
        headers.insert(REFERER, ("https://www.nba.com").parse().unwrap());
        headers.insert(CACHE_CONTROL, ("no-cache").parse().unwrap());
        headers.insert(PRAGMA, ("no-cache").parse().unwrap());
        headers.insert(
            "Sec-Fetch-Dest".parse::<HeaderName>().unwrap(),
            ("empty").parse().unwrap(),
        );
        headers.insert(
            "Sec-Fetch-Mode".parse::<HeaderName>().unwrap(),
            ("cors").parse().unwrap(),
        );
        headers.insert(
            "Sec-Fetch-Site".parse::<HeaderName>().unwrap(),
            ("same-site").parse().unwrap(),
        );
        headers
    }
}

#[cfg(test)]
mod test_header_manager {
    use crate::constants::header_manager::HeaderManager;

    #[test]
    fn are_headers_present() {
        let manager = HeaderManager::load();

        assert!(true);
    }
}
