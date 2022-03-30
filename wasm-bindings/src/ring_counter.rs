use pcb_rs::{Chip, ChipInterface};
use ring_counter::{JohnsonRingCounter, OneHotCounter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CounterHandle {
    one_hot: OneHotCounter,
    johnson_ring: JohnsonRingCounter,
}

#[wasm_bindgen]
pub struct CounterValueBinding {
    pub q1: bool,
    pub q2: bool,
    pub q3: bool,
    pub q4: bool,
}

#[derive(Default)]
#[wasm_bindgen]
pub struct CounterBinding {
    pub e: bool,
    pub r: bool,
}

#[wasm_bindgen]
impl CounterBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
impl CounterHandle {
    pub fn tick_one_hot(&mut self, b: CounterBinding) -> CounterValueBinding {
        let one_hot = &mut self.one_hot;
        one_hot.set_pin_value("e", &b.e);
        one_hot.set_pin_value("r", &b.r);
        one_hot.tick();
        let q1 = *one_hot.get_pin_value("q1").unwrap().downcast_ref().unwrap();
        let q2 = *one_hot.get_pin_value("q2").unwrap().downcast_ref().unwrap();
        let q3 = *one_hot.get_pin_value("q3").unwrap().downcast_ref().unwrap();
        let q4 = *one_hot.get_pin_value("q4").unwrap().downcast_ref().unwrap();
        CounterValueBinding { q1, q2, q3, q4 }
    }
    pub fn tick_johnson_ring(&mut self, b: CounterBinding) -> CounterValueBinding {
        let johnson_ring = &mut self.johnson_ring;
        johnson_ring.set_pin_value("e", &b.e);
        johnson_ring.set_pin_value("r", &b.r);
        johnson_ring.tick();
        let q1 = *johnson_ring
            .get_pin_value("q1")
            .unwrap()
            .downcast_ref()
            .unwrap();
        let q2 = *johnson_ring
            .get_pin_value("q2")
            .unwrap()
            .downcast_ref()
            .unwrap();
        let q3 = *johnson_ring
            .get_pin_value("q3")
            .unwrap()
            .downcast_ref()
            .unwrap();
        let q4 = *johnson_ring
            .get_pin_value("q4")
            .unwrap()
            .downcast_ref()
            .unwrap();
        CounterValueBinding { q1, q2, q3, q4 }
    }
}

#[wasm_bindgen]
pub fn get_counter_handle() -> CounterHandle {
    CounterHandle {
        one_hot: OneHotCounter::default(),
        johnson_ring: JohnsonRingCounter::default(),
    }
}
