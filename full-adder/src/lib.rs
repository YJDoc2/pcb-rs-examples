use basic_gates::{AndGate, OrGate, XorGate};
use pcb_rs::pcb;

pcb!(FullAdder{
    chip xor1;
    chip xor2;

    chip and1;
    chip and2;
    chip or1;

    xor1::out - xor2::in1;
    xor1::out - and1::in1;
    and1::out - or1::in1;
    and2::out - or1::in2;


    expose xor1::in1,and2::in1 as bit1;
    expose xor1::in2,and2::in2 as bit2;

    expose xor2::in2,and1::in2 as cin;

    expose xor2::out as sum;
    expose or1::out as carry;
    expose xor1::out as p;


});

pub fn get_full_adder() -> FullAdder {
    let xor1 = Box::new(XorGate::default());
    let xor2 = Box::new(XorGate::default());

    let and1 = Box::new(AndGate::default());
    let and2 = Box::new(AndGate::default());
    let or1 = Box::new(OrGate::default());

    let t = FullAdderBuilder::new();
    let adder = t
        .add_chip("xor1", xor1)
        .add_chip("xor2", xor2)
        .add_chip("and1", and1)
        .add_chip("and2", and2)
        .add_chip("or1", or1)
        .build()
        .unwrap();
    adder
}
