let player="not ready";
let oauth_cb;
function init(oauth, on_ready, name, volume, enableMediaSession) {
    oauth_cb=oauth;
    console.log("init")
    window.onSpotifyWebPlaybackSDKReady  = ()=>{
        player = new Spotify.Player({
            name: name,
            getOAuthToken: cb => { 
                cb(oauth_cb());
            },
            volume: volume,
            enableMediaSession: enableMediaSession
            
        });
        on_ready();
    };
    let script = document.createElement("script");
    script.src = "https://sdk.scdn.co/spotify-player.js";
    document.head.appendChild(script);
}

function player_ready() {
    if(player==="not ready")
        return false;
    return true;
}



export {player_ready,init , player}

