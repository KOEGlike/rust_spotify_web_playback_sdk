use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

mod js_wrapper;
pub mod structs;
use crate::structs::web_playback::State;
pub use js_wrapper::init;
pub mod prelude{
    pub use crate::{
        *,
        structs::{
            web_playback::{State, Error, Player, Track},
            state_change::StateChange
        }
    };
}

/// #Description Connect our Web Playback SDK instance to Spotify with the credentials provided during initialization.
/// #Returns a Promise containing a Boolean (either true or false) with the success of the connection.
pub async fn connect() -> Result<bool, JsValue> {
    let promise = js_wrapper::connect();
    let result = JsFuture::from(promise).await?;
    match result.as_bool() {
        Some(b) => Ok(b),
        None => Err(result),
    }
}

/// #Closes the current session our Web Playback SDK has with Spotify.
pub fn disconnect() {
    js_wrapper::disconnect();
}


fn event_check(event: &str) -> bool {
    matches!(event, "ready" | "not_ready" | "player_state_changed" | "autoplay_failed" | "initialization_error"|"authentication_error"|"account_error"|"playback_error")
}

/// #Description Create a new event listener in the Web Playback SDK. Alias for Spotify.Player#on.
/// #Response Returns a Boolean. Returns true if the event listener for the event_name is unique. See #removeListener for removing existing listeners.
pub fn add_listener(event: &str, callback: &Closure<dyn FnMut(JsValue)>) -> bool {
    if event_check(event) {
        js_wrapper::addListener(event.to_string(), callback)
    } else {
        false
    }
}

/// #Description Remove a specific event listener in the Web Playback SDK.
/// #Response Returns a Boolean. Returns true if the event name is valid with registered callbacks from #addListener.
pub fn remove_specific_listener(event: &str , callback: &Closure<dyn FnMut(JsValue)>) -> bool {
    if event_check(event) {
        js_wrapper::removeSpecificListener(event.to_string(), callback)
    } else {
        false
    }
}

/// #Description Remove an event listener in the Web Playback SDK.
/// #Response Returns a Boolean. Returns true if the event name is valid with registered callbacks from #addListener.
pub fn remove_listener(event: &str) -> bool {
    if event_check(event) {
        js_wrapper::removeListener(event.to_string())
    } else {
        false
    }
}

/// #Description Collect metadata on local playback.
/// #Response Returns a Promise. It will return either a WebPlaybackState object or null depending on if the user is successfully connected.
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

pub fn on(event: &str, callback: &Closure<dyn FnMut(JsValue)>) -> bool{
    add_listener(event, callback)
}
