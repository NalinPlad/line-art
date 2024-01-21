<script>
    import { onMount } from 'svelte';

    // get data from server

    let lines = [];
    let numDrawings = 0;

    function deserialize(l) {
        // return array of lines
        return l.split(';').map(line => {
            let [x1, y1, x2, y2] = line.split(',');
            
            // convert to numbers
            x1 = x1 * 1;
            y1 = y1 * 1;
            x2 = x2 * 1;
            y2 = y2 * 1;

            return { x1, y1, x2, y2 };
        });
    }

    function shift(lines, i) {
        return lines.map(line => {
            return {
                x1: line.x1 + (600 * i),
                y1: line.y1 * 1,
                x2: line.x2 + (600 * i),
                y2: line.y2 * 1,
            };
        });
    }

    onMount(async () => {
        const res = await fetch('http://localhost:8080/drawings');
        const data = await res.json();
        data.forEach((drawing,i) => {

            // console.log(deserialize(drawing.lines))
            lines.push(shift(deserialize(drawing.lines), i));
            numDrawings = i+1;
        })

        console.log(lines)
        lines = lines;
        numDrawings = numDrawings;
    });

</script>

<div class="w-screen overflow-scroll max-w-full">
    <svg width={600*numDrawings} height=400>
        {#each lines as line, i}
        {#each line as l}
        <line x1={l.x1} y1={l.y1} x2={l.x2} y2={l.y2} stroke="black" stroke-width="2" />
        {/each}
        {/each}
    </svg>
</div>