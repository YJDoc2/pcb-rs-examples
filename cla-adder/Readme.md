# Carry Look Ahead Adder

This contains a 4-bit Carry Look ahead adder. This adder is faster than ripple adder as it does not have propagation delay for carries of previous bits. More detailed information can be found on [Wikipedia](https://en.wikipedia.org/wiki/Carry-lookahead_adder).

In particular, this is based on (source : wikimedia commons)
<img src="https://upload.wikimedia.org/wikipedia/commons/1/16/Four_bit_adder_with_carry_lookahead.svg" alt="Carry look ahead adder gate diagram" width="500"/>

As you can see, this circuit has 4 distinct stages from input to output, and thus this circuit needs roughly 5 ticks for its output to stabilize compared to ~9-12 ticks of Ripple carry adder. This difference can be seen in the web interface.

This CLA adder uses full adders and basic gates and `pcb!` macro to generate CLA struct.

The CLA needs :

- 4 Full adders
- 4 And gates , 1 Or gate
- 3 And3 gates, 2 And4 gates, 1 And5 gate
- 1 Or3, 1 Or4, 1 Or5 gate

This exposes

- a0-a3 pins for input-1 4 bits
- b0-b3 pins for input-2 4 bits
- cin pin for input carry
- s0-s3 pins for output 4 bits
- carry pin as output carry

This needs 5 ticks from input to stable output.
