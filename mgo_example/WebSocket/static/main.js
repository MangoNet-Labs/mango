const wsUri = "ws://localhost:7070/ws";

const log = document.getElementById('log');

connect = new WebSocket(wsUri);
console.log("Connecting...");

connect.onopen = function() {
    console.log("Connected Sucessfully!");
};

connect.onmessage = function(e) {
    console.log("Rec:" + e.data);
    document.getElementById("log").textContent =
        document.getElementById("log").textContent + "\n" + e.data;
};
connect.onclose = function() {
    console.log("Connect Closed!");
    connect = null;
};

document.addEventListener('DOMContentLoaded', function() {
    const input = document.getElementById('input');
    const button = document.getElementById('btn');

    function send() {
        const message = input.value.trim(); // Remove leading and trailing whitespace
        if (message === '') {
            alert('Please enter the message!');
            return;
        }
        if (!/^[\u0000-\u007F\u0080-\uFFFF\s]*$/u.test(message)) {
            alert('Input only supports words, letters, symbols!');
            return;
        }

        // Send the message and clear the field
        if (connect && connect.readyState === WebSocket.OPEN) {
            connect.send(message);
            console.log("Send: " + message);
        } else {
            console.log("WebSocket is not connected.");
        }
        input.value = '';
    }

    button.addEventListener('click', send);

    // Bind the Enter key to send a message
    input.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            send();
            // Prevents form submission or other default behavior from being triggered when the Enter key is pressed
            e.preventDefault();
        }
    });
});