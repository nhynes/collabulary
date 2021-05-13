<script lang="ts" context="module">
	export type Word = {
		word: string;
		wordDetail: string;
		definition: string | string[];
		translation: string;
	};
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	import Card, { Side as CardSide } from './Card.svelte';
	import Scorer from './Scorer.svelte';
	import NextRound from './NextRound.svelte';
	import { _, locale } from './i18n';

	export let clazz = '';
	export let myWord: Word;
	export let theirWord: Word;
	export { clazz as class };

	const dispatch = createEventDispatcher();

	let score;
	let submittedScore = false;

	function advanceRound() {
		submittedScore = true;
		dispatch('nextRound');
	}
</script>

<div class="tasks {clazz}">
	<div class="task">
		<h1>{$_('your_word')}</h1>
		<Card {...myWord} />
	</div>

	<NextRound selectable={score !== undefined} on:click={advanceRound} />

	<div class="task">
		<h1>{$_('their_word')}</h1>
		<Card {...theirWord} scored={false} />
		<Scorer bind:score disabled={submittedScore} />
	</div>
</div>

<style>
	.tasks {
		text-align: center;
		margin: auto;
	}

	.task {
		margin: 50px 0;
	}

	.task:first-of-type {
		margin-top: 0;
	}

	.task:last-of-type {
		margin-bottom: 0;
	}
</style>
