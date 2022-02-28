<script>
  export let bindings;
  import { HOME_PAGE, CPU_PAGE } from './constants';
  import Nav from './Nav.svelte';
  import Content from './Content.svelte';

  let page = CPU_PAGE;
</script>

<Nav bind:page />
<Content {page} {bindings} />
