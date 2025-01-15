let player = "not ready";
let oauth_cb;

function init(oauth, on_ready, name, volume, enableMediaSession) {
    oauth_cb = oauth;
    window.onSpotifyWebPlaybackSDKReady = () => {
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
    return player !== "not ready";
}

function log_player() {
    console.log(player);
}

function connect() {
    return player.connect();
}

function disconnect() {
    player.disconnect();
}

function addListener(event, callback) {
    return player.addListener(event, callback);
}

function addListenerAutoplayFailed(event, callback) {
    return player.addListener(event, callback);
}

function removeListener(event) {
    return player.removeListener(event);
}

function removeSpecificListener(event, callback) {
    return player.removeListener(event, callback);
}

function getCurrentState() {
    return player.getCurrentState();
}

function setName(name) {
    return player.setName(name);
}

function getVolume() {
    return player.getVolume();
}

function setVolume(volume) {
    return player.setVolume(volume);
}

function pause() {
    return player.pause();
}

function resume() {
    return player.resume();
}

function togglePlay() {
    return player.togglePlay();
}

function seek(position_ms) {
    return player.seek(position_ms);
}

function previousTrack() {
    return player.previousTrack();
}

function nextTrack() {
    return player.nextTrack();
}

function activateElement() {
    return player.activateElement();
}

export {
    init,
    player_ready,
    log_player,
    connect,
    disconnect,
    addListener,
    addListenerAutoplayFailed,
    removeListener,
    removeSpecificListener,
    getCurrentState,
    setName,
    getVolume,
    setVolume,
    pause,
    resume,
    togglePlay,
    seek,
    previousTrack,
    nextTrack,
    activateElement
};