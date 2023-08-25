function debounce (f, timeout=300) {
    let timer;
    return (...args) => {
        clearTimeout(timer);
        timer = setTimeout(() => { f.apply(this, args); }, timeout);
    };
}

console.log("DEV SERVER");
// Create ws connection
const socket = new WebSocket("ws://localhost:5000/reload");

// Connection opened
socket.addEventListener("open", (event) => {
    socket.send("Hello Server!");
});

// Listen for messages
socket.addEventListener("message", debounce((event) => {
    console.log("Message from server ", event.data);
    if (event.data === "reload") {
        console.log("RELOADING");
        location.reload();
    }
}));
