# Ripple Carry Adder

This contains a 4-bit Ripple Carry adder. This adder is comparatively slower, as calculation of previous carry output needed as next carry input requires time. More detailed information can be found on [Wikipedia](<https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder>).

In particular, this is based on (source : wikimedia commons)
<img src="https://upload.wikimedia.org/wikipedia/commons/5/5d/4-bit_ripple_carry_adder.svg" alt="Ripple carry adder gate diagram" width="500"/>

This consists of 4 full adders, with carry out of each connected to next. Thus to get the final output, the full adders must wait for the previous gates to calculate their outputs. As the Full adders need 3 ticks, this takes total 12 ticks for a stabilized output. Although this is the worst case, where the carry output of previous keeps changing due to calculations. On an average, the output would usually stabilize in ~9 ticks. On the upside, the circuit for this is considerably simple.

This Ripple adder uses full adders `pcb!` macro to generate Ripple adder struct.

The Ripple adder needs :

- 4 Full adders

This exposes

- a0-a3 pins for input-1 4 bits
- b0-b3 pins for input-2 4 bits
- cin pin for input carry
- s0-s3 pins for output 4 bits
- carry pin as output carry

This needs 12 ticks from input to stable output.
