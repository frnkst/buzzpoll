<script lang="ts">
	import { TextField, FormField, Button } from 'attractions';

	function done() {}

	let numberOfAnswers = 1;

	function addNewAnswer(index: number) {
		console.log("frank", index);
    if (index + 1 === numberOfAnswers) {
      numberOfAnswers = numberOfAnswers + 1;
     }
    
    
	}
</script>

<FormField name="Question">
	<TextField />
</FormField>

{#each Array(numberOfAnswers) as _, index}
	<FormField name="Answer 1">
		<TextField on:input={() => addNewAnswer(index)} />
	</FormField>
{/each}

<Button filled on:click={done}>Done</Button>
