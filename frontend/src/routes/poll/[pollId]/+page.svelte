<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from "svelte";
	import * as echarts from 'echarts';
	import {option} from "./chart";
	import { nanoid } from 'nanoid'

	let poll;

	function setCookie() {
		document.cookie = `buzzpoll=${nanoid()}`;
  }



	onMount(async function () {
		setCookie();

		const endpoint = "http://localhost:8080/poll/" + $page.params.pollId;
    var myChart = echarts.init(document.getElementById('main'));

    const response = await fetch(endpoint);
		poll  = await response.json();
		option.yAxis.data = poll.answers.map(answer => answer.text);
		myChart.setOption(option);

		const socket = new WebSocket('ws://127.0.0.1:8080/ws/');

		socket.addEventListener('open', function (event) {
			console.log("It's open");
		});

		socket.addEventListener('message', function (event: any) {
			if (!event.data) {
				return
			}
      const obj = JSON.parse(event.data);
			option.series[0].data = obj.answers.map(answer => answer.votes.length );
			myChart.setOption(option);
		});
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



