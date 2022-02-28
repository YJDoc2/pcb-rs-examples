<script>
  import { Grid, Row, Column } from 'carbon-components-svelte';
  import { Tile, TextArea, Button } from 'carbon-components-svelte';
  import compile from './compiler';
  import { onMount } from 'svelte';

  export let pcb;

  let compileErr = false; // need this for textarea
  let compileErrText = '';
  let code = '';
  let disabled = false;
  let compiled = false;

  let addr_bus = 0;
  let data_bus = 'Tristated';
  let mem_active = false;
  let read_mem = true;
  let io_latch = false;
  let cpu_state = 'Idle';

  let reg1 = 5;
  let reg2 = 7;
  let instr_ptr = 2;
  let zero = false;
  let lt = true;
  let gt = false;
  let instr_cache = [5, 6, 7, 8, 9, 10, 11, 12, 13];

  let ram_addr = 0;
  let ram_data = 'Tristated';
  let ram_active = false;
  let ram_read = true;
  let ram_latch = false;

  let mem = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1,
  ];

  onMount(() => {
    set_state();
  });

  function load() {
    try {
      let asm = compile(code);
      pcb.load(asm);
      compiled = true;
      compileErr = false;
      compileErrText = '';
      set_state();
    } catch (err) {
      compileErr = true;
      compileErrText = err;
      disabled = true;
      console.log(err);
    }
  }

  function tick() {
    if (!compiled) {
      return;
    }
    // disable next button in case, but this might trigger re-render,
    // so would be better is svelte by-default de-bounces clicks
    disabled = true;
    pcb.tick();
    set_state();
    disabled = false;
  }

  function set_state() {
    cpu_state = pcb.get_cpu_state();
    let cpu_reg_flags = pcb.get_cpu_reg_flags();
    addr_bus = cpu_reg_flags.addr;
    data_bus = cpu_reg_flags.data || 'Tristated';
    mem_active = cpu_reg_flags.mem_active;
    read_mem = cpu_reg_flags.read_mem;
    io_latch = cpu_reg_flags.io_latch;

    reg1 = cpu_reg_flags.reg1;
    reg2 = cpu_reg_flags.reg2;
    instr_ptr = cpu_reg_flags.instr_ptr;
    zero = cpu_reg_flags.zero;
    lt = cpu_reg_flags.lt;
    gt = cpu_reg_flags.gt;
    instr_cache = pcb.get_cpu_instr_cache();

    let ram_state = pcb.get_ram_state();

    ram_addr = ram_state.addr;
    ram_data = ram_state.data || 'Tristated';
    ram_active = ram_state.is_active;
    ram_read = ram_state.is_read;
    ram_latch = ram_state.io_latch;

    mem = pcb.get_mem_array();
  }
</script>

<Grid padding={false}>
  <Row>
    <Column>
      <Tile>
        <h3>CPU</h3>
        <div class="row-separator" />
        <Grid padding={false}>
          <Row>
            <Column>
              <h5>Address Bus : {addr_bus}</h5>
              <hr />
              <h5>Data Bus : {data_bus}</h5>
              <hr />
              <h5>Memory Select : {mem_active}</h5>
              <hr />
              <h5>Memory Read : {read_mem}</h5>
              <hr />
              <h5>IO Latch : {io_latch}</h5>
              <hr />
              <h5>CPU State : {cpu_state}</h5>
            </Column>
            <Column>
              <h5>Register A : {reg1}</h5>
              <hr />
              <h5>Register B : {reg2}</h5>
              <hr />
              <h5>Instruction Pointer : {instr_ptr}</h5>
              <hr />
              <h5>Zero Flag : {zero}</h5>
              <hr />
              <h5>LessThan Flag : {lt}</h5>
              <hr />
              <h5>GreateThan Flag : {gt}</h5>
            </Column>
          </Row>
          <Row><div class="row-separator" /></Row>
          <Row>
            <Column>
              <h4>Instruction Cache : [ {instr_cache} ]</h4>
            </Column>
          </Row>
        </Grid>
        <div class="row-separator" />
        <h3>Memory</h3>
        <div class="row-separator" />
        <Grid>
          <Row><div class="row-separator-thin" /></Row>
          <Row>
            <Column>
              <h5>Chip Select : {ram_active}</h5>
            </Column>
            <Column><h5>Address Bus : {ram_addr}</h5></Column>
            <Column><h5>Data Bus : {ram_data}</h5></Column>
          </Row>
          <Row>
            <div class="row-separator-thin" />
          </Row>
          <Row>
            <Column><h5>Access State : {ram_read}</h5></Column>
            <Column />
            <Column><h5>IO Latch : {ram_latch}</h5></Column>
          </Row>
        </Grid>
        <div class="row-separator" />
        {#each Array(16) as _, i}
          {#each Array(16) as _, j}
            <div class="cell">{String(mem[i * 16 + j]).padStart(3, '0')}</div>
          {/each}
          <br />
          <div class="row-separator" />
        {/each}
      </Tile>
    </Column>
    <Column>
      <TextArea
        rows={25}
        invalid={compileErr}
        invalidText={compileErrText}
        bind:value={code}
      />
      <br />
      <Button size="field" kind="danger" on:click={load}>Load</Button>
      <Button
        size="field"
        kind="tertiary"
        disabled={disabled || !compiled}
        on:click={tick}>Next</Button
      >
    </Column>
  </Row>
</Grid>

<style>
  hr {
    border: none;
    height: 1px;
    background-color: #333;
  }
  div.row-separator {
    margin: 0.5rem;
  }
  div.row-separator-thin {
    margin: 0.3rem;
  }
  div.cell {
    display: inline;
    margin: 0rem 0.5rem;
    padding: 0rem 0.5rem;
    border: solid black;
    border-width: 0 1px 1px 0;
  }
</style>
