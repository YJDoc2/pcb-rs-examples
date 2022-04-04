# Basic gates

---

This contains several basic gates which are used in the other examples.All of these are chips, i.e. structs, which use `#[derive(Chip)]` and implement their processing login in the Chip trait.

Currently this has the following gates :

| Gate             | Description                                                                           |
| ---------------- | ------------------------------------------------------------------------------------- |
| Not gate         | inverts the input                                                                     |
| And gate         | output is true only if both of the inputs are true                                    |
| Or gate          | output is true if any of the input is true                                            |
| Nor gate         | output is true only if neither of the inputs is true                                  |
| Nand gate        | output is true if any of the input is false                                           |
| Xor gate         | output is true only if exactly one input is true                                      |
| And3, And4, And5 | And gates having 3,4,5 inputs respectively Output is true only if all inputs are true |
| Or3, Or4, Or5    | Or gates having 3,4,5 inputs respectively. Output is true if any of the input is true |
| Nor3             | Nor gate having 3 inputs. Output is true only if none of the inputs is true           |
