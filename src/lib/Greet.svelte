<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
  
  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }
  async function clear() {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
      console.log("CLEAR")
      sendNotification('CLEAR')
      greetMsg = await invoke ("clear", {})
    }
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>
    <button on:click={clear}>
      Clear
    </button>
  </div>
  <p>{greetMsg}</p>
</div>