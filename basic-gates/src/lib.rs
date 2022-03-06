use pcb_rs::Chip;

#[derive(Chip, Default)]
pub struct NotGate {
    #[pin(input)]
    pub in1: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for NotGate {
    fn tick(&mut self) {
        self.out = !self.in1;
    }
}

#[derive(Chip, Default)]
pub struct AndGate {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for AndGate {
    fn tick(&mut self) {
        self.out = self.in1 && self.in2;
    }
}

#[derive(Chip, Default)]
pub struct OrGate {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for OrGate {
    fn tick(&mut self) {
        self.out = self.in1 || self.in2;
    }
}

#[derive(Chip, Default)]
pub struct XorGate {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for XorGate {
    fn tick(&mut self) {
        self.out = self.in1 ^ self.in2;
    }
}
