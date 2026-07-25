// Bration-Extinction core engine
// Thin WASM wrapper around Brave's adblock-rust, exposing simple functions
// that a browser extension's JavaScript can call directly.

use adblock::engine::Engine;
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
        let engine = Engine::from_rules(&rules, Default::default());
        AdblockEngine { engine }
    }

    /// Check whether a given request URL should be blocked.
    /// - url: the full request URL being loaded
    /// - source_url: the URL of the page making the request (for third-party detection)
    /// - request_type: e.g. "script", "image", "xmlhttprequest", "sub_frame"
    /// Returns true if the request should be blocked.
    #[wasm_bindgen(js_name = shouldBlock)]
    pub fn should_block(&self, url: &str, source_url: &str, request_type: &str) -> bool {
        let result = self.engine.check_network_urls(url, source_url, request_type);
        result.matched
    }

    /// Reload the engine with a fresh filter list (for periodic filter list updates).
    #[wasm_bindgen(js_name = updateFilterList)]
    pub fn update_filter_list(&mut self, filter_list_text: &str) {
        let rules: Vec<String> = filter_list_text
            .lines()
            .map(|line| line.to_string())
            .collect();
        self.engine = Engine::from_rules(&rules, Default::default());
    }
}
