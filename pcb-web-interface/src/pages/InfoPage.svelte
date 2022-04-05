<script>
  import { Tile, Link } from 'carbon-components-svelte';
  import { Tabs, Tab, TabContent } from 'carbon-components-svelte';
  import { intros } from 'svelte/internal';
</script>

<Tile>
  <h1>Info Page</h1>
  <br />
  <Tabs>
    <Tab label="Basic Gates" />
    <Tab label="Ripple Adder" />
    <Tab label="Carry Look Ahead Adder" />
    <Tab label="Latches" />
    <Tab label="Ring Counter" />
    <Tab label="CPU-RAM PCB" />
    <svelte:fragment slot="content">
      <TabContent>
        <h4>This has the following gates :</h4>
        <br />
        <h4>
          Not gate : inverts the input And gate : output is true only if both of
          the inputs are true
        </h4>
        <br />
        <h4>Or gate : output is true if any of the input is true</h4>
        <br />
        <h4>And gate : output is true only if both of the inputs are true</h4>
        <br />
        <h4>Xor gate : output is true only if exactly one input is true</h4>

        <br />
        <br />
        <br />
        <h4>
          The tick function is called automatically whenver the slider is
          changed
        </h4>
      </TabContent>
      <TabContent>
        <p>
          This contains a 4-bit Ripple Carry adder. This adder is comparatively
          slower, as calculation of previous carry output needed as next carry
          input requires time.
        </p>
        <p>
          More detailed information can be found on
          <Link
            href="https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder"
            >Wikipedia</Link
          >
        </p>
        <p>
          <img
            src="https://upload.wikimedia.org/wikipedia/commons/5/5d/4-bit_ripple_carry_adder.svg"
            alt="Ripple carry adder gate diagram"
            width="500"
          />
          (source : wikimedia commons)
        </p>
        <br />
        <p>
          To run, give a 4 bit binary input using toggle for two number to be
          added, and an optiona carry using carry toggle.
        </p>
        <br />
        <p>
          Then click tick on this to see the changes in the Output section. This
          will need roughly 12 ticks for the output to stabilize, but most of
          the times the output should stabilize in ~9 ticks.
        </p>
      </TabContent>
      <TabContent>
        <p>
          This contains a 4-bit Carry Look ahead adder. This adder is faster
          than ripple adder as it does not have propagation delay for carries of
          previous bits.
        </p>
        <p>
          More detailed information can be found on
          <Link href="https://en.wikipedia.org/wiki/Carry-lookahead_adder"
            >Wikipedia</Link
          >
        </p>
        <p>
          <img
            src="https://upload.wikimedia.org/wikipedia/commons/1/16/Four_bit_adder_with_carry_lookahead.svg"
            alt="Ripple carry adder gate diagram"
            width="500"
          />
          (source : wikimedia commons)
        </p>
        <br />
        <p>
          To run, give a 4 bit binary input using toggle for two number to be
          added, and an optiona carry using carry toggle.
        </p>
        <br />
        <p>
          Then click tick on this to see the changes in the Output section. This
          will need roughly 5 ticks for the output to stabilize, which is almost
          half of the ripple carry adder.
        </p>
      </TabContent>
      <TabContent>
        <p>
          This has examples of SR, Gated SR, D , T and JK Latches. Each of the
          latch has its own individual tick button, which can be used to run the
          latches.
        </p>
        <p>
          More information on latches can be found at <Link
            href="https://en.wikipedia.org/wiki/Flip-flop_(electronics)"
            >Wikipedia</Link
          >.
        </p>
        <br />
        <p>
          <strong>SR Latch :</strong>
          Initially it will toggle between true-false when it is not initialized.
          To initialize it, turn on either set or reset, after which it will stabalize
          into one of the valid states. Then you can either set it or reset it by
          setting the respective option. Any changes take 2 ticks to propagate from
          input to output.
        </p>
        <br />
        <p>
          <strong>Gated SR Latch :</strong>
          Similar to SR Latch, but adds another input of enable. If enable is turned
          off, then the changes in S and R does not affect the state. When enable
          is turned on, it behaves same as SR Latch.
        </p>
        <br />
        <p>
          <strong>D Latch :</strong>
          This latch is used to store data bits. When enable is active, the value
          of D (true/false) is stored in the latch. When enable is false, changes
          in D does not affect the state. It takes 2 ticks for input to propagate
          to output.
        </p>
        <br />
        <p>
          <strong>T Latch :</strong>
          This Latch toggles its state when enable and T is active. If T is inactive
          then the state does not change. If enable is not active, then changes in
          T does not affect the state. Initially it is set in the state of Q = False,
          NotQ = True. Note that this takes 2 ticks for changes to propagate, and
          if value of T is changed before that, it will go into an invalid state,
          and stay in it.
        </p>

        <br />
        <p>
          <strong>JK Latch :</strong>
          This latch combines Gated SR latch and T latch, and overcomes the instability
          issues of SR when both inputs are active. When enable is inavtive, the
          state is not affected by changes in input. With enable active, if both
          J and K are in active the state does not change. Whne J and K both are
          active , it acts like T latch and toggles the input. Otherwise, J acts
          like S and K acts like R inputs of SR Latch. This takes 3 ticks to propagate
          the input to the output.
        </p>
      </TabContent>
      <TabContent>
        <p>
          This contains two of the 4-bit Ring counters : One-Hot Ring counter
          and Johnson's Ring counter.
        </p>
        <br />
        <p>
          <strong>One-Hot Ring Counter :</strong> This counts in the pattern of `1000`->`0100`->...->`1000`.
          To initialize this, turn off enable, and turn on reset, and tick once.
          This will initialize the counter to `1000`. Same can be used to reset the
          counter to `1000` state in between its counting. Then activate the enable,
          turn reset off, and it will start shifting the `1` with ticks. It take
          3 ticks to shift the 1 by one position.
        </p>

        <br />
        <p>
          <strong>Johnson's Ring Counter :</strong> This counts in the pattern of
          `0000`->`1000`->...->`1111`->`0111`->...`0000`. To initialize this, turn
          off enable, and turn on reset, and tick once. This will initialize the
          counter to `0000`. Same can be used to reset the counter to `0000` state
          in between its counting. Then activate the enable, turn reset off, and
          it will start shifting states with ticks. It take 3 ticks to change into
          next state.
        </p>
      </TabContent>
      <TabContent>
        <p>
          This is a CPU having 2 registers, 3 flags, 18 instructions interfaced
          with a 256 byte RAM.
        </p>
        <br />
        <p>
          The default program adds numbers 0-5 and takes ~255 ticks, which have
          to be done manualy. But this shows all of the CPU functions such as
          cache fetching when empty, invalidataing cache for jumps etc, waiting
          on RAM for datainstruction fetch etc.
        </p>
        <br />
        <p>The opcodes for the CPU assembly are :</p>
        <br />
        <ul>
          <li>hlt : Halts the execution (1 byte)</li>
          <li>
            mov a/b , addr : move value from address addr to reg1/2 (1 byte)
          </li>
          <li>
            mov addr, a/b : move value from reg1/2 to address addr (1 byte)
          </li>
          <li>mvi a/b, val : move value `val` into reg1/2 (2 byte)</li>
          <li>mvi addr, val : move value `val` into address addr (3 byte)</li>
          <li>xchg: exchagnes value of reg1 and reg2 (1 byte)</li>
          <li>
            add: adds values of reg1 and reg2 and store result in reg1 (1 byte)
          </li>
          <li>
            adi val: add value `val` to contents of reg1 and store it in reg1 (2
            byte)
          </li>
          <li>
            sub: subtracts values of reg2 from reg1 and store result in reg1 (1
            byte)
          </li>
          <li>
            sbi val: subtracts value `val` from contents of reg1 and store it in
            reg1 (2 byte)
          </li>
          <li>jz addr: jumps to address addr if zero flag is set (2 byte)</li>
          <li>
            jnz addr: jumps to address addr if zero flag is not set (2 byte)
          </li>
          <li>
            jl addr: jumps to address addr if less than flag is set (2 byte)
          </li>
          <li>
            jg addr: jumps to address addr if greater than flag is set (2 byte)
          </li>
          <li>nop: No operation (1 byte)</li>
        </ul>
        <br />
        <p>The flags are set as follows on all add/adi/sub/sbi operations :</p>
        <br />
        <ul>
          <li>zero flag: set if reg1 is zero after the operation</li>
          <li>less than flag: set if reg1 &lt; reg2 after the operation</li>
          <li>greater than flag: set if reg1 &gt; reg2 after the operation</li>
        </ul>
        <br />
        <p>
          The compiler provided in the CPU-RAM page will compile the
          instructions to binary, but lables are not supported, so the jump
          addresses will have to be calculated manually. The instructions are
          loaded at memory location 0.
        </p>
        <br />
        <p>
          To compile and load the program, click on load. If there is any syntac
          error, it will show invalid code error. After successfully loading,
          you can click on next to tick the CPU and RAM.
        </p>
      </TabContent>
    </svelte:fragment>
  </Tabs>
</Tile>

<style>
  li {
    font-size: 1.05rem;
    margin-bottom: 0.5rem;
  }
</style>
