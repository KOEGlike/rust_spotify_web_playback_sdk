# A wrapper around the Spotify web playback SDK for targeting wasm with rust
## All the methods now are functions
Because you only can have only 1 player per page, so there is no need for an explicit class, rust calls all the methods of the class  through JS
**Use the `init` function first** this function adds the script to the document, and creates an instance of the `Spotify.Player` class, if you don't call this function all the other functions will be useless
## [Docs](https://there.is.none.right.now)
## [Repo](https://github.com/KOEGlike/rust_spotify_web_playback_sdk)

# Example in leptos:
```rust
use rust_spotify_web_playback_sdk as sp;
#[component]
fn HomePage() -> impl IntoView {
    let (is_sp_ready, set_is_sp_ready) = create_signal(false);
    if cfg!(any(target_arch = "wasm32", target_arch = "wasm64")) {
        let token="BQDTd92HRscBbveUPGqAL1WNd0RaFTMC5ctC64eDMo0txTZnkeWdzf-okCSJ8YAYCp4fvm-JuZXB3sl1PJk1I4JpFzsZ_kzZa8Rt_DX5SdHpeUVxoWAZFO_8ATVVp0Ix86N83nXiZJpf5-GBLvjaA_HDGu05j9PfvjWy1-630mhTcj_N4BN5fSX3idngPEMgpooXyVRILH9WYO-fXUs5M7xIZdn4";
        let oauth_cb = || {
            log!("oauth was called");
            token.to_string()
        };
        let oauth_cb = Closure::new(oauth_cb);
        let update_signal = move || {
            set_is_sp_ready(true);
        };
        let on_ready = Closure::new(update_signal);

        create_effect(move |_| {
            sp::init(
                &oauth_cb,
                &on_ready,
                "example player".to_string(),
                1.0,
                false,
            );
        });
    }

    let connect = create_action(|_| async {
        match sp::connect().await {
            Ok(_) => log!("connected"),
            Err(e) => log!("error {:?}", e.as_string()),
        };
    });

    let get_state = create_action(|_| async {
        log!(
            "{:#?}",
            sp::get_current_state()
                .await
                .expect("something went wrong")
                .expect("this device is not in use be spotify connect")
        );
    });

    let (current_song_name, set_current_song_name) = create_signal(String::new());

    if cfg!(any(target_arch = "wasm32", target_arch = "wasm64")) {
        let cb = Closure::new(move |jsv: JsValue| {
            let state: sp::structs::state_change::StateChange = sp::structs::from_js(jsv);
            log!("state changed, {}", state.track_window.current_track.name);
            set_current_song_name(state.track_window.current_track.name);
        });
        create_effect(move |_| {
            if is_sp_ready() {
                log!("ready");
                connect.dispatch(());
                sp::add_listener("player_state_changed", &cb);
            }
        });
    }

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <button  on:click=move |_| get_state.dispatch(())>
                "state"
            </button>
            <p>"Current song: " {current_song_name()}</p>
        </Suspense>
    }
}
```
