//! # A wrapper around the Spotify web playback SDK for targeting wasm with rust
//! ## All the methods now are functions
//! Because you only can have only 1 player per page, so there is no need for an explicit class, rust calls all the methods of the class  through JS
//! **Use the `init` function first** this function adds the script to the document, and creates an instance of the `Spotify.Player` class, if you don't call this function all the other functions will be useless
//! ## [Docs](https://there.is.none.right.now)
//! ## [Repo](https://github.com/KOEGlike/rust_spotify_web_playback_sdk)
//!
//! # Example in leptos:
//! ```rust
//! use leptos::*;
//! #[component]
//! fn Player() -> impl IntoView {
//!     use leptos::logging::log;
//!     use rust_spotify_web_playback_sdk::prelude as sp;
//!
//!     let (current_song_name, set_current_song_name) = create_signal(String::new());
//!
//!     let token = "BQAdHQqBLczVFdCIM58tVbF0eaztF-83cXczNdz2Aua-U7JyOdIlpiG5M7oEww-dK7jo3qjcpMJ4isuyU2RYy3EoD_SWEOX1uW39bpR-KDbjSYeBPb0Jn4QtwXQw2yjQ33oRzVdyRufKF8o7kwXYW-ij6rtio6oDq0PNYIGIyMsDxKhgM5ijt4LXWz-iWQykftBMXdeSWZuU-Z51VyFOPuznUBQj";
//!
//!     let connect = create_action(|_| async {
//!         match sp::connect().await {
//!             Ok(_) => log!("connected"),
//!             Err(e) => log!("error {:?}", e),
//!         };
//!     });
//!
//!     create_effect(move |_| {
//!         sp::init(
//!             || {
//!                 log!("oauth was called");
//!                 token.to_string()
//!             },
//!             move || {
//!                 log!("ready");
//!                 connect.dispatch(());
//!
//!                 sp::add_listener!("player_state_changed", move |state: sp::StateChange| {
//!                     log!("state changed, {}", state.track_window.current_track.name);
//!                     set_current_song_name(state.track_window.current_track.name);
//!                 });
//!             },
//!             "example player",
//!             1.0,
//!             false,
//!         );
//!     });
//!
//!     let get_state = create_action(|_| async {
//!         let state = sp::get_current_state().await.unwrap();
//!         log!("{:#?}", state);
//!     });
//!
//!     let activate_player = create_action(|_| async {
//!        sp::activate_element().await
//!     });
//!
//!    
//!     view! {
//!         <h1>"Welcome to Leptos"</h1>
//!         <button on:click=move |_| activate_player.dispatch(())>
//!             "activate player"
//!         </button>
//!         {
//!             move || match activate_player.value().get() {
//!             Some(Ok(_)) => {
//!                 view! {
//!                     <button  on:click=move |_| get_state.dispatch(())>
//!                         "log state in console"
//!                     </button>
//!                     <p>"Current song: " {current_song_name}</p>
//!                 }.into_view()
//!             }
//!             Some(Err(e)) => {
//!                 view! {
//!                     <p>"Error activating player: " {e}</p>
//!                 }.into_view()
//!             }
//!             None => {
//!                 view! {
//!                     <p>"Activating player..."</p>
//!                 }.into_view()
//!             }
//!         }
//!     }
//!     
//!     }
//! }
//! ```

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;


pub mod js_wrapper;
pub mod structs;
pub mod prelude {
    pub use crate::{
        structs::{
            state_change::StateChange,
            web_playback::{Error, Player, State},
            Track,
        },
        *,
        js_wrapper::player_ready,
    };
    pub mod wasm_bindgen {
        pub use wasm_bindgen::prelude::*;
    }
}

