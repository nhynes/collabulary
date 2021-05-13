<script lang="ts">
	import { _, locale } from './i18n';

	import Round from './Round.svelte';

	let theirWord = {
		word: '与',
		wordDetail: 'yǔ',
		definition: '(same as 欤, final particle expression doubt or surprise, similar to 吗 or 呢); and/to give/together with; take part in'.split(
			'/',
		),
		side: 'definition',
	};

	let myWord = {
		word: '你',
		wordDetail: 'nǐ',
		definition: 'you',
		side: 'word',
	};

	let main;
	let myNextWord = theirWord;
	let theirNextWord = myWord;
	let scrolling = false;

	function waitForNextRound() {
		setTimeout(() => {
			scrolling = true;
			setTimeout(() => {
				myWord = myNextWord;
				theirWord = theirNextWord;
			}, 500);
			setTimeout(() => {
				myNextWord = undefined;
				theirNextWord = undefined;
				scrolling = false;
			}, 1000);
		}, 2500);
	}
</script>

<main bind:this={main} class:scrolling="{scrolling}">
	{#key myWord}
		<Round class="round" {myWord} {theirWord} on:nextRound={waitForNextRound} />
	{/key}
	{#if myNextWord && theirNextWord}
		<Round class="round" myWord={myNextWord} theirWord={theirNextWord} />
	{/if}
</main>

<style>
	main {
		width: 200vw;
		display: inline-block;
	}

	main.scrolling {
		transition: transform 1s;
		transform: translateX(-100vw);
	}

	:global(.scrolling *) {
		transition: 0s !important;
	}

	:global(.round) {
		width: 100vw;
		float: left;
	}
</style>
