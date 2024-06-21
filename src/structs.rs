use core::panic;

pub fn from_js<T>(js_value: wasm_bindgen::JsValue) -> T
where
    T: serde::de::DeserializeOwned,
{
    match serde_wasm_bindgen::from_value(js_value) {
        Ok(value) => value,
        Err(e) => {
            panic!("This is a bug, submit an issue.Error deserializing JS value: {:?}", e.to_string())
        }
    
    }
}

pub mod web_playback {
    use nestify::*;
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Player {
        pub device_id: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Error {
        pub message: String,
    }

    nest! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct State {
            pub context:
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Context {
                    /// The URI of the context
                    pub uri: Option<String>,
                    /// Additional metadata for the context (can be null)
                    pub metadata: Option<
                        #[derive(Serialize, Deserialize, Debug)]
                        pub struct ContextMetadata {
                            pub context_description:Option<String>
                        }
                    >
                },
            pub disallows:
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Disallows {
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
                pub struct TrackWindow{
                    /// The track currently on local playback
                    pub current_track: Track,
                    /// Previously played tracks. Number can vary.
                    pub previous_tracks: Vec<Track>,
                    /// Tracks queued next. Number can vary.
                    pub next_tracks: Vec<Track>
                }
        }
    }

    nest! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Track{
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
                pub struct Album {
                    /// Spotify Album URI
                    pub uri: String,
                    pub name: String,
                    pub images: Vec<
                        #[derive(Serialize, Deserialize, Debug)]
                        pub struct Image{
                            pub url: String,
                        }
                    >
                },
            pub artists: Vec<
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Artist{
                    pub uri: String,
                    pub name: String,
                }
            >
        }
    }
}

pub mod state_change {
    use crate::structs::web_playback::Context;
    use nestify::*;
    use serde::{Deserialize, Serialize};

    nest! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Track {
            pub album: 
            #[derive(Serialize, Deserialize, Debug)]
            pub struct Album {
                /// Spotify Album URI
                pub uri: String,
                pub name: String,
                pub images: Vec<
                    #[derive(Serialize, Deserialize, Debug)]
                    pub struct Image{
                        pub url: String,
                        pub size: String,
                        pub width: i32,
                        pub height: i32,
                    }
                >
            },
            pub artists: Vec<
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Artist {
                    pub uri: String,
                    pub url: String,
                    pub name: String,
                }
            >,
            pub duration_ms: i32,
            pub id: String,
            pub is_playable: bool,
            pub linked_from: Option<
                #[derive(Serialize, Deserialize, Debug)]
                pub struct LinkedFrom {
                    pub uri: Option<String>,
                    pub id: Option<String>,
                }
            >,
            pub media_type: String,
            pub metadata: Option<
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Metadata {
                }
            >,
            pub name: String,
            pub track_type: String,
            pub uid: String,
            pub uri: String, 
        }
    }

    nest! {
        #[derive(Serialize, Deserialize, Debug)]
        ///couldn't find any documentation for this js object, in the official docs it says that the event listener returns a WebPlaybackPlayer object, but in practice it returns this object
        pub struct StateChange {
            pub context: Context,
            pub disallows:
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Disallows {
                    pub peeking_next: bool,
                    pub peeking_prev: bool,
                    pub seeking: bool,
                    pub skipping_next: bool,
                    pub skipping_prev: bool,
                    pub toggling_repeat_context: bool,
                    pub toggling_repeat_track: bool,
                    pub toggling_shuffle: bool,
                    pub undefined: bool,
                },
            pub duration: i32,
            pub loading: bool,
            pub playback_features:
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Features {
                    pub hifi_status: String,
                    pub playback_speed:
                        #[derive(Serialize, Deserialize, Debug)]
                        pub struct Speed {
                            pub current: i32,
                            pub restricted:bool,
                            pub selected: i32,
                        },
                    pub signal_ids: Vec<String>,
                },
            pub playback_id:String,
            pub playback_quality: String,
            pub playback_speed:i32,
            pub position: i32,
            pub repeat_mode: i8,
            pub restrictions:
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Restrictions {
                    pub disallow_peeking_next_reasons: Vec<String>,
                    pub disallow_peeking_prev_reasons: Vec<String>,
                    pub disallow_seeking_reasons: Vec<String>,
                    pub disallow_skipping_next_reasons: Vec<String>,
                    pub disallow_skipping_prev_reasons: Vec<String>,
                    pub disallow_toggling_repeat_context_reasons: Vec<String>,
                    pub disallow_toggling_repeat_track_reasons: Vec<String>,
                    pub disallow_toggling_shuffle_reasons: Vec<String>,
                    pub undefined:Vec<String>
                },
            pub shuffle: bool,
            pub shuffle_mode: i8,
            pub timestamp: i64,
            pub track_window: 
                #[derive(Serialize, Deserialize, Debug)]
                pub struct TrackWindow {
                    pub current_track: Track,
                    pub next_tracks: Vec<Track>,
                    pub previous_tracks: Vec<Track>,
                },
        }
    }
}
