<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from "svelte";

	console.log("frank", $page.params.pollId);
  let poll;

	onMount(async function () {
		console.log("trying");
		const endpoint = "http://localhost:8080/poll/" + $page.params.pollId;

		const response = await fetch(endpoint);
		poll  = await response.json();
		console.log("franky: ", poll);
	});

	async function submit(answerId: string) {
		const voteRequest = {
			poll_id: $page.params.pollId,
			answer_id: answerId
		};

		await fetch("http://localhost:8080/vote", {
			headers: {
				'Accept': 'application/json',
				'Content-Type': 'application/json',
			},
      credentials: 'include',
			method: "POST",
			body: JSON.stringify(voteRequest)
		});
	}


</script>

{#if poll}
  <h1>Poll ID: {$page.params.pollId}</h1>

  <h2>{ poll.question}</h2>
  {#each poll.answers as answer}
    <button on:click={() => submit(answer.id)}>{ answer.text } </button>
  {/each}
{/if}



