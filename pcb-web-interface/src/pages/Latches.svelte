<script>
  import {
    Button,
    Column,
    Grid,
    Row,
    Tile,
    Toggle,
  } from 'carbon-components-svelte';

  export let latches;
  export let bindings;

  let s = false;
  let r = false;
  let sr_q = false;
  let sr_nq = false;

  let gr = false;
  let gs = false;
  let gsr_e = false;
  let gsr_q = false;
  let gsr_nq = false;

  let d = false;
  let de = false;
  let d_q = false;
  let d_nq = false;

  let t = false;
  let te = false;
  let t_q = false;
  let t_nq = false;

  let j = false;
  let k = false;
  let jke = false;
  let jk_q = false;
  let jk_nq = false;

  let sr_tick = () => {
    let b = bindings.sr.new();
    b.s = s;
    b.r = r;
    let temp = latches.tick_sr(b);
    sr_q = temp.q;
    sr_nq = temp.notq;
  };
  let gsr_tick = () => {
    let b = bindings.gsr.new();
    b.s = gs;
    b.r = gr;
    b.e = gsr_e;
    let temp = latches.tick_gated_sr(b);
    gsr_q = temp.q;
    gsr_nq = temp.notq;
  };
  let d_tick = () => {
    let b = bindings.d.new();
    b.d = d;
    b.e = de;
    let temp = latches.tick_dlatch(b);
    d_q = temp.q;
    d_nq = temp.notq;
  };
  let t_tick = () => {
    let b = bindings.t.new();
    b.t = t;
    b.e = te;
    let temp = latches.tick_tlatch(b);
    t_q = temp.q;
    t_nq = temp.notq;
  };
  let jk_tick = () => {
    let b = bindings.jk.new();
    b.j = j;
    b.k = k;
    b.e = jke;
    let temp = latches.tick_jk(b);
    jk_q = temp.q;
    jk_nq = temp.notq;
  };

  // this must be done to get T latch in a usable state
  t_tick();
</script>

<h1>Latches</h1>
<br />

<Grid fullWidth={true}>
  <Row>
    <Column>
      <Tile>
        <h3>SR Latch</h3>
        <Grid>
          <Row>
            <Column>
              <Toggle labelText="S" bind:toggled={s} />
              <br />
              <Toggle labelText="R" bind:toggled={r} />
              <br />
              <br />
            </Column>
            <Column>
              <h6>Q</h6>
              <h4 class="output">{sr_q}</h4>
              <br />
              <h6>Not Q</h6>
              <h4 class="output">{sr_nq}</h4>
              <br />
              <Button kind="tertiary" on:click={sr_tick}>Tick</Button>
            </Column>
          </Row>
        </Grid>
      </Tile>
    </Column>
    <Column>
      <Tile>
        <h3>Gated SR Latch</h3>
        <Grid>
          <Row>
            <Column>
              <Toggle labelText="S" bind:toggled={gs} />
              <Toggle labelText="R" bind:toggled={gr} />
              <Toggle labelText="E" bind:toggled={gsr_e} />
            </Column>
            <Column>
              <h6>Q</h6>
              <h4 class="output">{gsr_q}</h4>
              <br />
              <h6>Not Q</h6>
              <h4 class="output">{gsr_nq}</h4>
              <Button kind="tertiary" on:click={gsr_tick}>Tick</Button>
            </Column>
          </Row>
        </Grid>
      </Tile>
    </Column>
  </Row>

  <Row>
    <Column>
      <Tile>
        <h3>Gated D Latch</h3>
        <Grid>
          <Row>
            <Column>
              <Toggle labelText="D" bind:toggled={d} />
              <br />
              <Toggle labelText="E" bind:toggled={de} />
            </Column>
            <Column>
              <h6>Q</h6>
              <h4 class="output">{d_q}</h4>
              <br />
              <h6>Not Q</h6>
              <h4 class="output">{d_nq}</h4>
              <Button kind="tertiary" on:click={d_tick}>Tick</Button>
            </Column>
          </Row>
        </Grid>
      </Tile>
    </Column>
    <Column>
      <Tile>
        <h3>Gated T Latch</h3>
        <Grid>
          <Row>
            <Column>
              <Toggle labelText="T" bind:toggled={t} />
              <br />
              <Toggle labelText="E" bind:toggled={te} />
            </Column>
            <Column>
              <h6>Q</h6>
              <h4 class="output">{t_q}</h4>
              <br />
              <h6>Not Q</h6>
              <h4 class="output">{t_nq}</h4>
              <Button kind="tertiary" on:click={t_tick}>Tick</Button>
            </Column>
          </Row>
        </Grid>
      </Tile>
    </Column>
  </Row>

  <Row padding={true}>
    <Column />
    <Column>
      <Tile>
        <h3>Gated JK Latch</h3>
        <Grid>
          <Row>
            <Column>
              <Toggle labelText="J" bind:toggled={j} />
              <Toggle labelText="K" bind:toggled={k} />
              <Toggle labelText="E" bind:toggled={jke} />
            </Column>
            <Column>
              <h6>Q</h6>
              <h4 class="output">{jk_q}</h4>
              <br />
              <h6>Not Q</h6>
              <h4 class="output">{jk_nq}</h4>
              <br />
              <Button kind="tertiary" on:click={jk_tick}>Tick</Button>
            </Column>
          </Row>
        </Grid>
      </Tile>
    </Column>
    <Column />
  </Row>
</Grid>

<style>
  .output::first-letter {
    text-transform: capitalize;
  }
</style>
