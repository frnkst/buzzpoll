<script lang="ts">
	import type {Poll} from "./Model.svelte";

	export async function save() {
		const poll = {
			question: question,
      answers: ["answer1", "answer2"]
    };

		await fetch("http://localhost:8080/poll",{ headers: {
				'Accept': 'application/json',
				'Content-Type': 'application/json'
			},
			method: "POST",
      body: JSON.stringify(poll)
    });
	}

	let question;
	let answer1;
	let answer2;
</script>

<form>
  <div>
    <label for="question" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Question</label>
    <input bind:value={question} type="text" id="question" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="What do you want to know?" required />
  </div>
  <div>
    <label for="answer1" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Answer 1</label>
    <input bind:value={answer1} type="text" id="answer1" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Answer 1" required />
  </div>
  <div>
    <label for="answer2" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Answer 2</label>
    <input bind:value={answer2} type="text" id="answer2" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Answer 2" required />
  </div>

  <button type="button" on:click={() => save()} class="text-white bg-gradient-to-br from-pink-500 to-orange-400 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-pink-200 dark:focus:ring-pink-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2">Submit</button>
</form>
