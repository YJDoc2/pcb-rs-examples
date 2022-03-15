use basic_gates::{AndGate, NorGate, NotGate, OrGate};
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
    chip dlatch;
    chip and1;
    chip and2;
    chip or;
    chip not;

    and1::in1 - dlatch::q;
    and2::in2 - dlatch::notq;
    not::out - and1::in2;
    or::in1 - and1::out;
    or::in2 - and2::out;
    or::out - dlatch::d;

    expose and2::in1,not::in1 as t;
    expose dlatch::e as e;
    expose dlatch::q as q;
    expose dlatch::notq as notq;
});

impl Default for GatedTLatch {
    fn default() -> Self {
        let dlatch = Box::new(GatedDLatch::default());
        let not = Box::new(NotGate::default());
        let and1 = Box::new(AndGate::default());
        let and2 = Box::new(AndGate::default());
        let or = Box::new(OrGate::default());

        let t = GatedTLatchBuilder::new();
        t.add_chip("dlatch", dlatch)
            .add_chip("not", not)
            .add_chip("and1", and1)
            .add_chip("and2", and2)
            .add_chip("or", or)
            .build()
            .unwrap()
    }
}

pcb!(GatedJKLatch{
    chip dlatch;
    chip and1;
    chip and2;
    chip or;
    chip not;

    and1::out - or::in1;
    and2::out - or::in2;
    or::out - dlatch::d;
    and1::in1 - dlatch::notq;
    and2::in2 - dlatch::q;
    not::out - and2::in1;

    expose and1::in2 as j;
    expose not::in1 as k;
    expose dlatch::e as e;
    expose dlatch::q as q;
    expose dlatch::notq as notq;
});

impl Default for GatedJKLatch {
    fn default() -> Self {
        let dlatch = Box::new(GatedDLatch::default());
        let not = Box::new(NotGate::default());
        let and1 = Box::new(AndGate::default());
        let and2 = Box::new(AndGate::default());
        let or = Box::new(OrGate::default());

        let t = GatedJKLatchBuilder::new();
        t.add_chip("dlatch", dlatch)
            .add_chip("not", not)
            .add_chip("and1", and1)
            .add_chip("and2", and2)
            .add_chip("or", or)
            .build()
            .unwrap()
    }
}
