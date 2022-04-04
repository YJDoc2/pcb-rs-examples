# Full adder

This contains a Full adder, capable of adding 3 bits. More detailed information can be found on [Wikipedia](<https://en.wikipedia.org/wiki/Adder_(electronics)#Full_adder>).

In particular, this is based on (source : wikimedia commons)
<img src="https://upload.wikimedia.org/wikipedia/commons/6/69/Full-adder_logic_diagram.svg" alt="Full adder diagram" width="500"/>

This circuit has 3 distinct stages, thus need about 3 ticks for the output to stabilize. This uses Basic gates with `pcb!` macro to generate the struct.

This needs :

- 2 Xor gates
- 2 And gates
- 1 Or gate

This exposes

- bit1 as input bit 1
- bit2 as input bit 2
- cin as input carry
- sum as output sum
- carry as output carry
- p as the output of bit1 xor bit2 (needed for CLA adder)

This needs 3 ticks from input to stable output
