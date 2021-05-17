<script lang="ts">
  import websocketStore from 'svelte-websocket-store';

  import Round from './Round.svelte';
  import { _, locale } from './i18n';

  let requestedLocale = window.location.hash.slice(1).startsWith('lang=')
    ? window.location.hash.slice(6)
    : undefined;
  let userLocale = requestedLocale ?? locale;

  let gameState;
  try {
    let wsUri = `${window.location.protocol === 'https:' ? 'wss' : 'ws'}://${
      window.location.host
    }/${userLocale.toLowerCase()}`;
    gameState = websocketStore(wsUri, {});
  } catch (e) {
    error = e.toString();
    throw new Error(e);
  }
  console.log('connected as', userLocale);

  // gameState.subscribe(gs => {
  //   console.log(gs);
  // });

  let main;

  let scrolling = false;
  let cards: { mine: Card; theirs: Card };
  let nextCards: { mine: Card; theirs: Card };
  let error: $gameState.error;
  $: ready = $gameState.ready ?? false;
  $: {
    if ($gameState.myCard !== undefined) {
      if (typeof cards === 'undefined') {
        cards = {
          mine: $gameState.myCard,
          theirs: $gameState.theirCard,
        };
      } else if (cards?.mine?.word !== $gameState.myCard.word) {
        nextCards = {
          mine: $gameState.myCard,
          theirs: $gameState.theirCard,
        };
        scrolling = true;
        setTimeout(() => {
          cards = nextCards;
        }, 500);
        setTimeout(() => {
          nextCards = undefined;
          scrolling = false;
        }, 1000);
      }
    }
  }

  function advanceRound() {
    ready = true;
    gameState.set({ action: 'advance-round' });
  }

  function setScore({ detail: score }) {
    gameState.set({ score });
  }
</script>

<main bind:this={main} class:scrolling>
  {#if error}
    <div class="error">
      <h1>Error: {error}</h1>
    </div>
  {:else}
    {#if cards}
      <Round
        class="round"
        myCard={cards.mine}
        theirCard={cards.theirs}
        hasReceivedScore={$gameState.hasScore ?? false}
        scored={$gameState.scored}
        {ready}
        on:setScore={setScore}
        on:advanceRound={advanceRound}
      />
    {/if}
    {#if nextCards}
      <Round class="round" myCard={nextCards.mine} theirCard={nextCards.theirs} />
    {/if}
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

  .error {
    width: 100vw;
    text-align: center;
  }

  :global(.scrolling *) {
    transition: 0s !important;
  }

  :global(.round) {
    width: 100vw;
    float: left;
  }
</style>
