<script>
import  Button  from '../components/button.svelte';
import { onMount } from 'svelte';

  const handleClick = () => {
    // Navigate to the "new" page
    window.location.href = '/new'; // Replace with your actual route
  };


onMount( () => {
    var conn;
    var msg = document.getElementById("msg");
    var log = document.getElementById("log");

    function appendLog(item) {
        var doScroll = log.scrollTop > log.scrollHeight - log.clientHeight - 1;
        log.appendChild(item);
        if (doScroll) {
            log.scrollTop = log.scrollHeight - log.clientHeight;
        }
    }

    document.getElementById("form").onsubmit = function () {
        if (!conn) {
            return false;
        }
        if (!msg.value) {
            return false;
        }
        conn.send(msg.value);
        msg.value = "";
        return false;
    };

    if (window["WebSocket"]) {
        conn = new WebSocket("ws://" + "localhost:8080" + "/ws");
        conn.onclose = function (evt) {
            var item = document.createElement("div");
            item.innerHTML = "<b>Connection closed.</b>";
            appendLog(item);
        };
        conn.onmessage = function (evt) {
            var messages = evt.data.split('\n');
            for (var i = 0; i < messages.length; i++) {
                var item = document.createElement("div");
                item.innerText = messages[i];
                appendLog(item);
            }
        };
    } else {
        var item = document.createElement("div");
        item.innerHTML = "<b>Your browser does not support WebSockets.</b>";
        appendLog(item);
    }
});
</script>

<style>
.logo-container { 
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100px;
            background-color: purple;
            font-family: Arial, sans-serif;
            font-size: 24px;
            font-weight: bold;
        }

        /* Custom styles for the text */
        .buzz {
            color: #ffcc00; /* Buzz yellow */
        }

        .poll {
            color: #00ccff; /* Poll blue */
        }

</style>

<Button></Button>
<form id="form">
    <input type="submit" value="Send" />
    <input type="text" id="msg" size="64" autofocus />
</form>
