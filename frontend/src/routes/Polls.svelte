<script lang="ts">
    import { onMount } from "svelte";
  import { qr } from '@svelte-put/qr/svg';
  import type { Poll } from "./model.svelte";

  const endpoint = "http://localhost:8080/poll";
  let polls: Poll[] = [];

  onMount(async function () {
    const response = await fetch(endpoint);
    const data  = await response.json();
    polls = data.map(poll => ({...poll, link: `poll/${poll.id}/results`}));
  });
</script>

{#each polls as poll}
  <a href="{ `poll/${poll.id}`}">
  <div class="max-w-sm rounded overflow-hidden shadow-lg">
    <div class="px-6 py-4">
      <div class="font-bold text-xl mb-2">{ poll.question }</div>
      <p class="text-gray-700 text-base">
        Lorem ipsum dolor sit amet, consectetur adipisicing elit. Voluptatibus quia, nulla! Maiores et perferendis eaque, exercitationem praesentium nihil.
      </p>
    </div>

    <svg use:qr={{
      data: poll.link,
      logo: 'https://svelte-put.vnphanquang.com/images/svelte-put-logo.svg',
      shape: 'circle',
      }}
    />
  </div>
  </a>
{/each}


