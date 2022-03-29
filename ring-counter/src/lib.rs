use latches::SRDLatch;
use pcb_rs::pcb;

pcb!(OneHotCounter{
    chip d1;
    chip d2;
    chip d3;
    chip d4;

    d1::q -d2::d;
    d2::q - d3::d;
    d3::q - d4::d;
    d4::q - d1::d;

    expose d1::e,d2::e,d3::e,d4::e as e;
    expose d1::p,d2::r,d3::r,d4::r as r;
});

impl Default for OneHotCounter {
    fn default() -> Self {
        let d1 = Box::new(SRDLatch::default());
        let d2 = Box::new(SRDLatch::default());
        let d3 = Box::new(SRDLatch::default());
        let d4 = Box::new(SRDLatch::default());

        let t = OneHotCounterBuilder::new();
        t.add_chip("d1", d1)
            .add_chip("d2", d2)
            .add_chip("d3", d3)
            .add_chip("d4", d4)
            .build()
            .unwrap()
    }
}

pcb!(JohnsonRingCounter {
    chip d1;
    chip d2;
    chip d3;
    chip d4;

    d1::q -d2::d;
    d2::q - d3::d;
    d3::q - d4::d;
    d4::notq - d1::d;

    expose d1::e,d2::e,d3::e,d4::e as e;
    expose d1::r,d2::r,d3::r,d4::r as r;

});

impl Default for JohnsonRingCounter {
    fn default() -> Self {
        let d1 = Box::new(SRDLatch::default());
        let d2 = Box::new(SRDLatch::default());
        let d3 = Box::new(SRDLatch::default());
        let d4 = Box::new(SRDLatch::default());

        let t = JohnsonRingCounterBuilder::new();
        t.add_chip("d1", d1)
            .add_chip("d2", d2)
            .add_chip("d3", d3)
            .add_chip("d4", d4)
            .build()
            .unwrap()
    }
}
