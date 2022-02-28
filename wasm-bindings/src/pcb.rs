use wasm_bindgen::prelude::*;

use cpu::CPU;
#[allow(unused_imports)]
use pcb_rs::{pcb, Chip}; // traits need to be imported to use the trait methods
use ram::Memory;

pcb!(PCB{
    chip cpu;
    chip mem;

    cpu::addr_bus - mem::addr;
    cpu::data_bus - mem::data;
    cpu::mem_active - mem::is_active;
    cpu::read_mem - mem::is_read;

});

#[wasm_bindgen]
pub struct PCBHandle {
    pcb: PCB,
}

#[wasm_bindgen]
pub struct CPUStateData {
    pub addr: u8,
    pub data: Option<u8>,
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

#[wasm_bindgen]
pub struct RAMStateData {
    pub addr: u8,
    pub data: Option<u8>,
    pub is_active: bool,
    pub is_read: bool,
    pub io_latch: bool,
}

#[wasm_bindgen]
impl PCBHandle {
    pub fn get_cpu_instr_cache(&self) -> Vec<u8> {
        let cpu: &CPU = self.pcb.get_chip("cpu").unwrap();
        cpu.get_instr_cache()
    }
    pub fn get_cpu_state(&self) -> String {
        let cpu: &CPU = self.pcb.get_chip("cpu").unwrap();
        cpu.get_state()
    }
    pub fn get_cpu_reg_flags(&self) -> CPUStateData {
        let cpu: &CPU = self.pcb.get_chip("cpu").unwrap();
        let data = cpu.get_reg_flags();
        CPUStateData {
            addr: data.addr_bus,
            data: data.data_bus,
            mem_active: data.mem_active,
            read_mem: data.read_mem,
            io_latch: data.io_latch,
            instr_ptr: data.instr_ptr,
            reg1: data.reg1,
            reg2: data.reg2,
            zero: data.zero,
            gt: data.gt,
            lt: data.lt,
        }
    }

    pub fn get_ram_state(&self) -> RAMStateData {
        let ram: &Memory = self.pcb.get_chip("mem").unwrap();
        let data = ram.get_state();
        RAMStateData {
            addr: data.addr,
            data: data.data,
            is_active: data.is_active,
            is_read: data.is_read,
            io_latch: data.io_latch,
        }
    }

    pub fn get_mem_array(&self) -> Vec<u8> {
        let ram: &Memory = self.pcb.get_chip("mem").unwrap();
        ram.get_mem_array()
    }
}

#[wasm_bindgen]
pub fn get_pcb_handle() -> PCBHandle {
    let mem = Box::new(Memory::new());
    let cpu = Box::new(CPU::new());
    let temp = PCBBuilder::new();
    let temp = temp.add_chip("cpu", cpu).add_chip("mem", mem);
    let pcb = temp.build().unwrap();
    PCBHandle { pcb: pcb }
}
