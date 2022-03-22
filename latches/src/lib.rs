use basic_gates::{And3, AndGate, NorGate, NotGate};
use pcb_rs::pcb;

pcb!(SRLatch{
    chip nor1;
    chip nor2;

    nor1::in2 - nor2::out;
    nor2::in2 - nor1::out;

    expose nor1::in1 as r;
    expose nor2::in1 as s;
    expose nor1::out as q;
    expose nor2::out as notq;
});

impl Default for SRLatch {
    fn default() -> Self {
        let nor1 = Box::new(NorGate::default());
        let nor2 = Box::new(NorGate::default());

        let t = SRLatchBuilder::new();
        t.add_chip("nor1", nor1)
            .add_chip("nor2", nor2)
            .build()
            .unwrap()
    }
}

pcb!(GatedSRLatch{
    chip nor1;
    chip nor2;
    chip and1;
    chip and2;

    and1::out - nor1::in1;
    and2::out - nor2::in2;
    nor1::in2 - nor2::out;
    nor2::in1 - nor1::out;

    expose and1::in1 as r;
    expose and2::in2 as s;
    expose and1::in2,and2::in1 as e;
    expose nor1::out as q;
    expose nor2::out as notq;
});

impl Default for GatedSRLatch {
    fn default() -> Self {
        let nor1 = Box::new(NorGate::default());
        let nor2 = Box::new(NorGate::default());
        let and1 = Box::new(AndGate::default());
        let and2 = Box::new(AndGate::default());

        let t = GatedSRLatchBuilder::new();
        t.add_chip("nor1", nor1)
            .add_chip("nor2", nor2)
            .add_chip("and1", and1)
            .add_chip("and2", and2)
            .build()
            .unwrap()
    }
}

pcb!(GatedDLatch{
    chip gated_sr;
    chip not;

    not::out - gated_sr::r;

    expose not::in1,gated_sr::s as d;
    expose gated_sr::e as e;
    expose gated_sr::q as q;
    expose gated_sr::notq as notq;
});

impl Default for GatedDLatch {
    fn default() -> Self {
        let gated_sr = Box::new(GatedSRLatch::default());
        let not = Box::new(NotGate::default());

        let t = GatedDLatchBuilder::new();
        t.add_chip("not", not)
            .add_chip("gated_sr", gated_sr)
            .build()
            .unwrap()
    }
}

pcb!(GatedTLatch{
    chip and3_1;
    chip and3_2;
    chip nor1;
    chip nor2;

    and3_1::out - nor1::in1;
    and3_2::out - nor2::in2;
    nor1::in2 - nor2::out;
    nor2::in1 - nor1::out;

    nor1::out - and3_1::in1;
    nor2::out - and3_2::in3;

    expose and3_1::in2, and3_2::in2 as t;
    expose and3_1::in3,and3_2::in1 as e;
    expose nor1::out as q;
    expose nor2::out as notq;
});

impl Default for GatedTLatch {
    fn default() -> Self {
        use pcb_rs::Chip;
        let mut and3_1 = Box::new(And3::default());
        let and3_2 = Box::new(And3::default());
        let nor1 = Box::new(NorGate::default());
        let nor2 = Box::new(NorGate::default());
        // We have to prime the T latch with a state such that Q and Not Q are valid
        // i.e. Q = ! notQ, only then T latch will function properly
        // for that we set the input of 1 nor to be true, other to be false
        // and propagate those inputs by calling tick()
        // This way, the T latch is in state where Q = False, notQ = True
        // and it functions as expected
        and3_1.in1 = true;
        and3_1.in2 = true;
        and3_1.in3 = true;
        let t = GatedTLatchBuilder::new();
        let mut l = t
            .add_chip("and3_1", and3_1)
            .add_chip("and3_2", and3_2)
            .add_chip("nor1", nor1)
            .add_chip("nor2", nor2)
            .build()
            .unwrap();
        // propagate the priming values
        l.tick();
        l.tick();
        l
    }
}

pcb!(GatedJKLatch{
    chip and3_1;
    chip and3_2;
    chip nor1;
    chip nor2;

    and3_1::out - nor1::in1;
    and3_2::out - nor2::in2;
    nor1::in2 - nor2::out;
    nor2::in1 - nor1::out;

    nor1::out - and3_1::in1;
    nor2::out - and3_2::in3;

    expose and3_1::in2 as k;
    expose and3_2::in2 as j;
    expose and3_1::in3,and3_2::in1 as e;
    expose nor1::out as q;
    expose nor2::out as notq;
});

impl Default for GatedJKLatch {
    fn default() -> Self {
        let and3_1 = Box::new(And3::default());
        let and3_2 = Box::new(And3::default());
        let nor1 = Box::new(NorGate::default());
        let nor2 = Box::new(NorGate::default());

        let t = GatedJKLatchBuilder::new();
        t.add_chip("and3_1", and3_1)
            .add_chip("and3_2", and3_2)
            .add_chip("nor1", nor1)
            .add_chip("nor2", nor2)
            .build()
            .unwrap()
    }
}