///this function adds the script to the document, and creates an instance of the Spotify.Player class, if you don't call this function all the other functions will be useless
/// # Arguments
/// * `oauth` - A closure that returns a String containing the Spotify OAuth token.
/// * `on_ready` - A closure that is called when the Web Playback SDK is ready.
/// * `name` - A String containing the name of the player.
/// * `volume` - A Float containing the initial volume of the player.
/// * `enableMediaSession` - A Boolean indicating whether to enable media session support.
///
pub fn init<T, F>(oauth: T, on_ready: F, name: &str, volume: f32, enable_media_session: bool)
where
    T: FnMut() -> String + 'static,
    F: FnMut() + 'static,
{
    let oauth = Closure::wrap(Box::new(oauth) as Box<dyn FnMut() -> String>);
    let on_ready = Closure::wrap(Box::new(on_ready) as Box<dyn FnMut()>);
    //leak these closures so they don't get cleaned up
    let oauth = Box::leak(Box::new(oauth)) as &'static Closure<dyn FnMut() -> String>;
    let on_ready = Box::leak(Box::new(on_ready)) as &'static Closure<dyn FnMut()>;
    js_wrapper::init(oauth, on_ready, name.into(), volume, enable_media_session);
}

/// Connect our Web Playback SDK instance to Spotify with the credentials provided during initialization.
///
/// # Response
/// a Promise containing a Boolean (either true or false) with the success of the connection.
pub async fn connect() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::connect();
    let result = match JsFuture::from(promise).await {
        Ok(e) => {e},
        Err(e) => return Err(format!("{:#?}",e)),
    
    };
    match result.as_bool() {
        Some(b) => if b { Ok(()) } else { Err("could not connect".into()) },
        None => Err(format!("not bool, error: {:#?}", result)),
    }
}



/// Closes the current session our Web Playback SDK has with Spotify.
pub fn disconnect() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    js_wrapper::disconnect();
    Ok(())
}


/// # Response
/// Returns a Boolean. Returns true if the event listener for the event_name is unique.
///  See #removeListener for removing existing listeners.
///
/// # Arguments
/// * `event` - A valid event name. See Web Playback SDK Events. Events type
/// * `callback` - A callback function to be fired when the event has been executed.
/// 
/// # Events
/// * `ready` - Emitted when the Spotify Player has been successfully connected to Spotify. 
///             The callback will be called with a Player .
/// 
/// * `not_ready` - Emitted when the Spotify Player has been disconnected from Spotify.
///                 The callback will be called with a Player .
/// 
/// * `player_state_changed` - Emitted when the state of the player has changed.
///                            The callback will be called with a StateChange .
/// 
/// * `autoplay_failed` - Emitted when the Spotify Player has failed to autoplay.
///                       The callback doesn't take any arguments.
/// 
/// * `initialization_error` - Emitted when the Spotify Player has failed to initialize. 
///                            The callback will be called with a Error .
/// 
/// * `authentication_error` - Emitted when the Spotify Player has failed to authenticate.
///                            The callback will be called with a Error .
/// 
/// * `account_error` - Emitted when the Spotify Player has encountered an account error.
///                     The callback will be called with a Error .
/// 
/// * `playback_error` - Emitted when the Spotify Player has encountered a playback error.
///                      The callback will be called with a Error .

