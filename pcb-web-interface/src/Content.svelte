<script>
  import { Content } from 'carbon-components-svelte';
  import * as constants from './constants';

  export let page;
  export let bindings;
</script>

<Content>
  {#if page == constants.HOME_PAGE}
    <h1>Home Page</h1>
  {:else if page == constants.INFO_PAGE}
    <h1>Info Page</h1>
  {:else if page == constants.CPU_PAGE}
    <h1>CPU page</h1>
  {:else if page == constants.ADDER_PAGE}
    <h1>Adder page</h1>
  {:else if page == constants.GATES_PAGE}
    <h1>Gates page</h1>
  {:else if page == constants.RING_COUNTER_PAGE}
    <h1>Ring Counter page</h1>
  {:else}
    <h1>unicorn page</h1>
  {/if}
</Content>
