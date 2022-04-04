# RAM

This contains a simple 256 byte RAM implementation, done using `derive(Chip)` macro.

This exposes :

- 8 bit addr (address bus) in form of `u8`
- 8 bit data (data bus) in form of `Option<u8>`
- 1 bit is_active (chip select) in form of bool
- 1 bit is_read (read/write) in form of bool

This takes 1 tick to store data, and 1 tick to put data on data bus into memory. Thus to read data, one will need two ticks - 1 tick where address is placed on the address bus, next tick the data will be available on the data bus. To write data, one will need to put data and address on respective buses, and in the next tick, it will be stored. See [PCB](../pcb/Readme.md) to see an example of interfacing this.

To read the data :

- make is_active `true`
- make is_read `true`
- set addr as value of the memory location to read
- set data as `Some(_)` this is so that the data bus will be un-tristated, any value will do, as it will be overwritten

In the next tick the data will be `Some(data_at_mem_loc)`

To store data :

- make is_active `true`
- make is_read `false`
- set addr as the memory location to store data at
- set data as Some(data) to be stored

In next tick, the data will be stored.
