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

// 255 : nop

// TODO
// port these to compiler.js as well

const INSTRUCTION_CACHE_LENGTH: usize = 8;

// the u8 stores ram address that is being fetched
#[derive(Debug)]
enum FetchState {
    Priming, // need a better name to denote `this will start the fetch`
    Blocked(u8),
    Open(u8), //  give a better name to indicate `not-blocked on ram fetch`
}

#[derive(Debug)]
enum CPUState {
    Hlt,
    InstrFetch(FetchState),
    Executing(u8, u8),
    Idle,
}

// mainly for wasm binding
pub struct CPUFlagReg {
    pub addr_bus: u8,
    pub data_bus: Option<u8>,
    pub mem_active: bool,
    pub read_mem: bool,
    pub io_latch: bool,
    pub instr_ptr: u8,
    pub reg1: u8,
    pub reg2: u8,
    pub zero: bool,
    pub gt: bool,
    pub lt: bool,
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
    instr_ptr: u8,
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
            instr_ptr: 0,
            instr_cache: VecDeque::new(),
            reg1: 0,
            reg2: 0,
            zero: false,
            gt: false,
            lt: false,
        }
    }

    // for wasm binding
    pub fn get_reg_flags(&self) -> CPUFlagReg {
        CPUFlagReg {
            addr_bus: self.addr_bus,
            data_bus: self.data_bus,
            mem_active: self.mem_active,
            read_mem: self.read_mem,
            io_latch: self.io_latch,
            instr_ptr: self.instr_ptr,
            reg1: self.reg1,
            reg2: self.reg2,
            zero: self.zero,
            gt: self.gt,
            lt: self.lt,
        }
    }

    pub fn get_instr_cache(&self) -> Vec<u8> {
        Vec::from(self.instr_cache.clone())
    }

    pub fn get_state(&self) -> String {
        format!("{:?}", self.state)
    }

    fn queue_ram_fetch(&mut self, addr: u8) {
        self.mem_active = true;
        self.read_mem = true;
        self.addr_bus = addr;
        self.data_bus = Some(0);
        self.io_latch = true;
    }

    fn set_flags(&mut self) {
        self.zero = self.reg1 == 0;

        self.lt = self.reg1 < self.reg2;

        self.gt = self.reg1 > self.reg2;
    }
}

impl Chip for CPU {
    fn tick(&mut self) {
        // println!("reg1:{}\treg2:{}", self.reg1, self.reg2);
        match self.state {
            CPUState::Hlt => return,
            CPUState::InstrFetch(ref state) => match state {
                FetchState::Priming => {
                    self.queue_ram_fetch(self.instr_ptr);
                    self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                }
                FetchState::Blocked(addr) => {
                    self.state = CPUState::InstrFetch(FetchState::Open(*addr));
                    self.mem_active = false;
                    self.read_mem = true;
                }
                FetchState::Open(addr) => {
                    if self.instr_cache.len() < INSTRUCTION_CACHE_LENGTH {
                        self.instr_cache.push_back(self.data_bus.unwrap());
                        let next_addr = addr.wrapping_add(1);
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
                    self.queue_ram_fetch(self.instr_ptr);
                    self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                    return;
                }
                // this unwrap will not fail
                let instr = self.instr_cache.pop_front().unwrap();
                self.mem_active = false;
                match instr {
                    0 => self.state = CPUState::Hlt,
                    1 | 2 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.queue_ram_fetch(addr);
                        self.state = CPUState::Executing(1, instr);
                        self.instr_ptr += 1; // addr byte
                    }
                    3 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        self.io_latch = false;
                        self.data_bus = Some(self.reg1);
                        self.instr_ptr += 1; // addr byte
                    }
                    4 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        self.io_latch = false;
                        self.data_bus = Some(self.reg2);
                        self.instr_ptr += 1; // addr byte
                    }
                    5 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg1 = v;
                        self.instr_ptr += 1;
                    }
                    6 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg2 = v;
                        self.instr_ptr += 1;
                    }
                    7 => {
                        if self.instr_cache.len() < 2 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let addr = self.instr_cache.pop_front().unwrap();
                        let v = self.instr_cache.pop_front().unwrap();
                        self.addr_bus = addr;
                        self.mem_active = true;
                        self.read_mem = false;
                        self.io_latch = false;
                        self.data_bus = Some(v);
                        self.instr_ptr += 2; //  1 addr + 1 value
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
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg1 += v;
                        self.set_flags();
                        self.instr_ptr += 1;
                    }
                    11 => {
                        self.reg1 -= self.reg2;
                        self.set_flags();
                    }
                    12 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
                            return;
                        }
                        let v = self.instr_cache.pop_front().unwrap();
                        self.reg1 -= v;
                        self.set_flags();
                        self.instr_ptr += 1;
                    }
                    13 | 14 | 15 | 16 => {
                        if self.instr_cache.len() < 1 {
                            self.queue_ram_fetch(self.instr_ptr);
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(self.instr_ptr));
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
                            self.instr_cache.clear();
                            self.state = CPUState::InstrFetch(FetchState::Blocked(addr));
                            self.instr_ptr = addr;
                            return;
                        }
                        self.instr_ptr += 1;
                    }
                    255 => {
                        // nop
                    }

                    _ => {}
                }
                // this accounts for instruction byte
                // this is after the match, as if we have an incomplete instruction, the
                // instr_ptr must be set to the value of instruction byte
                // eg if add immediate is at 51, and cache is empty, the cache fill needs to fill from 51, not 52
                self.instr_ptr += 1;
            }
        }
    }
}
