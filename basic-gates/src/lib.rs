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

#[derive(Chip, Default)]
pub struct And3 {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(input)]
    pub in3: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for And3 {
    fn tick(&mut self) {
        self.out = self.in1 && self.in2 && self.in3;
    }
}

#[derive(Chip, Default)]
pub struct And4 {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(input)]
    pub in3: bool,
    #[pin(input)]
    pub in4: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for And4 {
    fn tick(&mut self) {
        self.out = self.in1 && self.in2 && self.in3 && self.in4;
    }
}

#[derive(Chip, Default)]
pub struct And5 {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(input)]
    pub in3: bool,
    #[pin(input)]
    pub in4: bool,
    #[pin(input)]
    pub in5: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for And5 {
    fn tick(&mut self) {
        self.out = self.in1 && self.in2 && self.in3 && self.in4 && self.in5;
    }
}

#[derive(Chip, Default)]
pub struct Or3 {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(input)]
    pub in3: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for Or3 {
    fn tick(&mut self) {
        self.out = self.in1 || self.in2 || self.in3;
    }
}

#[derive(Chip, Default)]
pub struct Or4 {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(input)]
    pub in3: bool,
    #[pin(input)]
    pub in4: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for Or4 {
    fn tick(&mut self) {
        self.out = self.in1 || self.in2 || self.in3 || self.in4;
    }
}

#[derive(Chip, Default)]
pub struct Or5 {
    #[pin(input)]
    pub in1: bool,
    #[pin(input)]
    pub in2: bool,
    #[pin(input)]
    pub in3: bool,
    #[pin(input)]
    pub in4: bool,
    #[pin(input)]
    pub in5: bool,
    #[pin(output)]
    pub out: bool,
}

impl pcb_rs::Chip for Or5 {
    fn tick(&mut self) {
        self.out = self.in1 || self.in2 || self.in3 || self.in4 || self.in5;
    }
}
