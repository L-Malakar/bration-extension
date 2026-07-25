// Bration-Extinction core engine
// Thin WASM wrapper around Brave's adblock-rust, exposing simple functions
// that a browser extension's JavaScript can call directly.
//
// Provides two kinds of blocking:
//  1. Network blocking (shouldBlock) — should this request be stopped before it loads?
//  2. Cosmetic filtering (getCosmeticSelectors) — CSS selectors for ad containers
//     that should be hidden on the page even if their content already loaded
//     (e.g. reserved ad slots, sponsored banners injected as first-party content).

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
    /// Create a new engine instance from raw filter list text.
    /// Multiple lists can be concatenated together before being passed in
    /// (e.g. EasyList + EasyPrivacy + a YouTube-specific list).
    #[wasm_bindgen(constructor)]
    pub fn new(filter_list_text: &str) -> AdblockEngine {
        console_error_panic_hook::set_once();
        let engine = build_engine(filter_list_text);
        AdblockEngine { engine }
    }

    /// Check whether a given request URL should be blocked at the network level.
    #[wasm_bindgen(js_name = shouldBlock)]
    pub fn should_block(&self, url: &str, source_url: &str, request_type: &str) -> bool {
        match Request::new(url, source_url, request_type) {
            Ok(request) => self.engine.check_network_request(&request).matched,
            Err(_) => false,
        }
    }

    /// Get CSS selectors for elements that should be hidden on this page
    /// (ad containers, sponsored banners, etc. that aren't blockable at the
    /// network level because they're first-party content).
    /// Returns a JSON array of selector strings, e.g. ["#ad-slot", ".sponsored"].
    #[wasm_bindgen(js_name = getCosmeticSelectors)]
    pub fn get_cosmetic_selectors(&self, url: &str) -> String {
        let resources = self.engine.url_cosmetic_resources(url);
        let selectors: Vec<String> = resources.hide_selectors.into_iter().collect();
        serde_json::to_string(&selectors).unwrap_or_else(|_| "[]".to_string())
    }

    /// Reload the engine with fresh filter list text (for periodic updates).
    #[wasm_bindgen(js_name = updateFilterList)]
    pub fn update_filter_list(&mut self, filter_list_text: &str) {
        self.engine = build_engine(filter_list_text);
    }
}

fn build_engine(filter_list_text: &str) -> Engine {
    let rules: Vec<String> = filter_list_text
        .lines()
        .map(|line| line.to_string())
        .collect();
    let mut filter_set = FilterSet::new(false);
    filter_set.add_filters(&rules, ParseOptions::default());
    Engine::from_filter_set(filter_set, true)
}