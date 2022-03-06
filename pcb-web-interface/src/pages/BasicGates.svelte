<script>
  import { Column, Grid, Row, Tile, Toggle } from 'carbon-components-svelte';

  export let gates;
  export let GatesBinding;

  let not_in = false;
  let not_out = !not_in;

  let and_in1 = false;
  let and_in2 = false;
  let and_out = false;

  let or_in1 = false;
  let or_in2 = false;
  let or_out = false;

  let xor_in1 = false;
  let xor_in2 = false;
  let xor_out = false;

  let tick = () => {
    let t = GatesBinding.new();

    t.not_in = not_in;
    t.and_in1 = and_in1;
    t.and_in2 = and_in2;
    t.or_in1 = or_in1;
    t.or_in2 = or_in2;
    t.xor_in1 = xor_in1;
    t.xor_in2 = xor_in2;

    let o = gates.tick(t);

    not_out = o.not_out;
    and_out = o.and_out;
    or_out = o.or_out;
    xor_out = o.xor_out;
  };
</script>

<h1>Basic Logic Gates</h1>
<br />

<Tile>
  <Grid fullWidth={true} padding={true}>
    <Row padding={true}>
      <Column padding={true}>
        <h3>Not Gate</h3>
        <br />
        <Grid>
          <Row>
            <Column>
              <Toggle
                labelText="Input"
                bind:toggled={not_in}
                on:toggle={tick}
              />
            </Column>
            <Column>
              <h6>Output</h6>
              <h3 class="output">{not_out}</h3>
            </Column>
          </Row>
        </Grid>
      </Column>
      <Column padding={true}>
        <h3>And Gate</h3>
        <br />
        <Grid>
          <Row>
            <Column>
              <Toggle
                labelText="Input 1"
                bind:toggled={and_in1}
                on:toggle={tick}
              />
              <Toggle
                labelText="Input 2"
                bind:toggled={and_in2}
                on:toggle={tick}
              />
            </Column>
            <Column>
              <h6>Output</h6>
              <h3 class="output">{and_out}</h3>
            </Column>
          </Row>
        </Grid>
      </Column>
    </Row>

    <Row padding={true}>
      <Column padding={true}>
        <h3>Or Gate</h3>
        <br />
        <Grid>
          <Row>
            <Column>
              <Toggle
                labelText="Input 1"
                bind:toggled={or_in1}
                on:toggle={tick}
              />
              <Toggle
                labelText="Input 2"
                bind:toggled={or_in2}
                on:toggle={tick}
              />
            </Column>
            <Column>
              <h6>Output</h6>
              <h3 class="output">{or_out}</h3>
            </Column>
          </Row>
        </Grid>
      </Column>
      <Column padding={true}>
        <h3>Xor Gate</h3>
        <br />
        <Grid>
          <Row>
            <Column>
              <Toggle
                labelText="Input 1"
                bind:toggled={xor_in1}
                on:toggle={tick}
              />
              <Toggle
                labelText="Input 2"
                bind:toggled={xor_in2}
                on:toggle={tick}
              />
            </Column>
            <Column>
              <h6>Output</h6>
              <h3 class="output">{xor_out}</h3>
            </Column>
          </Row>
        </Grid>
      </Column>
    </Row>
  </Grid>
</Tile>

<style>
  .output::first-letter {
    text-transform: capitalize;
  }
</style>
