<script>
	import Polls from "./Polls.svelte";
	import New from "./New.svelte";

	import { onMount } from 'svelte';

	onMount(() => {
		window.document.cookie = 'myCookie=myValue; path=/';
	});

</script>

<svelte:head>
	<title>buzzpoll</title>
	<meta name="description" content="buzzpoll" />
</svelte:head>

<section>
  <h1 class="text-6xl text-emerald-500 pacifico-regular">buzzpoll</h1>
	<New></New>
	<Polls />
</section>
