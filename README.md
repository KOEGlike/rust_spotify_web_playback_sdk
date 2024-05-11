# A wrapper around the Spotify web playback SDK for targeting wasm with rust
## All the methods now are functions
Because you only can have only 1 player per page, so there is no need for an explicit class, rust calls all the methods of the class  through JS
**Use the `init` function first** this function adds the script to the document, and creates an instance of the `Spotifi.Player` class, if you don't call this function all the other functions will be useless
# [Docs](https://there.is.none.right.now)
# [Repo](https://github.com/KOEGlike/rust_spotify_web_playback_sdk)
