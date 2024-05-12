//! # A wrapper around the Spotify web playback SDK for targeting wasm with rust
//! ## All the methods now are functions
//! Because you only can have only 1 player per page, so there is no need for an explicit class, rust calls all the methods of the class  through JS
//! **Use the `init` function first** this function adds the script to the document, and creates an instance of the `Spotify.Player` class, if you don't call this function all the other functions will be useless
//! ## [Docs](https://there.is.none.right.now)
//! ## [Repo](https://github.com/KOEGlike/rust_spotify_web_playback_sdk)
//! 
//! # Example in leptos:
//! ```rust
//! use rust_spotify_web_playback_sdk as sp;
//! #[component]
//! fn HomePage() -> impl IntoView {
//!     let (is_sp_ready, set_is_sp_ready) = create_signal(false);
//!     if cfg!(any(target_arch = "wasm32", target_arch = "wasm64")) {
//!         let token="[Your token goes here]";
//!         let oauth_cb = || {
//!             log!("oauth was called");
//!             token.to_string()
//!         };
//!         let oauth_cb = Closure::new(oauth_cb);
//!         let update_signal = move || {
//!             set_is_sp_ready(true);
//!         };
//!         let on_ready = Closure::new(update_signal);
//! 
//!         create_effect(move |_| {
//!             sp::init(
//!                 &oauth_cb,
//!                 &on_ready,
//!                 "example player".to_string(),
//!                 1.0,
//!                 false,
//!             );
//!         });
//!     }
//! 
//!     let connect = create_action(|_| async {
//!         match sp::connect().await {
//!             Ok(_) => log!("connected"),
//!             Err(e) => log!("error {:?}", e.as_string()),
//!         };
//!     });
//! 
//!     let get_state = create_action(|_| async {
//!         log!(
//!             "{:#?}",
//!             sp::get_current_state()
//!                 .await
//!                 .expect("something went wrong")
//!                 .expect("this device is not in use be spotify connect")
//!         );
//!     });
//! 
//!     let (current_song_name, set_current_song_name) = create_signal(String::new());
//! 
//!     if cfg!(any(target_arch = "wasm32", target_arch = "wasm64")) {
//!         let cb = Closure::new(move |jsv: JsValue| {
//!             let state: sp::structs::state_change::StateChange = sp::structs::from_js(jsv);
//!             log!("state changed, {}", state.track_window.current_track.name);
//!             set_current_song_name(state.track_window.current_track.name);
//!         });
//!         create_effect(move |_| {
//!             if is_sp_ready() {
//!                 log!("ready");
//!                 connect.dispatch(());
//!                 sp::add_listener("player_state_changed", &cb);
//!             }
//!         });
//!     }
//! 
//!     view! {
//!         <h1>"Welcome to Leptos!"</h1>
//!         <Suspense fallback=move || view! { <p>"Loading..."</p> }>
//!             <button  on:click=move |_| get_state.dispatch(())>
//!                 "state"
//!             </button>
//!             <p>"Current song: " {current_song_name()}</p>
//!         </Suspense>
//!     }
//! }
//! ```

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


/// Connect our Web Playback SDK instance to Spotify with the credentials provided during initialization.
/// 
/// # Response 
/// a Promise containing a Boolean (either true or false) with the success of the connection.
pub async fn connect() -> Result<bool, JsValue> {
    let promise = js_wrapper::connect();
    let result = JsFuture::from(promise).await?;
    match result.as_bool() {
        Some(b) => Ok(b),
        None => Err(result),
    }
}


/// Closes the current session our Web Playback SDK has with Spotify.
pub fn disconnect() {
    js_wrapper::disconnect();
}


fn event_check(event: &str) -> bool {
    matches!(event, "ready" | "not_ready" | "player_state_changed" | "autoplay_failed" | "initialization_error"|"authentication_error"|"account_error"|"playback_error")
}


/// Create a new event listener in the Web Playback SDK. Alias for Spotify.Player#on.
/// 
/// # Response 
/// Returns a Boolean. Returns true if the event listener for the event_name is unique.
///  See #removeListener for removing existing listeners.
/// 
/// # Arguments
/// * `event` - A valid event name. See Web Playback SDK Events.
/// * `callback` - A callback function to be fired when the event has been executed.
pub fn add_listener(event: &str, callback: &Closure<dyn FnMut(JsValue)>) -> bool {
    if event_check(event) {
        js_wrapper::addListener(event.to_string(), callback)
    } else {
        false
    }
}


/// Remove a specific event listener in the Web Playback SDK.
/// 
/// # Response 
/// Returns a Boolean. Returns true if the event name is valid with registered callbacks from #addListener.
/// 
/// # Arguments
/// * `event` - A valid event name. See Web Playback SDK Events.
/// * `callback` - The callback function you would like to remove from the listener.
pub fn remove_specific_listener(
    
    event: &str , 
    
    callback: &Closure<dyn FnMut(JsValue)>) -> bool {
    if event_check(event) {
        js_wrapper::removeSpecificListener(event.to_string(), callback)
    } else {
        false
    }
}


