use nestify::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebPlaybackPlayer {
    pub device_id: String,
}

impl From<JsValue> for WebPlaybackPlayer {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
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
    pub position: i32,
    pub duration: i32,
    pub track_window: WebPlaybackTrack,
}

impl From<JsValue> for StateChange {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
}
