<script lang="ts">

  export async function save() {
		const response = await fetch("htt://localhost:8080/poll",{ method: 'POST'} );
		const data = await response.json();
		console.log("frank", data);
	}
</script>

<button on:click={save}>click here</button>
