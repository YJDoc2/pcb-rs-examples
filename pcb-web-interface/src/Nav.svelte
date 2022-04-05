<script>
  import {
    Header,
    SideNav,
    SideNavItems,
    SideNavLink,
  } from 'carbon-components-svelte';
  import * as constants from './constants.js';

  export let page = constants.HOME_PAGE;

  let isSideNavOpen = false;
</script>

<Header company="PCB-RS" platformName="Web Interface" bind:isSideNavOpen />
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
      text="Ripple Adder"
      on:click={() => {
        page = constants.RIPPLE_ADDER_PAGE;
      }}
    />
    <SideNavLink
      text="Carry Look Ahead Adder"
      on:click={() => {
        page = constants.CLA_ADDER_PAGE;
      }}
    />
    <SideNavLink
      text="Latches"
      on:click={() => {
        page = constants.LATCHES_PAGE;
      }}
    />
    <SideNavLink
      text="Ring Counter"
      on:click={() => {
        page = constants.RING_COUNTER_PAGE;
      }}
    />
    <SideNavLink
      text="CPU PCB"
      on:click={() => {
        page = constants.CPU_PAGE;
      }}
    />
  </SideNavItems>
</SideNav>
