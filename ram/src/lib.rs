use pcb_rs::Chip;

#[derive(Chip)]
pub struct Memory {
    #[pin(input)]
    addr: u8,
    #[pin(io, io_latch)]
    data: Option<u8>,
    #[pin(input)]
    is_active: bool,
    #[pin(input)]
    is_read: bool,

    io_latch: bool,
    pub mem: [u8; 256],
}

pub struct MemState {
    pub addr: u8,
    pub data: Option<u8>,
    pub is_active: bool,
    pub is_read: bool,
    pub io_latch: bool,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            addr: 0,
            data: None,
            is_active: false,
            is_read: false,
            io_latch: true,
            mem: [0; 256],
        }
    }

    // for wasm binding
    pub fn get_state(&self) -> MemState {
        MemState {
            addr: self.addr,
            data: self.data,
            is_active: self.is_active,
            is_read: self.is_read,
            io_latch: self.io_latch,
        }
    }

    // for wasm binding
    pub fn get_mem_array(&self) -> Vec<u8> {
        Vec::from_iter(self.mem.iter().map(|x| *x))
    }
}

impl Chip for Memory {
    fn tick(&mut self) {
        if !self.is_active {
            self.io_latch = true;
            return;
        }

        if self.is_read {
            let addr = self.addr as usize;
            let data = self.mem[addr];
            self.data = Some(data);
            self.io_latch = false;
            // println!("reading {} : {}", addr, data);
        } else {
            let addr = self.addr as usize;
            let val = self.data.unwrap();
            self.mem[addr] = val;
            // println!("writing {} : {}", addr, val);
        }

        // println!(
        //     "50:{}\t51:{}\t52:{}",
        //     self.mem[50], self.mem[51], self.mem[52]
        // );
    }
}
