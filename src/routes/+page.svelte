

<script>
    import { spring } from 'svelte/motion';
    import MdDelete from 'svelte-icons/md/MdDelete.svelte'

    let lines = [];
    let drawing = false;
    let startPoint = {};
    let rect;

    let name = "";

    let completed = false;

    const segment_length = 5;

    let endSizes = spring(
        {
            L: 1,
            R: 1,
        },
        {
            stiffness: 0.1,
            damping: 0.2,
        }
    );

    function serialize(lines) {
        return lines.map(line => `${line.x1},${line.y1},${line.x2},${line.y2}`).join(' ');
    }



    function startDrawing(event) {
        drawing = true;
        startPoint.x = event.clientX - rect.left;
        startPoint.y = event.clientY - rect.top;
    }
    
    function drawLine(event) {
        rect = document.getElementById("svg").getBoundingClientRect();
        const endPoint = {
            x: event.clientX - rect.left,
            y: event.clientY - rect.top,
        };

        // if close enough to either end circle, make it smaller, and snap to it
        if (Math.abs(endPoint.x) < 20 && Math.abs(endPoint.y - 200) < 20) {
            endSizes.set({L: 2, R: 1})
            endPoint.x = 0;
            endPoint.y = 200;
            startPoint = endPoint;
        } else if (Math.abs(endPoint.x - 600) < 20 && Math.abs(endPoint.y - 200) < 20) {
            endSizes.set({L: 1, R: 2})
            endPoint.x = 600;
            endPoint.y = 200;
            if(!drawing) startPoint = endPoint;
        } else {
            endSizes.set({L: 1, R: 1})
        }



        if (!drawing) return;

        // if more than 5 pixels away from the last point
        if (Math.abs(endPoint.x - startPoint.x) > segment_length || Math.abs(endPoint.y - startPoint.y) > segment_length) {
            lines = [...lines, { x1: startPoint.x, y1: startPoint.y, x2: endPoint.x, y2: endPoint.y }];
            startPoint = endPoint;
        }

        // check if array starts at left and ends at right
        if (lines.length > 0) {
            if (lines[0].x1 == 0 && lines[0].y1 == 200 && lines[lines.length - 1].x2 == 600 && lines[lines.length - 1].y2 == 200) {
                completed = true;
            }
        }
    }

    function stopDrawing() {
        drawing = false;
    }
</script>

<style>
    /* svg {
        cursor: crosshair;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
    }

    .main {
        display: flex;
        justify-content: center;
        flex-direction: column;
        align-items: center;
        height: 100vh;
    }

    .icons {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-top: 20px;
    }

    .icon {
        margin: 0 10px;
        cursor: pointer;
        width: 30px;
        height: 30px;
    }

    .icon > svg {
        margin: 1rem;
    }

    .settings {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-top: 20px;
    } */


</style>
<div class="flex flex-col justify-center align-middle text-center">
    <h1 class="text-4xl">Draw your Line Art below</h1>
    <h3 class="mb-32 opacity-25">Start your line on the left dot and end on the right dot</h3>

    <div>

        
        <svg class="shadow-lg" width="600" height="400" on:mousedown={startDrawing} on:mousemove={drawLine} on:mouseup={stopDrawing} on:mouseleave={stopDrawing} id="svg">
            {#each lines as line}
            <line x1={line.x1} y1={line.y1} x2={line.x2} y2={line.y2} stroke="black" stroke-width="4" stroke-linecap="round" />
            {/each}
            <circle cx="0" cy="200" r={5*$endSizes.L} fill="black" />
            <circle cx="600" cy="200" r={5*$endSizes.R} fill="black" />
        </svg>
    </div>
        <div class="settings">
            <div class="icons">
            <div class="icon" on:click={_ => lines = []}>
                <MdDelete/>
            </div>
        </div>
    </div>
</div>