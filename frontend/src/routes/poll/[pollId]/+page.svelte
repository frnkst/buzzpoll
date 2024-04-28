<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from "svelte";
	import {writable} from "svelte/store";
	import * as echarts from 'echarts';

	console.log("frank", $page.params.pollId);
  let poll;

  function chart() {
		var myChart = echarts.init(document.getElementById('main'));

		var data = [];
		for (let i = 0; i < 5; ++i) {
			data.push(Math.round(Math.random() * 200));
		}

		const option = {
			xAxis: {
				max: 'dataMax'
			},
			yAxis: {
				type: 'category',
				data: ['A', 'B', 'C', 'D', 'E'],
				inverse: true,
				animationDuration: 300,
				animationDurationUpdate: 300,
				max: 2 // only the largest 3 bars will be displayed
			},
			series: [
				{
					realtimeSort: true,
					name: 'X',
					type: 'bar',
					data: data,
					label: {
						show: true,
						position: 'right',
						valueAnimation: true
					}
				}
			],
			legend: {
				show: true
			},
			animationDuration: 0,
			animationDurationUpdate: 3000,
			animationEasing: 'linear',
			animationEasingUpdate: 'linear'
		};

		function run() {
			var data = option.series[0].data;
			for (var i = 0; i < data.length; ++i) {
				if (Math.random() > 0.9) {
					data[i] += Math.round(Math.random() * 2000);
				} else {
					data[i] += Math.round(Math.random() * 200);
				}
			}
			myChart.setOption(option);
		}

		myChart.setOption(option);

		setTimeout(function() {
			run();
		}, 0);
		setInterval(function() {
			run();
		}, 3000);

  }

	onMount(async function () {

  chart();






		console.log("trying");
		const endpoint = "http://localhost:8080/poll/" + $page.params.pollId;

		const response = await fetch(endpoint);
		poll  = await response.json();
		console.log("franky: ", poll);

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


<div id="main" style="width:600px; height:400px;"></div>



