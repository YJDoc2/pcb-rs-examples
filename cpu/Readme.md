# CPU

This contains an 8-bit, 2 registers, 3 Flags, 18 instruction CPU. This has an 8-byte instruction cache, which auto-fills when empty or invalidates and refills on jumps. This needs to interface with 8-bit wide address bus RAM, which takes 2 ticks to read data and 1 tick to store data, such as [RAM](../ram/Readme.md).

This exposes :

- 8 bit addr (address bus) in form of `u8`
- 8 bit data (data bus) in form of `Option<u8>`
- 1 bit mem_active (memory select) in form of bool
- 1 bit read_mem (read/write) in form of bool

The CPU starts executing code from mem location 0.

This is basically a state machine, which transfers from one state to another while running. The readme does not go into too details, but if you want, you can check the source to see the implementation. This has following states :

- Hlt : CPU is Halted, no change will occur.
- InstrFetch : CPU is fetching instructions to fill its instruction cache. THis has following sub-states:
  - Priming : Used when CPU is initially starting, and does initial fetch from mem location 0
  - Blocked : CPU is waiting on RAM to read the instruction
  - Open : CPU has received instruction from RAM, and can either enqueue next fetch or start execution
- Executing : CPU is currently in-between executing an instruction. This state is only used in case of data write, as only then CPU waits on RAM, otherwise all instructions execute in 1 tick
- Idle : CPU is not in-between execution of an instruction, so in this it executed next instruction. The name is a bit misleading, now that I'm reading it :sweat_smile:

Instruction set of this CPU is :

- 0 : stop
- 1 : read next byte addr and store in reg1. Takes form `1 addr`, eg. `1 5` to read data from location 5.
- 2 : read next byte addr and store in reg2. Takes form `1 addr`, eg. `1 5` to read data from location 5.
- 3 : write reg1 to next addr. Takes form `1 addr`, eg. `1 5` to write data in location 5.
- 4 : write reg2 to next addr. Takes form `1 addr`, eg. `1 5` to write data in location 5.
- 5 : store next byte in reg1. Takes form `1 data`, eg `1 5` to store `5` in reg1.
- 6 : store next byte in reg2. Takes form `1 data`, eg `1 5` to store `5` in reg2.
- 7 : read next byte as addr, and store next-to-next byte at addr. takes form `1 addr data`. eg `1 5 7` will store `7` at mem loc `5`
- 8 : xchg. Exchanges contents of reg1 and reg2.
- 9 : add reg1 and reg2 and store in reg1.
- 10 : add next byte to reg1. Takes form `10 data`, eg `10 5` to add `5` to reg1.
- 11 : sub reg2 from reg1 and store in reg1
- 12 : sub next byte from reg1. Takes form `12 data`, eg `12 5` to subtract `5` from reg1.
- 13 : jump to next byte if zero flag is set. Takes form `13 addr`, eg `13 5` will jump to location 5 if zero flag is set.
- 14 : jump to next byte if not zero. Takes form `14 addr`, eg `14 5` will jump to location 5 if zero flag is not set.
- 15 : jump to next byte if reg1 less than reg2. Takes form `15 addr`, eg `15 5` will jump to location 5 if reg1 < reg2.
- 16 : jump to next byte if reg1 greater than reg2. Takes form `16 addr`, eg `16 5` will jump to location 5 if reg1 > reg2.
- 255 : nop. Does nothing, takes 1 byte.

See [PCB](../pcb/Readme.md) to see more details for and example of how to interface this.
