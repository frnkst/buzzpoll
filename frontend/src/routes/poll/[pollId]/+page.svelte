<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from "svelte";

	console.log("frank", $page.params.pollId);
let polls: [];
	onMount(async function () {
		const endpoint = "http://localhost:8080/poll/" + $page.params.pollId;

		const response = await fetch(endpoint);
		polls  = await response.json();
	});


</script>


<h1>Poll ID: {$page.params.pollId}</h1>
<!-- Render your poll content here -->
{#each polls as poll}
  { poll.question }
  { poll.answers }

  {/each}
