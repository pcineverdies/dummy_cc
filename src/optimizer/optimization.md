# Optimization

## Level 0

No optimization is done at this level. 
This means that all that all the declared variables are allocated on the stack, and each time we need them their value is read back from the stack.
Whenever a constant is required, it is loaded in a register without checking if it was loaded before.
Everything is re-computed when needed.

## Level 1

### Variables cache
When a variable is read, we can store the number of the register in which the variable was read in order to get it back once the need the variable again.
Each time we read the variable again we make sure to update the register containing its value. 
When we use a `store` operation on an address which does not correspond to a variable, we cannot be sure which point of the address space is modified. 
Thus, we invalidate all the cached variables.

```c
int a = 0;  // Let's say `a` is stored @0x1000.
            // We can be sure `a` has value 0.

int b = 10;

int* p = (int*)0x1000;

b = 20;     // We are sure that `a`is not modified 
            // due to this instruction.

*p = 30;    // We cannot know which element has been modified, 
            // so we invalidate all the cached variables. Next
            // time we need `a`, we read it from the stack.
```

Clearly, this might be avoided by using some dataflow analysis to understand which area of the address spaces is modified by `*p = 30;`.

We must also be cautious about branches.

```c
int a = 0;

if(a > 10) {
    a = 30;
}

int b = a;
```

When using `a` to declare `b`, we cannot be sure if the value of `a` is `0` or `30` (giving for granted that we don't do an analysis of the value of `a`. 
In this situation it is obvious that `a` will be 0).  For this reason `a` must be read again from the stack. However, this happens only because `a` is modified 
in the branch. Let's consider this case:

```c
int a = 0;
int counter = 0;

while(counter < 10) {
    counter = counter + 1;
}

int b = a;
```

Since `a` is not modified in the loop, we are sure `a` is still 0 when used for `b`, and its cached value can be used again.

### Constants cache
When the code requires a constant, its value is always loaded in a register. 
This means that the same register can be used again when that same constant is required again.

```c
int a = 10;

// ...

int counter = 0;
while(counter < 10) { ... }
```

In the condition of the while, the same constant is to be used again.
Constants can be used only if previously declared in the same (or above) scopes. Let's consider this example:

```c
int a = 0;
int b = 20;

if (a > b) {
    a = 20;         // 20 can be used again since it was stored in a register while declaring `b`.
    b = 30;         // 30 has to be stored in a register, since it was never encountered.
}

a = 30;             // 30 was not encountered in this scope, so it has to be loaded, although it was
                    // stored in a register in the `if` branch.
```

### Binary operation cache
What has been said for variables and constants also holds for binary operations. If `v1 op v2` has been already computed, and stored in `v3`, 
the same register can be used for the same operation later in the code. This is extremely useful when combined with the two above optimizations:

```c

int a = 10; 
int b = a;

if(a + b < 20) {
    int c = b + 10;
}
```

The value of `a` is associated to the register storing `10`, let's say `v1`. The value of `b` is associated to the register storing `a`, `v1` again.
In the if condition we compute `a + b`, which corresponds to `v1 + v1`. Later in the if block we find the operation again, and the value is reused.
Clearly, branches affect which operations can be reused or not.

## Level 2

All the optimizations from level 1 are adopted as well. 

### Dead code removal
Look at the description of `dead_code_removal` in [optimizer.rs](./optimizer.rs).

### Control flow removal
Look at the description of `control_flow_removal` in [optimizer.rs](./optimizer.rs).
