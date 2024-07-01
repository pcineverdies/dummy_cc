# Linear Intermediate Representation

## Structure

The intermediate representation employs a list of instructions to represent the flow of the program.
Each instruction uses some source registers, and possibly has a destination register where the result is put.
Being a SSA-like representation, no register is assigned two times.
To simplify the implementation, the stack is used to store and retrieve values from. 
he usage of the stack limits the amount of optimization which can be done, but removes the need of the φ function.
Registers are virtual, thus endless. The backend is in charge of implementing register-allocation.

## List of instructions/nodes

### Program 

Just a list of function declarations.
The different functions are stored in sequence, without checking if the `main` function is defined or not.
The global declarations are stored at the beginning of an `init` function implemented ad hoc during the construction.
In this function, the assigned values to the global declarations are computed as well. 

### Function Declaration

This provides the declaration of a function, with the argument types and the registers in which the arguments are passed.
All the arguments are passed by value, thus they can all be handled in the same way.
I then contains the list of instructions of the function

### Alloc

This is used to allocate a value on the stack. The value can be either a single variable or an array. 
In the first case, the format is

`vx` = `alloc<type> {vy}`

Where `vx` is the register containing the address of the new variable, `vy` is an optional initialization value;
In the second case,

`vx` = `alloc<type> [vy]`

Where `vx` is the register containing the address of the new array, `vy` is the register containing the size of the array in bytes.

### Return

If the function is of type void, its structure is 

`return`

Otherwise, a source register is provided containing the return value.

`return<type> vx`


### MovC

`vx = <type> $c`

Move the constant `$C` int the register `vx`.

### Cast

`vx = <type_dest><type_src> vy`

The expression stored in `vy`, of type `type_src` is casted to `type_dest`, and saved in `vx`.

### Store

`store<type> vx, vy`

The value in `vy`, having type `type` is stored in the address contained in `vx`

### LoadA

`vx = load<type> @label`

Load the address of `@label` into the register `vx`. This is mainly used to load global variables.

### LoadR

`vx = load<type> vy`

Load in `vx` the value of the cell whose address is stored in `vy`.

### Label

`L_x:`

Destination of a jump, represented by an integer `x`.

### Call

`vx = call<type> name(vy1, vy2, ...)`

Call the function `name`, whose return type is `type`, with the arguments stored in `vy1`, `vy2`, ...

### Branch

A branch can be of three different types

- Binary: `j_binarycond<type> vx, vy Lz`: Jump to `Lz` if the binary condition between `vx` and `vy` is true. Possible conditions are: `le`, `lt`, `gt`, `ge`, `eq`, `ne`.
- Unary: `j_unarycond<type> vx Lz`: Jump to `Lz` if the unary condition on `vx` is true. Possible conditions are: `set`, `not set`
- Unconditional: `j Lz`: Always jump to `Lz`

### Binary

`vx = op <type> vy, vz `

Apply a binary operation on `vy` and `vz`, store the result in `vx`.
The available operations are: set if equal (`seq`), set if not equal (`sneq`), set if less than (`slt`), set if greater than (`sgt`), set if less equal than (`sle`), set if greater equal than (`sge`), subtract (`sub`), add (`add`), multiply (`mul`),  divide (`div`), xor (`xor`), and (`and`), or (`or`), remainder (`rem`), shift left (`sl`), shift right (`sr`);

### Unary

`vx = op <type> vy `

Apply an unary operation on `vy`, store the result in `vx`.
The available operations are: negative value (`neg`), complement (`comp`), logical opposite (`not`).

