let player;
let oauth_cb;
function init(oauth, on_ready) {
    oauth_cb=oauth;
    console.log("init")
    window.onSpotifyWebPlaybackSDKReady  = ()=>{
        player = new Spotify.Player({
            name: 'Web Playback SDK Quick Start Player',
            getOAuthToken: cb => { 
                cb(oauth_cb());
                console.log(oauth_cb());
            }
            
        });
        on_ready();
    };
    let script = document.createElement("script");
    script.src = "https://sdk.scdn.co/spotify-player.js";
    document.head.appendChild(script);
}



export {init , player}

