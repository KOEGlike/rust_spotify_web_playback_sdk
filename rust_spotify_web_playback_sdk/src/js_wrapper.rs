use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/wrapper.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn init(oauth: &Closure<dyn FnMut() -> String>, on_ready: &Closure<dyn FnMut()>, name: String, volume: f32, enable_media_session: bool);
    
    /// Check if the player object is ready
    #[wasm_bindgen]
    pub fn player_ready() -> bool;

    /// Log the player object to the console
    #[wasm_bindgen]
    pub fn log_player();

    #[wasm_bindgen(js_name = connect)]
    pub fn connect() -> Promise;

    #[wasm_bindgen(js_name = disconnect)]
    pub fn disconnect();

    #[wasm_bindgen(js_name = addListener)]
    pub fn addListener(event: String, callback: &Closure<dyn FnMut(JsValue)>) -> bool;

    #[wasm_bindgen(js_name = addListenerAutoplayFailed)]
    pub fn addListenerAutoplayFailed(event: String, callback: &Closure<dyn FnMut()>) -> bool;

    #[wasm_bindgen(js_name = removeListener)]
    pub fn removeListener(event: String) -> bool;

    #[wasm_bindgen(js_name = removeSpecificListener)]
    pub fn removeSpecificListener(event: String, callback: &Closure<dyn FnMut(JsValue)>) -> bool;

    #[wasm_bindgen(js_name = getCurrentState)]
    pub fn getCurrentState() -> Promise;

    #[wasm_bindgen(js_name = setName)]
    pub fn setName(name: String) -> Promise;

    #[wasm_bindgen(js_name = getVolume)]
    pub fn getVolume() -> Promise;

    #[wasm_bindgen(js_name = setVolume)]
    pub fn setVolume(volume: f32) -> Promise;

    #[wasm_bindgen(js_name = pause)]
    pub fn pause() -> Promise;

    #[wasm_bindgen(js_name = resume)]
    pub fn resume() -> Promise;

    #[wasm_bindgen(js_name = togglePlay)]
    pub fn togglePlay() -> Promise;

    #[wasm_bindgen(js_name = seek)]
    pub fn seek(position_ms: u32) -> Promise;

    #[wasm_bindgen(js_name = previousTrack)]
    pub fn previousTrack() -> Promise;

    #[wasm_bindgen(js_name = nextTrack)]
    pub fn nextTrack() -> Promise;

    #[wasm_bindgen(js_name = activateElement)]
    pub fn activateElement() -> Promise;
}