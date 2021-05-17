<script lang="ts" context="module">
  export type Card = {
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
  export let myCard: Card;
  export let theirCard: Card;

  export let hasReceivedScore = false;

  // Saved state from previous session.
  export let scored: number | undefined = undefined;
  export let ready: boolean;

  export { clazz as class };

  $: score = scored;

  const dispatch = createEventDispatcher();

  function advanceRound() {
    dispatch('advanceRound');
  }

  function setScore() {
    dispatch('setScore', score);
  }
</script>

<div class="tasks {clazz}">
  <div class="task">
    <h1>{$_('your_word')}</h1>
    <Card {...myCard} scored={!hasReceivedScore} />
  </div>

  <NextRound visible={score !== undefined} selected={ready} on:click={advanceRound} />

  <div class="task">
    <h1>{$_('their_word')}</h1>
    <Card {...theirCard} scored={false} />
    <Scorer bind:score on:score={setScore} disabled={ready} />
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
