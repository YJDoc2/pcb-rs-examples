<script>
  import { Tile } from 'carbon-components-svelte';
</script>

<Tile>
  <h1>PCB RS</h1>
  <br />
  <h4>
    This is the web interface for pcb-rs library examples. These are to show-off
    the strengths and usage of the <a
      href="https://www.github.com/YJDoc2/pcb-rs"
      target="_blank"
      rel="noreferrer">pcb-rs</a
    > library.
  </h4>
  <br />
  <h4>
    pcb-rs is a library, which helps in easily witing Software Emulated Hardware
    in a modular way in Rust. It provides two macros to achive this, and several
    traits to aid in the modularity.
  </h4>
  <br />
  <h4>
    Writing a chip is as easy as writing a struct and annotating its members as
    pins, then implementing a trait function to implement its logic.
  </h4>
  <br />
  <h4>
    You can easily interface several chips, written by you or others by using
    pcb! macro. You just have to list the chips, give the pin connections and
    write which pins you want to be exposed, and the macro will derive the rest!
    This includes the logic to validate chips, the connections of pins you have
    written and the tick-ing logic for the pcb.
  </h4>
  <br />
  <h4>Furthermore, you can use this pcb as a chip in other pcbs!</h4>

  <br />
  <h4>
    Checl out the <a
      href="https://www.github.com/YJDoc2/pcb-rs"
      target="_blank"
      rel="noreferrer">pcb-rs</a
    > github page for more details on the library.
  </h4>
  <br />
  <h4>
    This site contains:
    <br />
    <ul>
      <li>- Basic Gates</li>
      <li>- Ripple Adder</li>
      <li>- Carry Look Ahead Adder</li>
      <li>- Latches</li>
      <li>- Ring Counter</li>
      <li>- CPU-RAM PCB</li>
    </ul>
  </h4>
  <br />
  <h4>
    Also check out the info page for detailed information on each example.
  </h4>
  <br />
  <h4>
    Thanks for checking this! If you like this, or find this interesting, please
    check and start the github repo!
  </h4>
</Tile>
