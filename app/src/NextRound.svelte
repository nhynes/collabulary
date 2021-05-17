<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Moon as Spinner } from 'svelte-loading-spinners';

  export let visible;
  export let selected;

  const dispatch = createEventDispatcher();

  function advanceRound() {
    dispatch('click');
  }
</script>

<div class="next-round" class:visible>
  <hr />
  <button disabled={selected} on:click={advanceRound}>
    {#if selected}
      <Spinner size={30} color="green" duration="1.5s" />
    {:else}
      <span class="arrow">&rarr;</span>
    {/if}
  </button>
</div>

<style>
  .next-round {
    height: 52px;

    display: flex;
    align-items: center;
    justify-content: center;
  }

  .next-round:not(.visible) button {
    width: 0;
    height: 0;
    border: 0;
    box-shadow: unset;
    background-color: transparent;
  }

  .next-round:not(.visible) .arrow {
    font-size: 0;
  }

  hr {
    width: 80vw;
    max-width: 600px;
  }

  button {
    display: flex;
    align-items: center;
    justify-content: center;
    position: absolute;
    width: 50px;
    height: 50px;
    border-radius: 50px;
    transition: 0.2s;
    background-color: white;
    border-width: 2px;
    box-shadow: rgba(0, 0, 0, 0.4) 0px 6px 15px;
    transform: translateY(-2px);
  }

  button:active:enabled {
    box-shadow: rgba(0, 0, 0, 0.35) 0px 3px 10px;
    transform: translateY(2px);
    transition: 0.1s;
    background-color: white;
  }

  button:disabled {
    box-shadow: rgba(0, 0, 0, 0.35) 0px 3px 10px;
    transform: translateY(2px);
  }

  .arrow {
    transition: 0.2s;
    font-size: 30px;
    font-weight: 600;
    color: #555;
  }
</style>
