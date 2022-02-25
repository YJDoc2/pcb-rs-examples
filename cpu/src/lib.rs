use pcb_rs::Chip;
use std::collections::VecDeque;

// instructions are :

// 0 : stop

// 1 : read next byte addr and store in reg1
// 2 : read next byte addr and store in reg2
// 3 : write reg1 to next addr
// 4 : write reg2 to next addr

// 5 : store next byte in reg1
// 6 : store next byte in reg2
// 7 : read next byte as addr, and store next-to-next byte at addr
// 8 : xchg

// 9 : add reg1 and reg2 and store in reg1
// 10 : add next byte to reg1
// 11 : sub reg2 from reg1 and store in reg1
// 12 : sub next byte from reg1

// 13 : jump to next byte if zero
// 14 : jump to next byte if not zero
// 15 : jump if reg1 less than reg2
// 16 : jump if reg1 greater than reg2

// TODO
// port these to compiler.js as well

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
    state: CPUState,
    instr_ctr: u8,
    instr_cache: VecDeque<u8>,

    reg1: u8,
    reg2: u8,
    zero: bool,
    gt: bool,
    lt: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            addr_bus: 0,
            data_bus: None,
            mem_active: false,
            read_mem: true,
            io_latch: true,
            state: CPUState::InstrFetch(FetchState::Priming),
            instr_ctr: 0,
            instr_cache: VecDeque::new(),
            reg1: 0,
            reg2: 0,
            zero: false,
            gt: false,
            lt: false,
        }
    }

    fn queue_ram_fetch(&mut self, addr: u8) {
        self.mem_active = true;
        self.read_mem = true;
        self.addr_bus = addr;
        self.data_bus = Some(0);
        self.io_latch = true;
    }

    fn set_flags(&mut self) {
        if self.reg1 == 0 {
            self.zero = true;
        }
        if self.reg1 < self.reg2 {
            self.lt = true;
        }
        if self.reg1 > self.reg2 {
            self.gt = true;
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
                    self.queue_ram_fetch(self.instr_ctr);
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
                        self.queue_ram_fetch(next_addr);
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
                    // cache is empty, so prime fetch
                    self.queue_ram_fetch(self.instr_ctr);
                    self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                    return;
                }
                // this unwrap will not fail
                let instr = self.instr_cache.pop_front().unwrap();
                self.instr_ctr += 1;
                self.mem_active = false;
                match instr {
                    0 => self.state = CPUState::Hlt,
                    1 | 2 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.queue_ram_fetch(addr);
                        self.state = CPUState::Executing(1, instr);
                        self.instr_ctr += 1; // addr byte
                    }
                    3 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        self.io_latch = false;
                        self.data_bus = Some(self.reg1);
                        self.instr_ctr += 1; // addr byte
                    }
                    4 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        self.io_latch = false;
                        self.data_bus = Some(self.reg2);
                        self.instr_ctr += 1; // addr byte
                    }
                    5 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg1 = v;
                        self.instr_ctr += 1;
                    }
                    6 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg2 = v;
                        self.instr_ctr += 1;
                    }
                    7 => {
                        if self.instr_cache.len() < 2 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        let v = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        self.io_latch = false;
                        self.data_bus = Some(v);
                        self.instr_ctr += 2; //  1 addr + 1 value
                    }
                    8 => {
                        let t = self.reg1;
                        self.reg1 = self.reg2;
                        self.reg2 = t;
                    }
                    9 => {
                        self.reg1 += self.reg2;
                        self.set_flags();
                    }
                    10 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg1 += v;
                        self.set_flags();
                        self.instr_ctr += 1;
                    }
                    11 => {
                        self.reg1 -= self.reg2;
                        self.set_flags();
                    }
                    12 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg1 -= v;
                        self.set_flags();
                        self.instr_ctr += 1;
                    }
                    13 | 14 | 15 | 16 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ctr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ctr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        let jump;
                        match instr {
                            13 => jump = self.zero,
                            14 => jump = !self.zero,
                            15 => jump = self.lt,
                            16 => jump = self.gt,
                            _ => unreachable!(),
                        }
                        if jump {
                            self.queue_ram_fetch(addr);
                            self.state = CPUState::InstrFetch(FetchState::Blocked(addr));
                            self.instr_ctr = addr;
                            return;
                        }
                        self.instr_ctr += 1;
                    }

                    _ => {}
                }
            }
        }
    }
}
