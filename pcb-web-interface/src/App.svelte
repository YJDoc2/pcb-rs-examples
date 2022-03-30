<script>
  export let bindings;
  import { HOME_PAGE, RING_COUNTER_PAGE } from './constants';
  import Nav from './Nav.svelte';
  import Content from './Content.svelte';

  let page = RING_COUNTER_PAGE;
</script>

<Nav bind:page />
<Content {page} {bindings} />
