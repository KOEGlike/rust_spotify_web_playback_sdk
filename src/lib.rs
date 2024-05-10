use leptos::logging::log;
use nestify::nest;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

mod js_wrapper;
pub use js_wrapper::init;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebPlaybackPlayer {
    device_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebPlaybackError {}

nest! {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct WebPlaybackTrack{
        /// Spotify URI
        pub uri: String,
        /// Spotify ID from URI (can be null)
        pub id: Option<String>,
        /// Content type: can be "track", "episode" or "ad"
        #[serde(rename = "type")]
        pub track_type: String,
        /// Type of file: can be "audio" or "video"
        pub media_type: String,
        /// Name of content
        pub name: String,
        /// Flag indicating whether it can be played
        pub is_playable: bool,
        pub album:
            #[derive(Serialize, Deserialize, Debug)]
            pub struct WebPlaybackAlbum {
                /// Spotify Album URI
                pub uri: String,
                pub name: String,
                pub images: Vec<
                    #[derive(Serialize, Deserialize, Debug)]
                    pub struct WebPlaybackImage{
                        url: String,
                    }
                >
            },
        pub artists: Vec<
            #[derive(Serialize, Deserialize, Debug)]
            pub struct WebPlaybackArtist{
                pub uri: String,
                pub name: String,
            }
        >
    }
}

nest! {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct WebPlaybackState {
        pub context:
            #[derive(Serialize, Deserialize, Debug)]
            pub struct WebPlaybackContext {
            /// The URI of the context
            pub uri: Option<String>,
            /// Additional metadata for the context (can be null)
            pub metadata: Option<
                #[derive(Serialize, Deserialize, Debug)]
                pub struct WebPlaybackContextMetadata {
                    pub context_description:Option<String>
                }
            >
            },
        pub disallows:
            #[derive(Serialize, Deserialize, Debug)]
            pub struct WebPlaybackDisallows {
                /// The current track. By default, these fields
                //pub pausing: bool,
                /// will either be set to false or undefined, which
                pub peeking_next: bool,
                /// indicates that the particular operation is
                pub peeking_prev: bool,
                /// allowed. When the field is set to `true`, this
                pub resuming: bool,
                /// means that the operation is not permitted. For
                pub seeking: bool,
                /// example, `skipping_next`, `skipping_prev` and
                pub skipping_next: bool,
                /// `seeking` will be set to `true` when playing an ada track.
                pub skipping_prev: bool
            },
        /// Whether the current track is paused.
        pub paused: bool,
        /// The position_ms of the current track.
        pub position: i32,
        /// The repeat mode. No repeat mode is 0, repeat context is 1 and repeat track is 2.
        pub repeat_mode: i8,
        /// True if shuffled, false otherwise.
        pub shuffle: bool,
        pub track_window:
            #[derive(Serialize, Deserialize, Debug)]
            pub struct WebPlaybackTrackWindow{
                /// The track currently on local playback
                pub current_track: WebPlaybackTrack,
                /// Previously played tracks. Number can vary.
                pub previous_tracks: Vec<WebPlaybackTrack>,
                /// Tracks queued next. Number can vary.
                pub next_tracks: Vec<WebPlaybackTrack>
            }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateChange {
    position: i32,
    duration: i32,
    track_window: WebPlaybackTrack,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Events<A, B, C, D>
where
    A: Fn(WebPlaybackPlayer) + 'static,
    B: Fn(WebPlaybackPlayer) + 'static,
    C: Fn(StateChange) + 'static,
    D: Fn() + 'static,
{
    Ready(A),
    NotReady(B),
    PlayerStateChanged(C),
    AutoplayFailed(D),
}

impl<A, B, C, D> From<Events<A, B, C, D>> for &str
where
    A: Fn(WebPlaybackPlayer) + 'static,
    B: Fn(WebPlaybackPlayer) + 'static,
    C: Fn(StateChange) + 'static,
    D: Fn() + 'static,
{
    fn from(val: Events<A, B, C, D>) -> Self {
        match val {
            Events::Ready(_) => "ready",
            Events::NotReady(_) => "not_ready",
            Events::PlayerStateChanged(_) => "player_state_changed",
            Events::AutoplayFailed(_) => "autoplay_failed",
        }
    }
}

#[allow(dead_code)]


  
    

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

    pub fn add_listener<A, B, C, D>( event: Events<A, B, C, D>) -> bool 
    where
        A: Fn(WebPlaybackPlayer) + 'static,
        B: Fn(WebPlaybackPlayer) + 'static,
        C: Fn(StateChange) + 'static,
        D: Fn() + 'static,
    {
        match event {
            Events::Ready(cb) => {
                let cb_js=&Closure::new(move |js_value|cb(serde_wasm_bindgen::from_value(js_value).unwrap()));
                js_wrapper::addListenerWithParam("ready".to_string(), cb_js)
            },
            Events::NotReady(cb) => {
                let cb_js=&Closure::new(move |js_value|cb(serde_wasm_bindgen::from_value(js_value).unwrap()));
                js_wrapper::addListenerWithParam("not_ready".to_string(), cb_js)
            },
            Events::PlayerStateChanged(cb) => {
                let cb_js=&Closure::new(move |js_value|cb(serde_wasm_bindgen::from_value(js_value).unwrap()));
                js_wrapper::addListenerWithParam("player_state_changed".to_string(), cb_js)
            },
            Events::AutoplayFailed(cb) => {
                let cb_js=&Closure::new(cb);
                js_wrapper::addListenerAutoplayFailed("autoplay_failed".to_string(), cb_js)
            },
        }
    }

    pub fn remove_specific_listener<A, B, C, D>( event: Events<A, B, C, D>) -> bool
    where
        A: Fn(WebPlaybackPlayer),
        B: Fn(WebPlaybackPlayer),
        C: Fn(StateChange),
        D: Fn(),
    {
       match event {
        Events::Ready(_) => todo!(),
        Events::NotReady(_) => todo!(),
        Events::PlayerStateChanged(_) => todo!(),
        Events::AutoplayFailed(_) => todo!(),
        }
    }

    pub fn remove_listener<A, B, C, D>( event: Events<A, B, C, D>) -> bool 
    where
        A: Fn(WebPlaybackPlayer),
        B: Fn(WebPlaybackPlayer),
        C: Fn(StateChange),
        D: Fn(),
    {
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
        log!("{:?}", result.as_string());
        let state  = serde_wasm_bindgen::from_value(result)?;
        Ok(Some(state))
    }

    pub async fn set_name( name: String) -> Result<(), JsValue> {
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

    pub async fn set_volume( volume: f32) -> Result<(), JsValue> {
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

    pub async fn seek( position_ms: u32) -> Result<(), JsValue> {
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
