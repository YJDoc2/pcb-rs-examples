<script>
  export let bindings;
  import {
    Header,
    SideNav,
    SideNavItems,
    SideNavLink,
    SkipToContent,
    Content,
    Grid,
    Row,
    Column,
  } from 'carbon-components-svelte';

  let isSideNavOpen = false;
</script>

<Header company="PCB-rs" platformName="Web Interface" bind:isSideNavOpen>
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
</Header>
<SideNav bind:isOpen={isSideNavOpen}>
  <SideNavItems>
    <SideNavLink text="CPU PCB" />
    <SideNavLink text="Adder" />
    <SideNavLink text="Gates" />
    <SideNavLink text="Ring Counter" />
  </SideNavItems>
</SideNav>

<Content>
  <Grid>
    <Row>
      <Column>
        <h1>PCB RS</h1>
      </Column>
    </Row>
  </Grid>
</Content>
