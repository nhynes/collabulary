<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { _ } from './i18n';

  const dispatch = createEventDispatcher();

  export let score: number | undefined = undefined;
  export let disabled = false;

  const scoreDescriptions = [];
  for (let i = 0; i <= 5; i++) {
    scoreDescriptions.push($_('score_' + i));
  }

  function selectScore(score_) {
    if (score === score_) return;
    score = score_;
    dispatch('score', score);
  }
</script>

<form action="javascript:void(0)">
  <h2>{$_('score')}</h2>

  {#each scoreDescriptions as desc, i}
    <button
      class:selected={score === i}
      title={desc}
      {disabled}
      on:click={() => selectScore(i)}
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
