<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // import {start, stop} from "tauri-plugin-keepawake-api/dist-js"

  let status = "Not Active"

  async function startKeepAwake() {
    await invoke("plugin:tauri-plugin-keepawake|start");
    status = "Active"
  }

  async function stopKeepAwake() {
    await invoke("plugin:tauri-plugin-keepawake|stop");
    status = "Not Active"
  }

</script>
<div>
  <h1>Keep Awake API</h1>
  <p>Click the buttons below to start and stop the keep awake feature.</p>
  


  <button on:click={startKeepAwake}>Start</button>
  <button on:click={stopKeepAwake}>Stop</button>
  <p>Status: {status}</p>
</div>