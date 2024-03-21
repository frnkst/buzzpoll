<script>
async function getData() {
  
const value = await fetch("http://localhost:8080");
const test = await value.json()

console.log("frank", test);
}

getData();

</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
