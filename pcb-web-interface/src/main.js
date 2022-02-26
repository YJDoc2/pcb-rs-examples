import App from './App.svelte';
import wasm from '../../wasm-bindings/Cargo.toml';
import 'carbon-components-svelte/css/g10.css';

const init = async () => {
  const bindings = await wasm();

  const app = new App({
    target: document.body,
    props: {
      bindings,
    },
  });
};

init();
