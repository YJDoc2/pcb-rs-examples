use pcb_rs::Chip;
use std::collections::VecDeque;

// instructions are :
// 0 : stop
// 1 : read next byte addr and store in reg1
// 2 : read next byte addr and store in reg2
// 3 : write reg1 to next addr
// 4 : write reg2 to next addr
// 5 : add reg1 and reg2 and store in reg1

const INSTRUCTION_CACHE_LENGTH: usize = 10;

// the u8 stores ram address that is being fetched
enum FetchState {
    Priming, // need a better name to denote `this will start the fetch`
    Blocked(u8),
    Open(u8), //  give a better name to indicate `not-blocked on ram fetch`
}

enum CPUState {
    Hlt,
    InstrFetch(FetchState),
    Executing(u8, u8),
    Idle,
}

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
    state: CPUState,
    instr_ctr: u8,
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
            state: CPUState::InstrFetch(FetchState::Priming),
            instr_ctr: 0,
            instr_cache: VecDeque::new(),
        }
    }
}

impl Chip for CPU {
    fn tick(&mut self) {
        // println!("reg1:{}\treg2:{}", self.reg1, self.reg2);
        match self.state {
            CPUState::Hlt => return,
            CPUState::InstrFetch(ref state) => match state {
                FetchState::Priming => {
                    self.mem_active = true;
                    self.read_mem = true;
                    self.addr_bus = self.instr_ctr;
                    self.data_bus = Some(0);
                    self.io_latch = true;
                    self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                }
                FetchState::Blocked(addr) => {
                    self.state = CPUState::InstrFetch(FetchState::Open(*addr));
                    self.mem_active = false;
                    self.read_mem = true;
                }
                FetchState::Open(addr) => {
                    if self.instr_cache.len() < INSTRUCTION_CACHE_LENGTH {
                        self.instr_cache.push_back(self.data_bus.unwrap());
                        let next_addr = (addr + 1) % 255;
                        self.mem_active = true;
                        self.read_mem = true;
                        self.addr_bus = (*addr + 1) % 255;
                        self.data_bus = Some(0);
                        self.io_latch = true;
                        self.state = CPUState::InstrFetch(FetchState::Blocked(next_addr));
                    } else {
                        self.state = CPUState::Idle;
                        self.mem_active = false;
                        self.read_mem = true;
                    }
                }
            },
            CPUState::Executing(ctr, instr) => {
                if ctr != 0 {
                    self.state = CPUState::Executing(ctr - 1, instr);
                    return;
                }
                match instr {
                    0 => return,
                    1 => self.reg1 = self.data_bus.unwrap(),
                    2 => self.reg2 = self.data_bus.unwrap(),
                    _ => {}
                }
                self.mem_active = false;
                self.state = CPUState::Idle;
            }
            CPUState::Idle => {
                if self.instr_cache.is_empty() {
                    self.mem_active = false;
                    self.read_mem = true;
                    self.state = CPUState::Hlt;
                    return;
                }
                let instr = self.instr_cache.pop_front().unwrap();
                self.mem_active = false;
                match instr {
                    0 => self.state = CPUState::Hlt,
                    1 | 2 => {
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.mem_active = true;
                        self.read_mem = true;
                        self.addr_bus = addr;
                        self.data_bus = Some(0);
                        self.io_latch = true;
                        self.state = CPUState::Executing(1, instr);
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
