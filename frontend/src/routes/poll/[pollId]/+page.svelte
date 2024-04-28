<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from "svelte";
	import {writable} from "svelte/store";
	import * as echarts from 'echarts';
	import {option} from "./chart";

	console.log("frank", $page.params.pollId);
  let poll;

	onMount(async function () {

		document.cookie = "buzzpoll=dfsdfdsfd987338324897";

		var myChart = echarts.init(document.getElementById('main'));


    console.log("trying");
		const endpoint = "http://localhost:8080/poll/" + $page.params.pollId;

		const response = await fetch(endpoint);
		poll  = await response.json();
		console.log("franky: ", poll);
		option.yAxis.data = poll.answers.map(answer => answer.text);
		myChart.setOption(option);

		let message;
		let messages = [];

		const messageStore = writable('');

		const socket = new WebSocket('ws://127.0.0.1:8080/ws/');

// Connection opened
		socket.addEventListener('open', function (event) {
			console.log("It's open");
		});

// Listen for messages
		socket.addEventListener('message', function (event) {
			messageStore.set(event.data);
		});

		const sendMessage = (message: any) => {
			if (socket.readyState <= 1) {
				socket.send(message);
			}
		}

		messageStore.subscribe(currentMessage => {
			if (!currentMessage) {
				return
      }

			console.log("we are here", currentMessage);
			const obj = JSON.parse(currentMessage);
			console.log("we are here2", obj);
			option.series[0].data = obj.answers.map(answer => answer.votes.length );
			console.log("options: ", option.series[0].data)
			myChart.setOption(option);



			messages = [...messages, currentMessage];
			console.log("messages", messages);
		})
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


<div id="main" style="height:400px;"></div>