#[macro_export]
macro_rules! add_listener {
    ("ready", $cb:expr) => {{
        use $crate::js_wrapper::addListener;
        use $crate::structs::from_js;
        use $crate::structs::web_playback::Player;
        use $crate::prelude::wasm_bindgen::JsValue;
        use $crate::prelude::wasm_bindgen::Closure;
        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        } else {
            let test: Box<dyn FnMut(Player) + 'static> = Box::new($cb);
        let cb = $cb;
        let cb = move |jsv: JsValue| {
            let state = from_js(jsv);
            cb(state)
        };
        let closure = Closure::new(cb);
        let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
        Ok(addListener("ready".into(), closure_ref))
        }

        
    }};
    ("not_ready", $cb:expr) => {{
        use $crate::js_wrapper::addListener;
        use $crate::structs::from_js;
        use $crate::structs::web_playback::Player;
        use $crate::prelude::wasm_bindgen::JsValue;
        use $crate::prelude::wasm_bindgen::Closure;

        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        }

        else {
            let test: Box<dyn FnMut(Player) + 'static> = Box::new($cb);
        let cb = $cb;
        let cb = move |jsv: JsValue| {
            let state = from_js(jsv);
            cb(state)
        };
        let closure = Closure::new(cb);
        let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
        Ok(addListener("not_ready".into(), closure_ref))
        }
    }};
    ("player_state_changed", $cb:expr) => {{
        use $crate::js_wrapper::addListener;
        use $crate::structs::from_js;
        use $crate::structs::state_change::StateChange;
        use $crate::prelude::wasm_bindgen::JsValue;
        use $crate::prelude::wasm_bindgen::Closure;

        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        }

        else {
            let test: Box<dyn FnMut(StateChange) + 'static> = Box::new($cb);
        let cb = $cb;
        let cb = move |jsv: JsValue| {
            //web_sys::console::log_1(&jsv);
            let state = from_js(jsv);
            cb(state)
        };
        let closure = Closure::new(cb);
        let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
        Ok(addListener("player_state_changed".into(), closure_ref))
        }
    }};
    ("autoplay_failed", $cb:expr) => {{
        use std::result::Result;
        use std::string::String;
        use $crate::prelude::wasm_bindgen::Closure;
        use $crate::js_wrapper::addListenerAutoplayFailed;

        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        } else {
            let test: Box<dyn FnMut() + 'static> = Box::new($cb);
        let cb = $cb;

        let closure = Closure::wrap(Box::new(cb) as Box<dyn FnMut()>);
        let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut()>;
        Ok(addListenerAutoplayFailed("autoplay_failed".into(), closure_ref))
        }
    }};
    ("initialization_error", $cb:expr) => {{
        use std::result::Result;
        use std::string::String;
        use $crate::js_wrapper::addListener;
        use $crate::structs::from_js;
        use $crate::structs::web_playback::Error;
        use $crate::prelude::wasm_bindgen::JsValue;
        use $crate::prelude::wasm_bindgen::Closure;

        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        } else {
            let test: Box<dyn FnMut(Error) + 'static> = Box::new($cb);
        let cb = $cb;
        let cb = move |jsv: JsValue| {
            let state = from_js(jsv);
            cb(state)
        };
        let closure = Closure::new(cb);
        let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
        Ok(addListener("initialization_error".into(), closure_ref))
        }
    }};
    ("authentication_error", $cb:expr) => {{
        use std::result::Result;
        use std::string::String;
        use $crate::js_wrapper::addListener;
        use $crate::structs::from_js;
        use $crate::structs::web_playback::Error;
        use $crate::prelude::wasm_bindgen::JsValue;
        use $crate::prelude::wasm_bindgen::Closure;

        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        } else {
            let test: Box<dyn FnMut(Error) + 'static> = Box::new($cb);
            let cb = $cb;
            let cb = move |jsv: JsValue| {
                let state = from_js(jsv);
                cb(state)
            };
            let closure = Closure::new(cb);
            let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
            Ok(addListener("authentication_error".into(), closure_ref))
        }

       
    }};
    ("account_error", $cb:expr) => {{
        use std::result::Result;
        use std::string::String;
        use $crate::js_wrapper::addListener;
        use $crate::structs::from_js;
        use $crate::structs::web_playback::Error;
        use $crate::prelude::wasm_bindgen::JsValue;
        use $crate::prelude::wasm_bindgen::Closure;

        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        }

        else {
            let test: Box<dyn FnMut(Error) + 'static> = Box::new($cb);
        let cb = $cb;
        let cb = move |jsv: JsValue| {
            let state = from_js(jsv);
            cb(state)
        };
        let closure = Closure::new(cb);
        let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
        Ok(addListener("account_error".into(), closure_ref))
        }
    }};
    ("playback_error", $cb:expr) => {{
        use std::result::Result;
        use std::string::String;
        use $crate::js_wrapper::addListener;
        use $crate::structs::from_js;
        use $crate::structs::web_playback::Error;
        use $crate::prelude::wasm_bindgen::JsValue;
        use $crate::prelude::wasm_bindgen::Closure;

        use $crate::js_wrapper;

        if !js_wrapper::player_ready() {
            let err:String="player not ready".into();
                                Err(err)
        } else {
            let test: Box<dyn FnMut(Error) + 'static> = Box::new($cb);
            let cb = $cb;
            let cb = move |jsv: JsValue| {
                let state = from_js(jsv);
                cb(state)
            };
            let closure = Closure::new(cb);
            let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
            Ok(addListener("playback_error".into(), closure_ref))
        }

        
    }};
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
    event: &str,
    callback: &Closure<dyn FnMut(JsValue)>,
) -> Result<bool, JsValue> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    Ok(if event_check(event) {
        js_wrapper::removeSpecificListener(event.to_string(), callback)
    } else {
        false
    })
}

