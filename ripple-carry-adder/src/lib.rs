use full_adder::get_full_adder;
use pcb_rs::pcb;

pcb!(RippleAdder{
    chip fa1;
    chip fa2;
    chip fa3;
    chip fa4;

    fa1::carry - fa2::cin;
    fa2::carry - fa3::cin;
    fa3::carry - fa4::cin;

    expose fa1::bit1 as a1;
    expose fa2::bit1 as a2;
    expose fa3::bit1 as a3;
    expose fa4::bit1 as a4;

    expose fa1::bit2 as b1;
    expose fa2::bit2 as b2;
    expose fa3::bit2 as b3;
    expose fa4::bit2 as b4;

    expose fa1::cin as cin;

    expose fa1::sum as s1;
    expose fa2::sum as s2;
    expose fa3::sum as s3;
    expose fa4::sum as s4;

    expose fa4::carry as carry;
});

pub fn get_ripple_adder() -> RippleAdder {
    let fa1 = Box::new(get_full_adder());
    let fa2 = Box::new(get_full_adder());
    let fa3 = Box::new(get_full_adder());
    let fa4 = Box::new(get_full_adder());

    let t = RippleAdderBuilder::new();

    t.add_chip("fa1", fa1)
        .add_chip("fa2", fa2)
        .add_chip("fa3", fa3)
        .add_chip("fa4", fa4)
        .build()
        .unwrap()
}
