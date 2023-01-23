<script lang="ts">
    import {Graphic, Section, Line, XAxis, YAxis, XGridLines, YGridLines} from "@snlab/florence"
    import { emit, listen } from '@tauri-apps/api/event'
    import { invoke } from "@tauri-apps/api/tauri";


    let zoomIdentity = {x: 0, y: 0, kx: 1, ky: 1}
    let blockReindexing = false

    let x_points = [0]
    let y_points = [0]
    let count = 0

    let unlisten = undefined
    let subscribed = false

    const setZoomIdentity = zoomID => {zoomIdentity = zoomID}
    const setBlockReindexing = bool => {blockReindexing = bool}

    async function handleUpdate(count) {
        x_points = await invoke("update_x", {})
        y_points = await invoke("update_y", {count})
        console.log({y_points})
    }

    async function subscribe() {
        console.log("Subscribing")
        subscribed = true
        await invoke("init_process", {})
        unlisten = await listen<string>('update', (event) => {
            handleUpdate(count)
            console.log(event.payload)
            count += 1
            if (count > 100) {
                console.log("Unsubscribing")
                count = 0;
                unlisten()
            }
    })
    
    }

    async function unsubscribe() {
        console.log("Un")
        unlisten()
        subscribed = false
        //await invoke("kill_process", {})
    }

    

</script>

<Graphic width={200} height={200} padding={20}>
    <Section
        scaleY={[-1, 1]}
        scaleX={[0, 100]}
        pannable={true}
        zoomable={true}
    >
    <Line
        x={x_points}
        y={y_points}
        stroke={'green'}
    />
    <XAxis/>
    <YAxis/>
    <XGridLines/>
    <YGridLines/>
</Section>
</Graphic>
<div>
<button on:click={subscribe}>
    Subscribe
</button>
<button disabled={!subscribed} on:click={unsubscribe} >
    Unsubscribe
</button>
</div>