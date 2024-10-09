extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ExprClosure, LitStr, Token};

/// Struct to parse the macro input as two separate arguments
struct ListenerInput {
    event_name: LitStr,
    _comma: Token![,],
    callback: ExprClosure,
}

impl Parse for ListenerInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ListenerInput {
            event_name: input.parse()?,
            _comma: input.parse()?,
            callback: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn add_listener(input: TokenStream) -> TokenStream {
    // Parse the input as our custom ListenerInput struct
    let ListenerInput {
        event_name,
        _comma,
        callback,
    } = parse_macro_input!(input as ListenerInput);

    let event_name_str = event_name.value();

    // Match the event name to determine the correct listener
    let (add_listener_fn, closure_type) = match event_name_str.as_str() {
        "ready" | "not_ready" => (
            quote! { addListener },
            quote! { Box<dyn FnMut(Player) + 'static> },
        ),
        "player_state_changed" => (
            quote! { addListener },
            quote! { Box<dyn FnMut(StateChange) + 'static> },
        ),
        "autoplay_failed" => (
            quote! { addListenerAutoplayFailed },
            quote! { Box<dyn FnMut() + 'static> },
        ),
        "initialization_error" | "authentication_error" | "account_error" | "playback_error" => (
            quote! { addListener },
            quote! { Box<dyn FnMut(Error) + 'static> },
        ),
        _ => panic!("Unknown event name"),
    };

    // Generate the output code
    let output = quote! {
        {
            use rust_spotify_web_playback_sdk::js_wrapper::*;
            use rust_spotify_web_playback_sdk::structs::from_js;
            use rust_spotify_web_playback_sdk::prelude::*;
            use rust_spotify_web_playback_sdk::prelude::wasm_bindgen::prelude::JsValue;
            use rust_spotify_web_playback_sdk::prelude::wasm_bindgen::prelude::Closure;
            use rust_spotify_web_playback_sdk::js_wrapper;

            if !js_wrapper::player_ready() {
                let err: String = "player not ready".into();
                Err(err)
            } else {
                let mut cb: #closure_type = Box::new(#callback);
                let cb = move |jsv: JsValue| {
                    let state = from_js(jsv);
                    cb(state)
                };
                let closure = Closure::new(cb);
                let closure_ref = Box::leak(Box::new(closure)) as &'static Closure<dyn FnMut(JsValue)>;
                Ok(#add_listener_fn(#event_name_str.into(), closure_ref))
            }
        }
    };

    output.into()
}
