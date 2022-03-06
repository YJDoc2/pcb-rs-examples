use basic_gates::{AndGate, NotGate, OrGate, XorGate};
use pcb_rs::{pcb, Chip, ChipInterface};
use wasm_bindgen::prelude::*;

pcb!(GatesPCB{
    chip not;
    chip and;
    chip or;
    chip xor;

    expose not::in1 as not_in;
    expose not::out as not_out;

    expose and::in1 as and_in1;
    expose and::in2 as and_in2;
    expose and::out as and_out;

    expose or::in1 as or_in1;
    expose or::in2 as or_in2;
    expose or::out as or_out;

    expose xor::in1 as xor_in1;
    expose xor::in2 as xor_in2;
    expose xor::out as xor_out;

});

#[wasm_bindgen]
pub struct GatesHandle {
    gates: GatesPCB,
}

#[wasm_bindgen]
impl GatesHandle {
    pub fn tick(&mut self, binding: GatesBinding) -> GatesBinding {
        let gates = &mut self.gates;
        gates.set_pin_value("not_in", &binding.not_in);

        gates.set_pin_value("and_in1", &binding.and_in1);
        gates.set_pin_value("and_in2", &binding.and_in2);

        gates.set_pin_value("or_in1", &binding.or_in1);
        gates.set_pin_value("or_in2", &binding.or_in2);

        gates.set_pin_value("xor_in1", &binding.xor_in1);
        gates.set_pin_value("xor_in2", &binding.xor_in2);

        gates.tick();

        let mut out = binding;
        out.not_out = *gates
            .get_pin_value("not_out")
            .unwrap()
            .downcast_ref()
            .unwrap();

        out.and_out = *gates
            .get_pin_value("and_out")
            .unwrap()
            .downcast_ref()
            .unwrap();

        out.or_out = *gates
            .get_pin_value("or_out")
            .unwrap()
            .downcast_ref()
            .unwrap();

        out.xor_out = *gates
            .get_pin_value("xor_out")
            .unwrap()
            .downcast_ref()
            .unwrap();
        out
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[wasm_bindgen]
pub struct GatesBinding {
    pub not_in: bool,
    pub not_out: bool,
    pub and_in1: bool,
    pub and_in2: bool,
    pub and_out: bool,
    pub or_in1: bool,
    pub or_in2: bool,
    pub or_out: bool,
    pub xor_in1: bool,
    pub xor_in2: bool,
    pub xor_out: bool,
}

#[wasm_bindgen]
impl GatesBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
pub fn get_gates_handle() -> GatesHandle {
    let t = GatesPCBBuilder::new();
    let not = Box::new(NotGate::default());
    let and = Box::new(AndGate::default());
    let or = Box::new(OrGate::default());
    let xor = Box::new(XorGate::default());
    let gates = t
        .add_chip("not", not)
        .add_chip("and", and)
        .add_chip("or", or)
        .add_chip("xor", xor)
        .build()
        .unwrap();
    GatesHandle { gates }
}
