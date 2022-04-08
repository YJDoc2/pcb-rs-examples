<script>
  import { Content } from 'carbon-components-svelte';
  import * as constants from './constants';
  import CPU from './pages/CPU.svelte';
  import BasicGates from './pages/BasicGates.svelte';
  import RippleAdder from './pages/RippleAdder.svelte';
  import CLAAdder from './pages/CLAAdder.svelte';
  import Latches from './pages/Latches.svelte';
  import RingCounter from './pages/RingCounter.svelte';
  import HomePage from './pages/HomePage.svelte';
  import InfoPage from './pages/InfoPage.svelte';

  export let page;
  export let bindings;

  let pcb = bindings.get_pcb_handle();

  let gates = bindings.get_gates_handle();

  let GatesBinding = bindings.GatesBinding;

  let rippleAdder = bindings.get_ripple_adder_handle();
  let RippleAdderBinding = bindings.RippleAdderBinding;

  let claAdder = bindings.get_cla_adder_handle();
  let CLAAdderBinding = bindings.CLAAdderBinding;

  let latches = bindings.get_latch_handle();
  let latch_bindings = {
    sr: bindings.SRBinding,
    gsr: bindings.GatedSRBinding,
    d: bindings.DBinding,
    t: bindings.TBinding,
    jk: bindings.JKBinding,
  };
  let counter = bindings.get_counter_handle();
  let CounterBinding = bindings.CounterBinding;
</script>

<Content>
  {#if page == constants.HOME_PAGE}
    <HomePage />
  {:else if page == constants.INFO_PAGE}
    <InfoPage />
  {:else if page == constants.GATES_PAGE}
    <BasicGates {gates} {GatesBinding} />
  {:else if page == constants.RIPPLE_ADDER_PAGE}
    <RippleAdder {rippleAdder} {RippleAdderBinding} />
  {:else if page == constants.CLA_ADDER_PAGE}
    <CLAAdder {claAdder} {CLAAdderBinding} />
  {:else if page == constants.LATCHES_PAGE}
    <Latches {latches} bindings={latch_bindings} />
  {:else if page == constants.RING_COUNTER_PAGE}
    <RingCounter {counter} {CounterBinding} />
  {:else if page == constants.CPU_PAGE}
    <CPU {pcb} />
  {:else}
    <h1>unicorn page</h1>
  {/if}
</Content>