/// Remove an event listener in the Web Playback SDK.
/// 
/// # Response 
/// Returns a Boolean. Returns true if the event name is valid with 
/// registered callbacks from #addListener.
/// 
/// # Arguments
/// * `event` - A valid event name. See Web Playback SDK Events.
pub fn remove_listener(event: &str) -> bool {
    if event_check(event) {
        js_wrapper::removeListener(event.to_string())
    } else {
        false
    }
}


/// Collect metadata on local playback.
/// 
/// # Response 
/// Returns a Promise. It will return either a WebPlaybackState object or null depending on if the user is successfully connected. Wrapped in result if the future throws an exception
pub async fn get_current_state() -> Result<Option<State>, JsValue> {
    let promise = js_wrapper::getCurrentState();
    let result = JsFuture::from(promise).await?;
    if result.is_null() {
        return Ok(None);
    }
    let state = serde_wasm_bindgen::from_value(result)?;
    Ok(Some(state))
}


/// Rename the Spotify Player device. This is visible across all Spotify Connect devices.
/// 
/// # Response 
/// Returns a Promise.
/// 
/// # Arguments
/// * `name` - The new desired player name.
pub async fn set_name(name: String) -> Result<(), JsValue> {
    let promise = js_wrapper::setName(name);
    JsFuture::from(promise).await?;
    Ok(())
}


/// Get the local volume currently set in the Web Playback SDK.
/// 
/// # Response 
/// Returns a Promise containing the local volume (as a Float between 0 and 1).
pub async fn get_volume() -> Result<f32, JsValue> {
    let promise = js_wrapper::getVolume();
    let result = JsFuture::from(promise).await?;
    let volume: f32 = serde_wasm_bindgen::from_value(result)?;
    Ok(volume)
}


/// Set the local volume for the Web Playback SDK.
/// 
/// # Response 
/// Returns an empty Promise
/// 
/// # Arguments
/// * `volume` - The new desired volume for local playback. Between 0 and 1. Note: On iOS devices, the audio level is always under the user’s physical control. The volume property is not settable in JavaScript. Reading the volume property always returns 1. More details can be found in the iOS-specific Considerations documentation page by Apple.
pub async fn set_volume(volume: f32) -> Result<(), JsValue> {
    let promise = js_wrapper::setVolume(volume);
    JsFuture::from(promise).await?;
    Ok(())
}


/// Pause the local playback.
/// 
/// # Response 
/// Returns an empty Promise
pub async fn pause() -> Result<(), JsValue> {
    let promise = js_wrapper::pause();
    JsFuture::from(promise).await?;
    Ok(())
}


/// Resume the local playback.
/// 
/// # Response 
/// Returns an empty Promise
pub async fn resume() -> Result<(), JsValue> {
    let promise = js_wrapper::resume();
    JsFuture::from(promise).await?;
    Ok(())
}


/// Resume/pause the local playback.
/// 
/// # Response 
/// Returns an empty Promise
pub async fn toggle_play() -> Result<(), JsValue> {
    let promise = js_wrapper::togglePlay();
    JsFuture::from(promise).await?;
    Ok(())
}


/// Seek to a position in the current track in local playback.
/// 
/// # Response 
/// Returns an empty Promise
/// 
/// # Arguments
/// * `position_ms` - The position in milliseconds to seek to.
pub async fn seek(position_ms: u32) -> Result<(), JsValue> {
    let promise = js_wrapper::seek(position_ms);
    JsFuture::from(promise).await?;
    Ok(())
}


/// Switch to the previous track in local playback.
/// 
/// # Response 
/// Returns an empty Promise
pub async fn previous_track() -> Result<(), JsValue> {
    let promise = js_wrapper::previousTrack();
    JsFuture::from(promise).await?;
    Ok(())
}


/// Skip to the next track in local playback.
/// 
/// # Response 
/// Returns an empty Promise
pub async fn next_track() -> Result<(), JsValue> {
    let promise = js_wrapper::nextTrack();
    JsFuture::from(promise).await?;
    Ok(())
}


/// Some browsers prevent autoplay of media by ensuring that all playback is triggered 
/// by synchronous event-paths originating from user interaction such as a click. In the autoplay 
/// disabled browser, to be able to keep the playing state during transfer from other applications to yours, 
/// this function needs to be called in advance. Otherwise it will be in pause state once it’s transferred.
/// 
/// # Response 
/// Returns an empty Promise
pub fn activate_element() {
    js_wrapper::activateElement();
}


/// Create a new event listener in the Web Playback SDK.
/// 
/// # Response 
/// Returns a Boolean. Returns true if the event listener for the event_name is unique.
///  See #removeListener for removing existing listeners.
/// 
/// # Arguments
/// * `event` - A valid event name. See Web Playback SDK Events.
/// * `callback` - A callback function to be fired when the event has been executed.
pub fn on(event: &str, callback: &Closure<dyn FnMut(JsValue)>) -> bool{
    add_listener(event, callback)
}
