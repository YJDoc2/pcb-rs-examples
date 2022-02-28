<script>
  export let bindings;
  import { HOME_PAGE } from './constants';
  import Nav from './Nav.svelte';
  import Content from './Content.svelte';

  let page = HOME_PAGE;
</script>

<Nav bind:page />
<Content {page} {bindings} />
