use basic_gates::{And3, And4, And5, AndGate, Or3, Or4, Or5, OrGate};
use full_adder::get_full_adder;
use pcb_rs::pcb;

pcb!(CLAAdder{
    chip fa1;
    chip fa2;
    chip fa3;
    chip fa4;

    chip or1;
    chip and1;

    chip or3_1;
    chip and3_1;
    chip and2;

    chip or4_1;
    chip and4_1;
    chip and3_2;
    chip and3;

    chip or5_1;
    chip and5_1;
    chip and4_2;
    chip and3_3;
    chip and4;

    and1::in1 - fa1::p;
    or1::in1 - and1::out;
    or1::in2 - fa1::carry;

    fa2::cin - or1::out;

    and2::in1 - fa2::p;
    and2::in2 - fa1::carry;

    and3_1::in1 - fa2::p;
    and3_1::in2 - fa1::p;

    or3_1::in1 - fa2::carry;
    or3_1::in2 - and2::out;
    or3_1::in3 - and3_1::out;

    fa3::cin - or3_1::out;

    and3::in1 - fa3::p;
    and3::in2 - fa2::carry;

    and3_2::in1 - fa3::p;
    and3_2::in2 - fa2::p;
    and3_2::in3 - fa1::carry;

    and4_1::in1 - fa3::p;
    and4_1::in2 - fa2::p;
    and4_1::in3 - fa1::p;

    or4_1::in1 - fa3::carry;
    or4_1::in2 - and3::out;
    or4_1::in3 - and3_2::out;
    or4_1::in4 - and4_1::out;

    fa4::cin - or4_1::out;

    and4::in1 - fa4::p;
    and4::in2 - fa3::carry;

    and3_3::in1 - fa4::p;
    and3_3::in2 - fa3::p;
    and3_3::in3 - fa2::carry;

    and4_2::in1 - fa4::p;
    and4_2::in2 - fa3::p;
    and4_2::in3 - fa2::p;
    and4_2::in4 - fa1::carry;

    and5_1::in1 - fa4::p;
    and5_1::in2 - fa3::p;
    and5_1::in3 - fa2::p;
    and5_1::in4 - fa1::p;

    or5_1::in1 - fa4::carry;
    or5_1::in2 - and4::out;
    or5_1::in3 - and3_3::out;
    or5_1::in4 - and4_2::out;
    or5_1::in5 - and5_1::out;

    expose fa1::bit1 as a0;
    expose fa2::bit1 as a1;
    expose fa3::bit1 as a2;
    expose fa4::bit1 as a3;

    expose fa1::bit2 as b0;
    expose fa2::bit2 as b1;
    expose fa3::bit2 as b2;
    expose fa4::bit2 as b3;

    expose fa1::cin,and1::in2,and3_1::in3,and4_1::in4,and5_1::in5 as cin;

    expose fa1::sum as s0;
    expose fa2::sum as s1;
    expose fa3::sum as s2;
    expose fa4::sum as s3;

    expose or5_1::out as carry;

});

pub fn get_cla_adder() -> CLAAdder {
    let fa1 = Box::new(get_full_adder());
    let fa2 = Box::new(get_full_adder());
    let fa3 = Box::new(get_full_adder());
    let fa4 = Box::new(get_full_adder());

    let or1 = Box::new(OrGate::default());
    let and1 = Box::new(AndGate::default());

    let or3_1 = Box::new(Or3::default());
    let and3_1 = Box::new(And3::default());
    let and2 = Box::new(AndGate::default());

    let or4_1 = Box::new(Or4::default());
    let and4_1 = Box::new(And4::default());
    let and3_2 = Box::new(And3::default());
    let and3 = Box::new(AndGate::default());

    let or5_1 = Box::new(Or5::default());
    let and5_1 = Box::new(And5::default());
    let and4_2 = Box::new(And4::default());
    let and3_3 = Box::new(And3::default());
    let and4 = Box::new(AndGate::default());

    let t = CLAAdderBuilder::new();

    t.add_chip("fa1", fa1)
        .add_chip("fa2", fa2)
        .add_chip("fa3", fa3)
        .add_chip("fa4", fa4)
        .add_chip("or1", or1)
        .add_chip("and1", and1)
        .add_chip("or3_1", or3_1)
        .add_chip("and3_1", and3_1)
        .add_chip("and2", and2)
        .add_chip("or4_1", or4_1)
        .add_chip("and4_1", and4_1)
        .add_chip("and3_2", and3_2)
        .add_chip("and3", and3)
        .add_chip("or5_1", or5_1)
        .add_chip("and5_1", and5_1)
        .add_chip("and4_2", and4_2)
        .add_chip("and3_3", and3_3)
        .add_chip("and4", and4)
        .build()
        .unwrap()
}
