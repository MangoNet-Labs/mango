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
        const message = input.value.trim(); // 去除前后空白字符
        if (message === '') {
            alert('请输入消息内容！');
            return;
        }
        if (!/^[\u0000-\u007F\u0080-\uFFFF\s]*$/u.test(message)) {
            alert('输入仅支持文字、字母、符号！');
            return;
        }

        // 发送消息并清空输入框
        if (connect && connect.readyState === WebSocket.OPEN) {
            connect.send(message);
            console.log("Send: " + message);
        } else {
            console.log("WebSocket is not connected.");
        }
        input.value = '';
    }

    button.addEventListener('click', send);

    // 绑定Enter键发送消息
    input.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            send();
            e.preventDefault(); // 防止按下Enter键时触发表单提交或其他默认行为
        }
    });
});