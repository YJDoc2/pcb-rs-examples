use latches::{GatedDLatch, GatedJKLatch, GatedSRLatch, GatedTLatch, SRLatch};
use pcb_rs::{Chip, ChipInterface};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LatchHandle {
    srlatch: SRLatch,
    gated_sr: GatedSRLatch,
    dlatch: GatedDLatch,
    tlatch: GatedTLatch,
    jklatch: GatedJKLatch,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct LatchOut {
    pub q: bool,
    pub notq: bool,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct SRBinding {
    pub s: bool,
    pub r: bool,
}

#[wasm_bindgen]
impl SRBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct GatedSRBinding {
    pub s: bool,
    pub r: bool,
    pub e: bool,
}

#[wasm_bindgen]
impl GatedSRBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct DBinding {
    pub d: bool,
    pub e: bool,
}

#[wasm_bindgen]
impl DBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct TBinding {
    pub t: bool,
    pub e: bool,
}

#[wasm_bindgen]
impl TBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct JKBinding {
    pub j: bool,
    pub k: bool,
    pub e: bool,
}

#[wasm_bindgen]
impl JKBinding {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
pub fn get_latch_handle() -> LatchHandle {
    LatchHandle {
        srlatch: SRLatch::default(),
        gated_sr: GatedSRLatch::default(),
        dlatch: GatedDLatch::default(),
        tlatch: GatedTLatch::default(),
        jklatch: GatedJKLatch::default(),
    }
}

#[wasm_bindgen]
impl LatchHandle {
    pub fn tick_sr(&mut self, data: SRBinding) -> LatchOut {
        self.srlatch.set_pin_value("s", &data.s);
        self.srlatch.set_pin_value("r", &data.r);
        self.srlatch.tick();
        LatchOut {
            q: *self
                .srlatch
                .get_pin_value("q")
                .unwrap()
                .downcast_ref()
                .unwrap(),
            notq: *self
                .srlatch
                .get_pin_value("notq")
                .unwrap()
                .downcast_ref()
                .unwrap(),
        }
    }

    pub fn tick_gated_sr(&mut self, data: GatedSRBinding) -> LatchOut {
        self.gated_sr.set_pin_value("s", &data.s);
        self.gated_sr.set_pin_value("r", &data.r);
        self.gated_sr.set_pin_value("e", &data.e);
        self.gated_sr.tick();
        LatchOut {
            q: *self
                .gated_sr
                .get_pin_value("q")
                .unwrap()
                .downcast_ref()
                .unwrap(),
            notq: *self
                .gated_sr
                .get_pin_value("notq")
                .unwrap()
                .downcast_ref()
                .unwrap(),
        }
    }

    pub fn tick_dlatch(&mut self, data: DBinding) -> LatchOut {
        self.dlatch.set_pin_value("d", &data.d);
        self.dlatch.set_pin_value("e", &data.e);
        self.dlatch.tick();
        LatchOut {
            q: *self
                .dlatch
                .get_pin_value("q")
                .unwrap()
                .downcast_ref()
                .unwrap(),
            notq: *self
                .dlatch
                .get_pin_value("notq")
                .unwrap()
                .downcast_ref()
                .unwrap(),
        }
    }

    pub fn tick_tlatch(&mut self, data: TBinding) -> LatchOut {
        self.tlatch.set_pin_value("t", &data.t);
        self.tlatch.set_pin_value("e", &data.e);
        self.tlatch.tick();
        LatchOut {
            q: *self
                .tlatch
                .get_pin_value("q")
                .unwrap()
                .downcast_ref()
                .unwrap(),
            notq: *self
                .tlatch
                .get_pin_value("notq")
                .unwrap()
                .downcast_ref()
                .unwrap(),
        }
    }

    pub fn tick_jk(&mut self, data: JKBinding) -> LatchOut {
        self.jklatch.set_pin_value("j", &data.j);
        self.jklatch.set_pin_value("k", &data.k);
        self.jklatch.set_pin_value("e", &data.e);
        self.jklatch.tick();
        LatchOut {
            q: *self
                .jklatch
                .get_pin_value("q")
                .unwrap()
                .downcast_ref()
                .unwrap(),
            notq: *self
                .jklatch
                .get_pin_value("notq")
                .unwrap()
                .downcast_ref()
                .unwrap(),
        }
    }
}
