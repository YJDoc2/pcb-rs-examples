<script>
  export let counter;
  export let CounterBinding;

  import {
    Column,
    Grid,
    Row,
    Tile,
    Toggle,
    Button,
  } from 'carbon-components-svelte';

  let oh_e = false;
  let oh_r = false;

  let jr_e = false;
  let jr_r = false;

  let oh_1 = '0';
  let oh_2 = '0';
  let oh_3 = '0';
  let oh_4 = '0';

  let jr_1 = '0';
  let jr_2 = '0';
  let jr_3 = '0';
  let jr_4 = '0';

  let set_text = () => {
    let _a = '';
    let _b = '';
    c = cin ? '1' : '0';
    _a += a3 ? '1' : '0';
    _a += a2 ? '1' : '0';
    _a += a1 ? '1' : '0';
    _a += a0 ? '1' : '0';
    _b += b3 ? '1' : '0';
    _b += b2 ? '1' : '0';
    _b += b1 ? '1' : '0';
    _b += b0 ? '1' : '0';
    a = _a;
    b = _b;
  };

  let oh_tick = () => {
    let b = CounterBinding.new();
    b.e = oh_e;
    b.r = oh_r;
    let r = counter.tick_one_hot(b);
    oh_1 = r.q1 ? '1' : '0';
    oh_2 = r.q2 ? '1' : '0';
    oh_3 = r.q3 ? '1' : '0';
    oh_4 = r.q4 ? '1' : '0';
  };
  let jr_tick = () => {
    let b = CounterBinding.new();
    b.e = jr_e;
    b.r = jr_r;
    let r = counter.tick_johnson_ring(b);
    jr_1 = r.q1 ? '1' : '0';
    jr_2 = r.q2 ? '1' : '0';
    jr_3 = r.q3 ? '1' : '0';
    jr_4 = r.q4 ? '1' : '0';
  };
</script>

<h1>Ring Counter Page</h1>
<br />
<Grid padding={true}>
  <Row>
    <Column>
      <Tile>
        <h2>One Hot Ring Counter</h2>
        <br />
        <Grid>
          <Row>
            <Column>
              <h3>{oh_1}</h3>
            </Column>
            <Column>
              <h3>{oh_2}</h3>
            </Column>
            <Column>
              <h3>{oh_3}</h3>
            </Column>
            <Column>
              <h3>{oh_4}</h3>
            </Column>
          </Row>
          <br />
          <hr />
          <br />
          <Row>
            <Column>
              <Toggle labelText="Enable" bind:toggled={oh_e} />
            </Column>
            <Column>
              <Toggle labelText="Reset" bind:toggled={oh_r} />
            </Column>
            <Column>
              <br />
              <Button on:click={oh_tick}>Tick</Button>
            </Column>
          </Row>
        </Grid>
      </Tile>
    </Column>
  </Row>
  <Row>
    <Column>
      <Tile>
        <h2>Johnson Ring Counter</h2>
        <br />
        <Grid>
          <Row>
            <Column>
              <h3>{jr_1}</h3>
            </Column>
            <Column>
              <h3>{jr_2}</h3>
            </Column>
            <Column>
              <h3>{jr_3}</h3>
            </Column>
            <Column>
              <h3>{jr_4}</h3>
            </Column>
          </Row>
          <br />
          <hr />
          <br />
          <Row>
            <Column>
              <Toggle labelText="Enable" bind:toggled={jr_e} />
            </Column>
            <Column>
              <Toggle labelText="Reset" bind:toggled={jr_r} />
            </Column>
            <Column>
              <br />
              <Button on:click={jr_tick}>Tick</Button>
            </Column>
          </Row>
        </Grid>
      </Tile>
    </Column>
  </Row>
</Grid>
