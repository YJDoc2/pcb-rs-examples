<script>
  export let bindings;
  import { HOME_PAGE, RIPPLE_ADDER_PAGE } from './constants';
  import Nav from './Nav.svelte';
  import Content from './Content.svelte';

  let page = RIPPLE_ADDER_PAGE;
</script>

<Nav bind:page />
<Content {page} {bindings} />
