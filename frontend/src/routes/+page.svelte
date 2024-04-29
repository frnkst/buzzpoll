<script>
	import Polls from "./Polls.svelte";
	import { goto } from '$app/navigation';

	import { onMount } from 'svelte';

	onMount(() => {
		window.document.cookie = 'myCookie=myValue; path=/';
	});

	function handleNew() {
		goto("/new");
	}

</script>

<svelte:head>
	<title>buzzpoll</title>
	<meta name="description" content="buzzpoll" />
</svelte:head>

<section>
  <h1 class="text-6xl text-emerald-500 pacifico-regular">buzzpoll</h1>

	<div class="w-full flex justify-center">
		<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full w-5/6 mt-12" on:click={handleNew}>
			New poll
		</button>
	</div>

	<Polls />
</section>
