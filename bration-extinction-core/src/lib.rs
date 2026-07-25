// Bration-Extinction core engine
// Thin WASM wrapper around Brave's adblock-rust, exposing simple functions
// that a browser extension's JavaScript can call directly.

use adblock::lists::{FilterSet, ParseOptions};
use adblock::request::Request;
use adblock::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AdblockEngine {
    engine: Engine,
}

#[wasm_bindgen]
impl AdblockEngine {
    /// Create a new engine instance from raw filter list text
    /// (e.g. the contents of EasyList, downloaded as a plain string).
    #[wasm_bindgen(constructor)]
    pub fn new(filter_list_text: &str) -> AdblockEngine {
        console_error_panic_hook::set_once();
        let rules: Vec<String> = filter_list_text
            .lines()
            .map(|line| line.to_string())
            .collect();
        let mut filter_set = FilterSet::new(false);
        filter_set.add_filters(&rules, ParseOptions::default());
        let engine = Engine::from_filter_set(filter_set, true);
        AdblockEngine { engine }
    }

    /// Check whether a given request URL should be blocked.
    /// - url: the full request URL being loaded
    /// - source_url: the URL of the page making the request (for third-party detection)
    /// - request_type: e.g. "script", "image", "xmlhttprequest", "sub_frame"
    /// Returns true if the request should be blocked.
    #[wasm_bindgen(js_name = shouldBlock)]
    pub fn should_block(&self, url: &str, source_url: &str, request_type: &str) -> bool {
        match Request::new(url, source_url, request_type) {
            Ok(request) => self.engine.check_network_request(&request).matched,
            Err(_) => false,
        }
    }

    /// Reload the engine with a fresh filter list (for periodic filter list updates).
    #[wasm_bindgen(js_name = updateFilterList)]
    pub fn update_filter_list(&mut self, filter_list_text: &str) {
        let rules: Vec<String> = filter_list_text
            .lines()
            .map(|line| line.to_string())
            .collect();
        let mut filter_set = FilterSet::new(false);
        filter_set.add_filters(&rules, ParseOptions::default());
        self.engine = Engine::from_filter_set(filter_set, true);
    }
}