<script>
	import Counter from './Counter.svelte';
  import Polls from "./Polls.svelte";
</script>

<svelte:head>
	<title>buzzpoll</title>
	<meta name="description" content="buzzpoll" />
</svelte:head>

<section>
  <h1 class="text-3xl text-amber-800">buzzpoll</h1>

	<Polls />
</section>
