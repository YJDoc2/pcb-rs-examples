use cpu::CPU;
use pcb_rs::{pcb, Chip};
use ram::Memory;

pcb!(PCB{
    chip cpu;
    chip mem;

    cpu::addr_bus - mem::addr;
    cpu::data_bus - mem::data;
    cpu::mem_active - mem::is_active;
    cpu::read_mem - mem::is_read;

});

fn main() {
    let mem = Box::new(Memory::new());
    let cpu = Box::new(CPU::new());
    let temp = PCBBuilder::new();
    let temp = temp.add_chip("cpu", cpu).add_chip("mem", mem);
    let mut pcb = temp.build().unwrap();

    for _ in 0..10 {
        pcb.tick();
    }
}
