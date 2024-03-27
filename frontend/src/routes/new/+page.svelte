<script lang="ts">
	import { TextField, FormField, Button } from 'attractions';

  let question: string;
  
	async function done() {
    await saveData();
    window.location.href = '/done';
  }
  
  async function saveData() {
		const body = JSON.stringify({
      question: question
    });
    
    const a = await fetch("http://localhost:8080/poll", { method: "POST", headers: { 'Content-Type': 'application/json', 'Accept': 'application/json' }, body})
		const b = await a.json();
		console.log("b", b)
  }

	let numberOfAnswers = 1;

	function addNewAnswer(currentAnswer: number) {
		if (currentAnswer === numberOfAnswers) {
			numberOfAnswers = numberOfAnswers + 1;
		}
	}
</script>1

<FormField name="Question">
	<TextField bind:value={question} />
</FormField>

{#each Array(numberOfAnswers) as _, index}
	<FormField name="Answer {index + 1}">
		<TextField on:input={() => addNewAnswer(index + 1)} />
	</FormField>
{/each}

<Button filled on:click={done}>Done</Button>
