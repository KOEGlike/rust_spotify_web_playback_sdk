let player;
let oauth_cb;
function init(cb) {
    oauth_cb=cb;
    console.log("init")
    window.onSpotifyWebPlaybackSDKReady  = ()=>{
        player = new Spotify.Player({
            name: 'Web Playback SDK Quick Start Player',
            getOAuthToken: cb => { 
                cb(oauth_cb());
                console.log(oauth_cb());
            }
            
        });
    };
    let script = document.createElement("script");
    script.src = "https://sdk.scdn.co/spotify-player.js";
    document.head.appendChild(script);
}



export {init , player}

