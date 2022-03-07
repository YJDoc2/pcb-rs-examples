<script>
  import { Content } from 'carbon-components-svelte';
  import * as constants from './constants';
  import CPU from './pages/CPU.svelte';
  import BasicGates from './pages/BasicGates.svelte';
  import RippleAdder from './pages/RippleAdder.svelte';

  export let page;
  export let bindings;
  let pcb = bindings.get_pcb_handle();
  let gates = bindings.get_gates_handle();
  let GatesBinding = bindings.GatesBinding;
  let rippleAdder = bindings.get_ripple_adder_handle();
  let RippleAdderBinding = bindings.RippleAdderBinding;
</script>

<Content>
  {#if page == constants.HOME_PAGE}
    <h1>Home Page</h1>
  {:else if page == constants.INFO_PAGE}
    <h1>Info Page</h1>
  {:else if page == constants.GATES_PAGE}
    <BasicGates {gates} {GatesBinding} />
  {:else if page == constants.CPU_PAGE}
    <CPU {pcb} />
  {:else if page == constants.RIPPLE_ADDER_PAGE}
    <RippleAdder {rippleAdder} {RippleAdderBinding} />
  {:else if page == constants.RING_COUNTER_PAGE}
    <h1>Ring Counter page</h1>
  {:else}
    <h1>unicorn page</h1>
  {/if}
</Content>
