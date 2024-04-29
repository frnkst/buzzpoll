<script lang="ts">
	import { goto } from '$app/navigation';

	export async function save() {
		const pollRequest = {
			question: question,
      answers: ["answer1", "answer2"]
    };

		const response = await fetch("http://localhost:8080/poll", {
			headers: {
				'Accept': 'application/json',
				'Content-Type': 'application/json'
			},
			method: "POST",
			body: JSON.stringify(pollRequest)
		});

		const poll = await response.json();
		await goto(`poll/${poll.id}`)
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

  <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full w-5/6 mt-12" on:click={() => save()} >
    Submit
  </button>
</form>
