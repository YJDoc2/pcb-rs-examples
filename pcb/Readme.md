# PCB

A simple binary showing how to interface [CPU](../cpu/Readme.md) and [RAM](../ram/Readme.md).

This has the interfacing set up using `pcb!` macro, and has the following program hardcoded by default, which can be changed if needed, but you will need to manually compile the instructions to `u8` for the CPU instruction set.

This will add up 0-5 numbers and store result in mem loc 52

```sh
mvi a 0
mvi b 5
add
xchg
sbi 1
jz 13
xchg
jnz 4
xchg
mov 52 a
hlt
```

To run simply run `cargo run`, but the output will be a bit uninteresting by default. It will show memory locations 50-55 before and after execution of the above program. It is better to see the program running tick-by-tick in the [web-interface](https://yjdoc2.github.io/pcb-rs-examples/).

This code takes roughly 225 ticks to complete. This number also includes the ticks needed for instruction-cache refilling, cache-invalidation after jumps and refilling etc.
