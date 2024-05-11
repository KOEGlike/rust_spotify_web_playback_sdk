use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/wrapper.js")]
extern "C" {
    ///this function adds the script to the document, and creates an instance of the Spotify.Player class, if you don't call this function all the other functions will be useless
    #[wasm_bindgen]
    pub fn init(oauth:&Closure<dyn FnMut()->String>, on_ready:&Closure<dyn FnMut()>,name:String, volume:f32, enableMediaSession:bool);
    
    #[wasm_bindgen( js_name=connect, js_namespace=player)]
    pub fn connect() -> Promise;

    #[wasm_bindgen(js_name=disconnect, js_namespace=player)]
    pub fn disconnect();

    #[wasm_bindgen( js_name=addListener, js_namespace=player)]
    pub fn addListener(event: String, callback: &Closure<dyn FnMut(JsValue)>) -> bool;

    #[wasm_bindgen( js_name=addListener, js_namespace=player)]
    pub fn addListenerAutoplayFailed(event: String, callback: &Closure<dyn FnMut()>) -> bool;

    #[wasm_bindgen( js_name=removeListener, js_namespace=player)]
    pub fn removeListener(event: String) -> bool;

    #[wasm_bindgen( js_name=removeListener, js_namespace=player)]
    pub fn removeSpecificListener(event: String, callback: &Closure<dyn FnMut(JsValue)>) -> bool;

    #[wasm_bindgen( js_name=getCurrentState, js_namespace=player)]
    pub fn getCurrentState() -> Promise;

    #[wasm_bindgen( js_name=setName, js_namespace=player)]
    pub fn setName(name: String) -> Promise;

    #[wasm_bindgen( js_name=getVolume, js_namespace=player)]
    pub fn getVolume() -> Promise;

    #[wasm_bindgen(js_name=setVolume, js_namespace=player)]
    pub fn setVolume(volume: f32) -> Promise;

    #[wasm_bindgen( js_name=pause, js_namespace=player)]
    pub fn pause() -> Promise;

    #[wasm_bindgen( js_name=resume, js_namespace=player)]
    pub fn resume() -> Promise;

    #[wasm_bindgen( js_name=togglePlay, js_namespace=player)]
    pub fn togglePlay() -> Promise;

    #[wasm_bindgen( js_name=seek, js_namespace=player)]
    pub fn seek(position_ms: u32) -> Promise;

    #[wasm_bindgen( js_name=previousTrack, js_namespace=player)]
    pub fn previousTrack() -> Promise;

    #[wasm_bindgen( js_name=nextTrack, js_namespace=player)]
    pub fn nextTrack() -> Promise;

    #[wasm_bindgen( js_name=activateElement, js_namespace=player)]
    pub fn activateElement();

}
