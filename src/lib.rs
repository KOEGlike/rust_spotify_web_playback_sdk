#![feature(type_alias_impl_trait)]
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

mod js_wrapper;
mod structs;
use crate::structs::web_playback::State;
pub use js_wrapper::init;
pub use structs::*;

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

pub fn add_listener(event: &str, callback: &Closure<dyn FnMut(JsValue)>) -> bool {
    match event {
        "ready" | "not_ready" | "player_state_changed" | "autoplay_failed" => {
            js_wrapper::addListener(event.to_string(), callback)
        }
        _ => false,
    }
}

pub fn remove_specific_listener(event: &str , callback: &Closure<dyn FnMut(JsValue)>) -> bool {
    match event {
        "ready" | "not_ready" | "player_state_changed" | "autoplay_failed" => {
            js_wrapper::removeSpecificListener(event.to_string(), callback)
        }
        _ => false,
    }
}

pub fn remove_listener(event: &str) -> bool {
    match event {
        "ready" | "not_ready" | "player_state_change" | "autoplay_failed" => {
            js_wrapper::removeListener(event.to_string())
        }
        _ => false,
    }
}

pub async fn get_current_state() -> Result<Option<State>, JsValue> {
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
