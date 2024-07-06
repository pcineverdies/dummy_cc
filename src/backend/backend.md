# Backend

## Instruction Selection

As the IR adopted in the middle-end is as general as possible, some nodes require many instructions to be executed.
This is the case of the branches: in the IR the available conditions are `gt`, `ge`, `le`, `lt`, `s` and `ns`, while the RV32I ISA provides `ge` and `lt` only.
However, being a RISC setup, the limited number of available instructions makes the translation simple enough. 

Some attention has to be put on the _arithmetic immediate_ instructions, which are not implemented in the IR.
In order to use these instructions, we remember which registers contain constants, and we substitute them with their constant whenever is possible.
At the end of the translation, some loads of constants into registers might be useless, as in this case:

```asm
main:
    addi x1, x0, 10
    addi x2, x0, 20
    addi a0, x1, 20
    ret
```

If a register is not used, then the instruction is removed from the list.

## Instruction Scheduling

No effort is put on instruction scheduling.

## Stack handling

The stack is full descending. This means that the stack pointer decreases as more space is needed, and it always points to a used location.
The stack pointer's value is always a multiple of 16. 
At the beginning of a function, some space is left for its activation record. The activation record of a function contains:

- The space for the registers `ra` and `s0`, which are always saved;
- The space for the registers `t0...t6` and `s1...s11`. 
The former registers are saved before a function call if their value is going to be used afterwards; 
the latter registers are saved at the beginning of a function (and saved at its end) in case they are used at least once in the function.
- The space for local variables.

The way `t` and `s` registers are used is compliant with the RISC-V ABI. `t` registers are saved by the caller, while `s` by the callee in case of modification.
Arrays are stored on the stack. `SP` is decreased to leave the appropriate space for them.
The stack is also used to store the arguments of a function when the 8 available registers `a0...a7` are not enough.
The base of the activation record is stored in `s0`

## Register allocation

In instruction selection, the general registers are virtual, keeping the same names of those in the IR. 
A last pass is required to allocate them to physical registers.
When a physical register is required, we search among `t0..s9` which one is available and we use it. 
When we use a register, we also check if the original virtual register is in the _LIVE-OUT_ list of the current instruction.
This means that we cover all the instructions that might be reached after the one we are considering, taking into account jumps and branches.
If we realized that the virtual register is not required anymore, the physical register gets free.

It might happen that no registers are free. In this case, we need to store the value of the virtual register on the memory.
To do so, the decision employed was to adopt a second stack, pointer by the register `tp`, on which virtual registers are allocated.
When a register needs to be used, its value is stored into `s10` or `s11`.
If the destination register is on memory, `s10` is used, followed by a store. The `tp` stack is handled in a full descending manner.
Instead of a second stack, `sp` could be used as well, by reserving extra space at the bottom of it. 
However, this seemed to be the cleanest way with the respect to the previous decisions.
