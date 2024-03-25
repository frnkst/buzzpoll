<script lang="ts">
	import { TextField, FormField, Button } from 'attractions';

  let question: string;
  
	function done() {
    saveData();
    window.location.href = '/done';
  }
  
  function saveData() {
		const body = JSON.stringify({
      question: question
    });
    
    await fetch("http://localhost:8080/poll", { method: "POST", headers: { 'Content-Type': 'application/json', 'Accept': 'application/json' }, body})

  }

	let numberOfAnswers = 1;

	function addNewAnswer(currentAnswer: number) {
		if (currentAnswer === numberOfAnswers) {
			numberOfAnswers = numberOfAnswers + 1;
		}
	}
</script>

<FormField name="Question">
	<TextField bind:value={question} />
</FormField>

{#each Array(numberOfAnswers) as _, index}
	<FormField name="Answer {index + 1}">
		<TextField on:input={() => addNewAnswer(index + 1)} />
	</FormField>
{/each}

<Button filled on:click={done}>Done</Button>
