<script lang="ts">
    export let value: number;

    const breaks = [4, 22, 27];

    const addNbsp = (input: string, breaks: number[]): string => {
        breaks.forEach((br, idx) => {
            input = input.slice(0, br + idx) + "\xa0" + input.slice(br + idx);
        });

        return input;
    }

    $: getFormattedValue = () => {
        let binary = value.toString(2).padStart(32, '0'); 
        return addNbsp(binary, breaks);
    }

    const annotation = 'NZCVQ--J--------------EAIFTMMMMM';
</script>

<span>{getFormattedValue()}</span><br>
<span>{addNbsp(annotation, breaks)}</span>
