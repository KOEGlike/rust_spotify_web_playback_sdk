use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

mod js_wrapper;
mod structs;
pub use js_wrapper::init;
pub use structs::*;

impl From<Events> for &str {
    fn from(val: Events) -> Self {
        match val {
            Events::Ready(_) => "ready",
            Events::NotReady(_) => "not_ready",
            Events::PlayerStateChanged(_) => "player_state_changed",
            Events::AutoplayFailed(_) => "autoplay_failed",
        }
    }
}

pub async fn connect() -> Result<bool, JsValue> {
    let promise = js_wrapper::connect();
    let result = JsFuture::from(promise).await?;
    match result.as_bool() {
        Some(b) => Ok(b),
        None => Err(result),
    }
}

pub fn disconnect() {
    js_wrapper::disconnect();
}

pub fn add_listener(event: Events) -> bool {
    match event {
        Events::Ready(cb) => {
            let closure = Closure::new(
                    move |js_value| cb(serde_wasm_bindgen::from_value(js_value).unwrap()),
                );
            let cb_js =
                &closure;
            js_wrapper::addListenerWithParam("ready".to_string(), cb_js)
        }
        Events::NotReady(cb) => {
            let cb_js =
                &Closure::new(
                    move |js_value| cb(serde_wasm_bindgen::from_value(js_value).unwrap()),
                );
            js_wrapper::addListenerWithParam("not_ready".to_string(), cb_js)
        }
        Events::PlayerStateChanged(cb) => {
            let cb_js =
                &Closure::new(
                    move |js_value| cb(serde_wasm_bindgen::from_value(js_value).unwrap()),
                );
            js_wrapper::addListenerWithParam("player_state_changed".to_string(), cb_js)
        }
        Events::AutoplayFailed(cb) => {
            let cb_js = &Closure::new(cb);
            js_wrapper::addListenerAutoplayFailed("autoplay_failed".to_string(), cb_js)
        }
    }
}

pub fn remove_specific_listener(event: Events) -> bool {
    match event {
        Events::Ready(_) => todo!(),
        Events::NotReady(_) => todo!(),
        Events::PlayerStateChanged(_) => todo!(),
        Events::AutoplayFailed(_) => todo!(),
    }
}

pub fn remove_listener(event: Events) -> bool {
    match event {
        Events::Ready(_) => todo!(),
        Events::NotReady(_) => todo!(),
        Events::PlayerStateChanged(_) => todo!(),
        Events::AutoplayFailed(_) => todo!(),
    }
}

pub async fn get_current_state() -> Result<Option<WebPlaybackState>, JsValue> {
    let promise = js_wrapper::getCurrentState();
    let result = JsFuture::from(promise).await?;
    if result.is_null() {
        return Ok(None);
    }
    let state = serde_wasm_bindgen::from_value(result)?;
    Ok(Some(state))
}

pub async fn set_name(name: String) -> Result<(), JsValue> {
    let promise = js_wrapper::setName(name);
    JsFuture::from(promise).await?;
    Ok(())
}

pub async fn get_volume() -> Result<f32, JsValue> {
    let promise = js_wrapper::getVolume();
    let result = JsFuture::from(promise).await?;
    let volume: f32 = serde_wasm_bindgen::from_value(result)?;
    Ok(volume)
}

pub async fn set_volume(volume: f32) -> Result<(), JsValue> {
    let promise = js_wrapper::setVolume(volume);
    JsFuture::from(promise).await?;
    Ok(())
}

pub async fn pause() -> Result<(), JsValue> {
    let promise = js_wrapper::pause();
    JsFuture::from(promise).await?;
    Ok(())
}

pub async fn resume() -> Result<(), JsValue> {
    let promise = js_wrapper::resume();
    JsFuture::from(promise).await?;
    Ok(())
}

pub async fn toggle_play() -> Result<(), JsValue> {
    let promise = js_wrapper::togglePlay();
    JsFuture::from(promise).await?;
    Ok(())
}

pub async fn seek(position_ms: u32) -> Result<(), JsValue> {
    let promise = js_wrapper::seek(position_ms);
    JsFuture::from(promise).await?;
    Ok(())
}

pub async fn previous_track() -> Result<(), JsValue> {
    let promise = js_wrapper::previousTrack();
    JsFuture::from(promise).await?;
    Ok(())
}

pub async fn next_track() -> Result<(), JsValue> {
    let promise = js_wrapper::nextTrack();
    JsFuture::from(promise).await?;
    Ok(())
}

pub fn activate_element() {
    js_wrapper::activateElement();
}