fn event_check(event: &str) -> bool {
    matches!(
        event,
        "ready"
            | "not_ready"
            | "player_state_changed"
            | "autoplay_failed"
            | "initialization_error"
            | "authentication_error"
            | "account_error"
            | "playback_error"
    )
}

/// Remove an event listener in the Web Playback SDK.
///
/// # Response
/// Returns a Boolean. Returns true if the event name is valid with
/// registered callbacks from #addListener.
///
/// # Arguments
/// * `event` - A valid event name. See Web Playback SDK Events.
pub fn remove_listener(event: &str) -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
   if event_check(event) {
       if js_wrapper::removeListener(event.to_string()) {
              Ok(())
         } else {
              Err("the event name is not valid with registered callbacks from add_listener".into())
       }
    } else {
        Err("event does not exist".into())
    }
}

use crate::structs::web_playback::State;
/// Collect metadata on local playback.
///
/// # Response
/// Returns a Promise. It will return either a WebPlaybackState object or null depending on if the user is successfully connected. Wrapped in result if the future throws an exception
pub async fn get_current_state() -> Result<Option<State>, String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::getCurrentState();
    let result = match JsFuture::from(promise).await {
        Ok(e) => {e},
        Err(e) => return Err(format!("{:#?}",e)),
    
    };
   // web_sys::console::log_1(&result);
    if result.is_null() {
        return Ok(None);
    }
    Ok(Some(structs::from_js(result)))
    
}

/// Rename the Spotify Player device. This is visible across all Spotify Connect devices.
///
/// # Response
/// Returns a Promise.
///
/// # Arguments
/// * `name` - The new desired player name.
pub async fn set_name(name: String) -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::setName(name);
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Get the local volume currently set in the Web Playback SDK.
///
/// # Response
/// Returns a Promise containing the local volume (as a Float between 0 and 1).
pub async fn get_volume() -> Result<f32, String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::getVolume();
    let result=match JsFuture::from(promise).await {
        Ok(e) => {e},
        Err(e) => return Err(format!("{:#?}",e)),
    };
    match serde_wasm_bindgen::from_value(result) {
        Ok(e) => Ok(e),
        Err(e) => Err(format!("{:#?}",e)),
    }
   
}

/// Set the local volume for the Web Playback SDK.
///
/// # Response
/// Returns an empty Promise
///
/// # Arguments
/// * `volume` - The new desired volume for local playback. Between 0 and 1. Note: On iOS devices, the audio level is always under the user’s physical control. The volume property is not settable in JavaScript. Reading the volume property always returns 1. More details can be found in the iOS-specific Considerations documentation page by Apple.
pub async fn set_volume(volume: f32) -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::setVolume(volume);
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Pause the local playback.
///
/// # Response
/// Returns an empty Promise
pub async fn pause() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::pause();
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Resume the local playback.
///
/// # Response
/// Returns an empty Promise
pub async fn resume() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::resume();
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Resume/pause the local playback.
///
/// # Response
/// Returns an empty Promise
pub async fn toggle_play() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::togglePlay();
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Seek to a position in the current track in local playback.
///
/// # Response
/// Returns an empty Promise
///
/// # Arguments
/// * `position_ms` - The position in milliseconds to seek to.
pub async fn seek(position_ms: u32) -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::seek(position_ms);
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Switch to the previous track in local playback.
///
/// # Response
/// Returns an empty Promise
pub async fn previous_track() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::previousTrack();
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Skip to the next track in local playback.
///
/// # Response
/// Returns an empty Promise
pub async fn next_track() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::nextTrack();
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
}

/// Some browsers prevent autoplay of media by ensuring that all playback is triggered
/// by synchronous event-paths originating from user interaction such as a click. In the autoplay
/// disabled browser, to be able to keep the playing state during transfer from other applications to yours,
/// this function needs to be called in advance. Otherwise it will be in pause state once it’s transferred.
///
/// # Response
/// Returns an empty Promise
pub async fn activate_element() -> Result<(), String> {
    if !js_wrapper::player_ready() {
        return Err("player not ready".into());
    }
    let promise = js_wrapper::activateElement();
    match JsFuture::from(promise).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:#?}",e)),
    }
    
}
