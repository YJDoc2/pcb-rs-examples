<script>
  export let rippleAdder;
  export let RippleAdderBinding;

  import {
    Column,
    Grid,
    Row,
    Tile,
    Toggle,
    Button,
  } from 'carbon-components-svelte';

  let a0 = false;
  let a1 = false;
  let a2 = false;
  let a3 = false;

  let a = '0000';

  let b0 = false;
  let b1 = false;
  let b2 = false;
  let b3 = false;

  let b = '0000';

  let cin = false;
  let c = '0';

  let sum = '0000';

  let carry = '0';

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

  let tick = () => {
    let t = RippleAdderBinding.new();

    t.a0 = a0;
    t.a1 = a1;
    t.a2 = a2;
    t.a3 = a3;

    t.b0 = b0;
    t.b1 = b1;
    t.b2 = b2;
    t.b3 = b3;

    t.cin = cin;

    let o = rippleAdder.tick(t);
    let _s = '';
    _s += o.s3 ? '1' : '0';
    _s += o.s2 ? '1' : '0';
    _s += o.s1 ? '1' : '0';
    _s += o.s0 ? '1' : '0';
    sum = _s;
    carry = o.carry ? '1' : '0';
  };
</script>

<h1>Ripple Adder Page</h1>
<br />
<Tile>
  <Grid fullWidth={true} padding={true}>
    <Row padding={true}>
      <Column>
        <Grid fullWidth={true} padding={true}>
          <Row padding={true}>
            <Column>
              <h3>Input</h3>
              <br />
              <Grid fullWidth={true} padding={true}>
                <Row padding={true}>
                  <Column>
                    <Toggle
                      labelText="A3"
                      bind:toggled={a3}
                      on:toggle={set_text}
                    />
                  </Column>
                  <Column>
                    <Toggle
                      labelText="A2"
                      bind:toggled={a2}
                      on:toggle={set_text}
                    />
                  </Column>
                  <Column>
                    <Toggle
                      labelText="A1"
                      bind:toggled={a1}
                      on:toggle={set_text}
                    />
                  </Column>
                  <Column>
                    <Toggle
                      labelText="A0"
                      bind:toggled={a0}
                      on:toggle={set_text}
                    />
                  </Column>
                </Row>
                <Row>
                  <Column />
                  <Column>
                    <h2>{a}</h2>
                  </Column>
                  <Column />
                </Row>
                <Row padding={true}>
                  <Column>
                    <Toggle
                      labelText="B3"
                      bind:toggled={b3}
                      on:toggle={set_text}
                    />
                  </Column>
                  <Column>
                    <Toggle
                      labelText="B2"
                      bind:toggled={b2}
                      on:toggle={set_text}
                    />
                  </Column>
                  <Column>
                    <Toggle
                      labelText="B1"
                      bind:toggled={b1}
                      on:toggle={set_text}
                    />
                  </Column>
                  <Column>
                    <Toggle
                      labelText="B0"
                      bind:toggled={b0}
                      on:toggle={set_text}
                    />
                  </Column>
                </Row>
                <Row>
                  <Column />
                  <Column>
                    <h2>{b}</h2>
                  </Column>
                  <Column />
                </Row>
                <Row padding={true}>
                  <Column>
                    <Toggle
                      labelText="Cin"
                      bind:toggled={cin}
                      on:toggle={set_text}
                    />
                    <h3>{c}</h3>
                  </Column>
                </Row>
              </Grid>
            </Column>
            <Column>
              <h3>Output</h3>
              <br />
              <h4>Sum : {sum}</h4>
              <h4>Carry : {carry}</h4>
            </Column>
          </Row>
        </Grid>
      </Column>
    </Row>
    <Row padding={true}>
      <Column>
        <Button on:click={tick}>Tick</Button>
      </Column>
    </Row>
  </Grid>
</Tile>
