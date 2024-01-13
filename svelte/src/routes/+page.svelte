

<script>
    import { spring } from 'svelte/motion';
    import MdDelete from 'svelte-icons/md/MdDelete.svelte'
    import MdUndo from 'svelte-icons/md/MdUndo.svelte'
    import MdGridOn from 'svelte-icons/md/MdGridOn.svelte'
    import MdDone from 'svelte-icons/md/MdDone.svelte'

    let lines = [];
    let drawing = false;
    let startPoint = {};
    let rect;

    let grid = false;

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
        return lines.map(line => `${line.x1},${line.y1},${line.x2},${line.y2}`).join(';');
    }

    function submit(serialized_drawing) {
        fetch('http://localhost:8080/submit', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                data: serialized_drawing,
                name: name,
            }),
        })
            // .then(res => res.json())
            .then(res => {
                if (res.status == 200) {
                    alert('Thanks for submitting!');
                } else {
                    alert('Something went wrong :(');
                }
            });
    }



    function startDrawing(event) {
        const clickX = event.clientX - rect.left;
        const clickY = event.clientY - rect.top;

        // check if a line has been started, if it has, then you must restart your drawing from where you left off
        if(!lines.length == 0) {
            // console.log("trying to restart");

            const lastPoint = lines.at(-1);
            console.log(lastPoint, clickX, clickY)
            if(Math.abs(lastPoint.x2 - clickX) < 20 && Math.abs(lastPoint.y2 - clickY) < 20) {
                drawing = true;
                startPoint.x = lastPoint.x2
                startPoint.y = lastPoint.y2
                // console.log("success")
            } else {
                // console.log("failed")
            }
        } else {
            drawing = true;
            startPoint.x = clickX
            startPoint.y = clickY
        }
    }
    
    function drawLine(event) {
        rect = document.getElementById("svg").getBoundingClientRect();
        const endPoint = {
            x: event.clientX - rect.left,
            y: event.clientY - rect.top,
        };

        // if close enough to either end circle, make it smaller, and snap to it
        if (Math.abs(endPoint.x) < 20 && Math.abs(endPoint.y - 200) < 20) {
            endSizes.set({L: 2, R: 1});
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

    function undo() {
        lines.pop();
        lines = lines;
    }

    function clear() {
        lines = [];
    }

    function toggleGrid() {
        grid = !grid;
    }
</script>

<div class="flex flex-col justify-center align-middle text-center">
    <h1 class="text-4xl mt-10">Draw your Line Art below</h1>
    <h3 class="mb-32 opacity-25">Start your line on the left dot and end on the right dot</h3>

    <div class="flex justify-center">
        <svg class="shadow-lg border-slate-400" width="600" height="400" on:mousedown={startDrawing} on:mousemove={drawLine} on:mouseup={stopDrawing} on:mouseleave={stopDrawing} id="svg">
            
            {#each lines as line}
                <line x1={line.x1} y1={line.y1} x2={line.x2} y2={line.y2} stroke="black" stroke-width="4" stroke-linecap="round" />
            {/each}

            <circle cx="0" cy="200" r={5*$endSizes.L} fill="black" />
            <circle cx="600" cy="200" r={5*$endSizes.R} fill="black" />

            <!--Generate dot background -->
            {#if grid}
                {#each Array(20) as _, i}
                    {#each Array(20) as _, j}
                        <!-- the circles opacity decreases proportional to their distance from the center -->
                        <circle cx={i*30} cy={j*30} r="2" fill="black" opacity={0.2 - Math.sqrt((i*30 - 300)**2 + (j*30 - 200)**2)/3000} />
                    {/each}
                {/each}
            {/if}
        </svg>
    </div>
    <div class="flex justify-center gap-4 mt-6">
        <!-- Todo wrap this in button component with slot for icons-->
        <div class="w-6 hover:scale-90 hover:rotate-1 hover:opacity-80 active:scale-110 active:opacity-100 transition-all"  on:click={undo}>
            <MdUndo/>
        </div>
        <div class="w-6 hover:scale-90 hover:rotate-1 hover:opacity-80 active:scale-110 active:opacity-100 transition-all" on:click={clear}>
            <MdDelete/>
        </div>
        <div class="w-6 hover:scale-90 hover:rotate-1 hover:opacity-80 active:scale-110 active:opacity-100 transition-all" on:click={toggleGrid}>
            <MdGridOn/>
        </div>
        <div class="w-6 hover:scale-90 hover:rotate-1 hover:opacity-80 active:scale-110 active:opacity-100 transition-all" on:click={_ => submit(serialize(lines))}>
            <MdDone/>
        </div>
    </div>
</div>
