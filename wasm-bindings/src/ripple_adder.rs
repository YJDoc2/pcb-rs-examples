use pcb_rs::{Chip, ChipInterface};
use ripple_carry_adder::{get_ripple_adder, RippleAdder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RippleAdderHandle {
    ripple_adder: RippleAdder,
}

#[wasm_bindgen]
impl RippleAdderHandle {
    pub fn tick(&mut self, bindings: RippleAdderBinding) -> RippleAdderBinding {
        let adder = &mut self.ripple_adder;
        adder.set_pin_value("a0", &bindings.a0);
        adder.set_pin_value("a1", &bindings.a1);
        adder.set_pin_value("a2", &bindings.a2);
        adder.set_pin_value("a3", &bindings.a3);

        adder.set_pin_value("b0", &bindings.b0);
        adder.set_pin_value("b1", &bindings.b1);
        adder.set_pin_value("b2", &bindings.b2);
        adder.set_pin_value("b3", &bindings.b3);

        adder.set_pin_value("cin", &bindings.cin);

        adder.tick();

        let mut out = bindings;
        out.s0 = *adder.get_pin_value("s0").unwrap().downcast_ref().unwrap();
        out.s1 = *adder.get_pin_value("s1").unwrap().downcast_ref().unwrap();
        out.s2 = *adder.get_pin_value("s2").unwrap().downcast_ref().unwrap();
        out.s3 = *adder.get_pin_value("s3").unwrap().downcast_ref().unwrap();
        out.carry = *adder
            .get_pin_value("carry")
            .unwrap()
            .downcast_ref()
            .unwrap();

        out
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[wasm_bindgen]
pub struct RippleAdderBinding {
    pub a0: bool,
    pub a1: bool,
    pub a2: bool,
    pub a3: bool,

    pub b0: bool,
    pub b1: bool,
    pub b2: bool,
    pub b3: bool,

    pub cin: bool,

    pub s0: bool,
    pub s1: bool,
    pub s2: bool,
    pub s3: bool,

    pub carry: bool,
}
#[wasm_bindgen]
impl RippleAdderBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
pub fn get_ripple_adder_handle() -> RippleAdderHandle {
    RippleAdderHandle {
        ripple_adder: get_ripple_adder(),
    }
}
