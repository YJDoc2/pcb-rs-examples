# PCB-RS Examples

---

This repository contains examples to demonstrate use of [pcb-rs](https://www.github.com/YJDoc2/pcb-rs). To see more information about what is pcb-rs, the reasoning behind it, and basic syntax and examples, visit its repository at [https://www.github.com/YJDoc2/pcb-rs](https://www.github.com/YJDoc2/pcb-rs).

This repository contains various examples on how to write components, as well as how to reuse components from other libraries to create a composite component. Each of the directories contain their own readme describing the that particular example.

The web version of these examples is hosted at [yjdoc2.github.io/pcb-rs-examples/](https://yjdoc2.github.io/pcb-rs-examples/), so you can try them right in your browser without needing to install anything, as well as show the possibilities with this library and WASM for making such electronics related tools and simulations.

---

Various directories in this repo contain different examples, and their details are explained in their own readmes in that directory. Currently this repo contains following directories :

- [Basic Gates](./basic-gates/) : This contains basic gates such as And, Or, Not etc.
- [Full Adder](./full-adder/) : This contains a full (3-bit) adder, made using basic gates.
- [Ripple Carry Adder](./ripple-carry-adder/) : This contains a 4-bit ripple carry adder made using full adders.
- [CLA Adder](./cla-adder/) : This contains a 4-bit carry look ahead adder, which is a faster version of ripple adder, but needs more gates and complex circuit.
- [Latches](./latches/) : This contains various latches such as SR, D, T, JK etc.
- [Ring Counter](./ring-counter/) : This contains One-Hot and Johnson's ring counter made using Latches.
- [RAM](./ram/) : This contains a 256-byte ram chip.
- [CPU](./cpu/) : This contains a 8-bit CPU having 18 instructions such as add, sub, mov, jmps etc.
- [PCB](./pcb/) : This has a pcb, which connected the RAM and CPU to form a runnable (although not much useful) microprocessor. This is a binary (not a lib) and can be run from command line using `cargo run`.

Apart from these, this also contains following two directories which provides a svelte web app to show the above using a web interface, and WASM bindings.

- [wasm-bindings](./wasm-bindings/) : This contains the WASM interface and bindings needed for using the above components from web interface.
- [pcb-web-interface](./pcb-web-interface/) : This has the svelte app, which provides a web interface for above components, so you don't have to install anything to try them. This is hosted at [yjdoc2.github.io/pcb-rs-examples/](https://yjdoc2.github.io/pcb-rs-examples/).

---

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
