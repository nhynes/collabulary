<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	import { _ } from './i18n';

	export let score: number | undefined = undefined;
	export let disabled = false;

	const scoreDescriptions = [];
	for (let i = 0; i <= 5; i++) {
		scoreDescriptions.push($_('score_' + i));
	}
</script>

<form action="javascript:void(0)">
	<h2>{$_('score')}</h2>

	{#each scoreDescriptions as desc, i}
		<button
			class:selected="{score === i}"
			title="{desc}"
			{disabled}
			on:click="{() => (score = i)}"
		>
			{i}
		</button>
	{/each}
</form>

<style>
	form {
		margin: 30px 0;
	}

	button {
		width: 40px;
		height: 40px;
		margin: 0 4px;
	}

	button.selected {
		border-width: 2px;
		border-color: #666;
	}
</style>
