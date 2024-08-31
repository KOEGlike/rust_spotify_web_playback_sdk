# A wrapper around the Spotify web playback SDK for targeting wasm with rust

## All the methods now are functions

Because you only can have only 1 player per page, so there is no need for an explicit class, rust calls all the methods of the class  through JS.
**Use the `init` function first** , this function adds the script to the document, and creates an instance of the `Spotify.Player` class, if you don't call this function all the other functions will be useless.

### [Docs](https://docs.rs/rust_spotify_web_playback_sdk/latest/)

### [Repo](https://github.com/KOEGlike/rust_spotify_web_playback_sdk)

### [Crates.io](https://crates.io/crates/rust_spotify_web_playback_sdk)

## Example in leptos

```rust
use leptos::*;
#[component]
fn Player() -> impl IntoView {
    use leptos::logging::log;
    use rust_spotify_web_playback_sdk::prelude as sp;

    let (current_song_name, set_current_song_name) = create_signal(String::new());

    let token = "BQAdHQqBLczVFdCIM58tVbF0eaztF-83cXczNdz2Aua-U7JyOdIlpiG5M7oEww-dK7jo3qjcpMJ4isuyU2RYy3EoD_SWEOX1uW39bpR-KDbjSYeBPb0Jn4QtwXQw2yjQ33oRzVdyRufKF8o7kwXYW-ij6rtio6oDq0PNYIGIyMsDxKhgM5ijt4LXWz-iWQykftBMXdeSWZuU-Z51VyFOPuznUBQj";

    let connect = create_action(|_| async {
        match sp::connect().await {
            Ok(_) => log!("connected"),
            Err(e) => log!("error {:?}", e),
        };
    });

    create_effect(move |_| {
        sp::init(
            || {
                log!("oauth was called");
                token.to_string()
            },
            move || {
                log!("ready");
                connect.dispatch(());

                sp::add_listener!("player_state_changed", move |state: sp::StateChange| {
                    log!("state changed, {}", state.track_window.current_track.name);
                    set_current_song_name(state.track_window.current_track.name);
                });
            },
            "example player",
            1.0,
            false,
        );
    });

    let get_state = create_action(|_| async {
        let state = sp::get_current_state().await.unwrap();
        log!("{:#?}", state);
    });

    let activate_player = create_action(|_| async {
       sp::activate_element().await
    });

    
    view! {
        <h1>"Welcome to Leptos"</h1>
        <button on:click=move |_| activate_player.dispatch(())>
            "activate player"
        </button>
        {
            move || match activate_player.value().get() {
            Some(Ok(_)) => {
                view! {
                    <button  on:click=move |_| get_state.dispatch(())>
                        "log state in console"
                    </button>
                    <p>"Current song: " {current_song_name}</p>
                }.into_view()
            }
            Some(Err(e)) => {
                view! {
                    <p>"Error activating player: " {e}</p>
                }.into_view()
            }
            None => {
                view! {
                    <p>"Activating player..."</p>
                }.into_view()
            }
        }
    }
     
    }
}
```
