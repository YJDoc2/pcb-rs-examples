<script>
  import {
    Header,
    SideNav,
    SideNavItems,
    SideNavLink,
    SkipToContent,
  } from 'carbon-components-svelte';
  import * as constants from './constants.js';

  export let page = constants.HOME_PAGE;

  let isSideNavOpen = false;
</script>

<Header company="PCB-rs" platformName="Web Interface" bind:isSideNavOpen />
<SideNav bind:isOpen={isSideNavOpen}>
  <SideNavItems>
    <SideNavLink
      text="Home"
      selected={true}
      on:click={() => {
        page = constants.HOME_PAGE;
      }}
    />
    <SideNavLink
      text="Info"
      on:click={() => {
        page = constants.INFO_PAGE;
      }}
    />
    <SideNavLink
      text="Gates"
      on:click={() => {
        page = constants.GATES_PAGE;
      }}
    />
    <SideNavLink
      text="CPU PCB"
      on:click={() => {
        page = constants.CPU_PAGE;
      }}
    />
    <SideNavLink
      text="Adder"
      on:click={() => {
        page = constants.ADDER_PAGE;
      }}
    />
    <SideNavLink
      text="Ring Counter"
      on:click={() => {
        page = constants.RING_COUNTER_PAGE;
      }}
    />
  </SideNavItems>
</SideNav>
