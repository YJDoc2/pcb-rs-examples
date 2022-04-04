# Web interface for PCB-Rs examples

This directory contains source for the web interface of pcb-rs examples, and is written in svelte + WASM.

The `src/` has the main svelte components and stuff. This uses `@wasm-tool/rollup-plugin-rust` for setting up the wasm.
Also see [wasm-bindings](../wasm-bindings/Readme.md)

The site should be up at [https://yjdoc2.github.io/pcb-rs-examples/](https://yjdoc2.github.io/pcb-rs-examples/).
