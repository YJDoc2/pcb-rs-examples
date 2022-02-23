use pcb_rs::Chip;
use std::collections::VecDeque;

// instructions are :
// 0 : stop
// 1 : read next byte addr and store in reg1
// 2 : read next byte addr and store in reg2
// 3 : write reg1 to next addr
// 4 : write reg2 to next addr
// 5 : add reg1 and reg2 and store in reg1
#[derive(Chip)]
pub struct CPU {
    #[pin(output)]
    addr_bus: u8,
    #[pin(io, io_latch)]
    data_bus: Option<u8>,
    #[pin(output)]
    mem_active: bool,
    #[pin(output)]
    read_mem: bool,

    io_latch: bool,
    reg1: u8,
    reg2: u8,
    crr_instr: Option<(u8, u8)>,
    pub instr_cache: VecDeque<u8>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            addr_bus: 0,
            data_bus: None,
            mem_active: false,
            read_mem: true,
            io_latch: true,
            reg1: 0,
            reg2: 0,
            crr_instr: None,
            instr_cache: VecDeque::from_iter([1, 50, 2, 51, 5, 3, 52, 0, 0, 0]),
        }
    }
}

impl Chip for CPU {
    fn tick(&mut self) {
        // println!("reg1:{}\treg2:{}", self.reg1, self.reg2);

        match &mut self.crr_instr {
            Some((instr, ctr)) => {
                if *ctr != 0 {
                    *ctr -= 1;
                    return;
                }
                match instr {
                    0 => return,
                    1 => self.reg1 = self.data_bus.unwrap(),
                    2 => self.reg2 = self.data_bus.unwrap(),
                    _ => {}
                }
                self.mem_active = false;
                self.crr_instr = None;
            }
            None => {
                if self.instr_cache.is_empty() {
                    self.mem_active = false;
                    self.read_mem = true;
                    self.crr_instr = None;
                    return;
                }
                let instr = self.instr_cache.pop_front().unwrap();
                self.mem_active = false;
                match instr {
                    0 => return,
                    1 | 2 => {
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.mem_active = true;
                        self.read_mem = true;
                        self.addr_bus = addr;
                        self.data_bus = Some(0);
                        self.io_latch = true;
                        self.crr_instr = Some((instr, 1));
                    }
                    3 => {
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        self.io_latch = false;
                        self.data_bus = Some(self.reg1);
                    }
                    4 => {
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        // self.io_latch = false;
                        self.data_bus = Some(self.reg2);
                    }
                    5 => {
                        self.reg1 += self.reg2;
                        // println!("reg1 = {}", self.reg1);
                    }
                    _ => {}
                }
            }
        }
    }
}
