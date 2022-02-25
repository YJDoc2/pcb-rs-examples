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
    pub mem: [u8; 255],
}

impl Memory {
    pub fn new() -> Self {
        let mut t = Memory {
            addr: 0,
            data: None,
            is_active: false,
            is_read: false,
            io_latch: true,
            mem: [0; 255],
        };
        // code
        // let instr = [255, 255, 255, 255, 255, 5, 55, 7, 52, 57];
        let instr = [5, 0, 6, 5, 9, 8, 12, 1, 13, 13, 8, 14, 4, 8, 3, 52, 0];
        for (m, i) in t.mem.iter_mut().zip(instr.iter()) {
            *m = *i;
        }

        // data
        t.mem[50] = 5;
        t.mem[51] = 7;

        t
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
