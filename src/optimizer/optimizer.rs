use crate::lirgen::irnode::IrNode;

use IrNode::*;

/// struct Optimizer
///
/// Object which allows the optimization of the linear IR
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Optimizer {
    opt: u32,
}

impl Optimizer {
    /// Optimizer::new
    ///
    /// Create a new optimizer given a specific optimization level.
    ///
    /// @in opt [u32]: optimization level
    /// @result [Optimizer]: built object
    pub fn new(opt: u32) -> Optimizer {
        Optimizer { opt }
    }

    /// Optimizer::optimizer
    ///
    /// Run the optimizations algorithms
    ///
    /// @in ir [IrNode]: program to be optimized
    /// @result [IrNode]: optimized program
    pub fn optimize(&mut self, mut ir: IrNode) -> IrNode {
        let mut is_changed: bool;

        // Apply the algorithms until nothing changes anymore. At that point, the final result is
        // provided back.
        loop {
            (ir, _) = self.dead_code_removal(ir.clone());
            (ir, is_changed) = self.control_flow_removal(ir.clone());
            if !is_changed {
                break;
            }
        }
        return ir;
    }

    /// Optimizer::control_flow_removal
    ///
    /// The algorithm is in charge of removing useless jumps, in which the label of the destination
    /// follows the jump instruction, as in the following case
    ///
    ///     ...
    ///     jmp [cond] [operands] L_x
    /// L_x:
    ///     ....
    ///
    /// Since each jump is associate to a single label and viceversa, when a jump is removed the
    /// corresponding label can be removed as well. The algorithm makes sense only if some
    /// instructions were removed by the `dead_code_removal` algorithm. The IR generator does not
    /// generate such situations.
    ///
    /// @in ir [IrNode]: program to optimize
    /// @result [(IrNode, bool)]: result of the optimization, whether some changes have been done
    /// or not.
    fn control_flow_removal(&mut self, ir: IrNode) -> (IrNode, bool) {
        let mut is_changed = false;

        // The initial node is always a program made of function declarations. The algorithm runs
        // on each function declaration individually
        if let Program(functions_list) = ir {
            // New list of functions
            let mut new_functions_list: Vec<IrNode> = vec![];
            for function in functions_list {
                // New list of nodes for the current function
                let mut new_nodes: Vec<IrNode> = vec![];
                if let FunctionDeclaration(n, t, args, nodes) = function {
                    // Do not optimize `init` function
                    if n == "init" {
                        new_functions_list.push(FunctionDeclaration(n, t, args, nodes.clone()));
                        continue;
                    }

                    // to_remove[i] ends up being true if node `i` is to be removed
                    let mut to_remove = vec![false; nodes.len()];

                    // Go through each node of the list in reverse order
                    for i in (0..nodes.len()).rev() {
                        // If the current node is a branch, analyze the following nodes. If one of
                        // them is the destination of the jump, mark as "to be removed" both the
                        // branch and the label
                        if let Branch(_, _, _, _, label) = nodes[i] {
                            for j in (i + 1)..nodes.len() {
                                if let Label(l) = &nodes[j] {
                                    if *l == label {
                                        to_remove[i] = true;
                                        to_remove[j] = true;
                                        is_changed = true;
                                        break;
                                    }

                                // The node is not a label, thus the branch cannot be removed
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // Create a new list of nodes containing the nodes which cannot be removed
                    for i in 0..nodes.len() {
                        if !to_remove[i] {
                            new_nodes.push(nodes[i].clone())
                        }
                    }

                    // Use the new list of nodes to build the corresponding optimized function
                    new_functions_list.push(FunctionDeclaration(n, t, args, new_nodes));
                } else {
                    panic!("Provided node to `control_flow_removal` not of type FunctionDeclaration")
                }
            }

            return (Program(new_functions_list), is_changed);
        }
        panic!("Provided node to `control_flow_removal` not of type Program")
    }

    /// Optimizer::dead_code_removal
    ///
    /// First step:
    /// Iterate over all the nodes of a function:
    ///     - save aside all the destinations of `alloc` operations
    ///
    /// Second step:
    /// <CriticalRegisters> = []
    /// While some new registers are added to <CriticalRegisters>
    ///     Iterate over all the nodes of a function in revers order:
    ///         - mark as critical the following kind of nodes, considering that a critical nodes
    ///           cannot be removed by the algorithm:
    ///             - Call instruction
    ///             - Store to non-local pointers
    ///             - Return instructions
    ///             - Branches
    ///             - Labels
    ///             - Nodes whose destination is in the set <CriticalRegisters>
    ///
    ///           For each critical node, save in the set <CriticalRegisters> their sources
    ///
    /// Third step:
    /// If a node is not critical, remove it
    ///
    /// This algorithm remove most of the useless nodes, but cannot optimize in a meaningful way in
    /// presence of loops, due to the lack of control dependence.
    ///
    /// @in ir [IrNode] -> Program to optimize
    /// @return [(IrNode, bool)] -> Program optimized, whether something has changed or not
    fn dead_code_removal(&mut self, ir: IrNode) -> (IrNode, bool) {
        let mut is_changed = false;

        // The initial node is always a program made of function declarations. The algorithm runs
        // on each function declaration individually
        if let Program(functions_list) = ir {
            let mut new_functions_list: Vec<IrNode> = vec![];
            for function in functions_list {
                // New list of nodes for the current function
                let mut new_nodes: Vec<IrNode> = vec![];
                if let FunctionDeclaration(n, t, args, nodes) = function {
                    // Do not optimize `init` function
                    if n == "init" {
                        new_functions_list.push(FunctionDeclaration(n, t, args, nodes.clone()));
                        continue;
                    }

                    // List of critical nodes, which cannot be removed since they handle some
                    // critical registers
                    let mut is_node_critical = vec![false; nodes.len()];

                    // List of critical registers
                    let mut critical_registers: Vec<u32> = vec![];

                    // List of registers which represent the destination of a local allocation.
                    // When a store is performed, it might be useless if the value is not used
                    // again and it refers to a local variable. On the contrary, a store to a
                    // random point in the address space cannot be removed.
                    let mut local_alloc_references: Vec<u32> = vec![];

                    // For each node in the list, add the destination of an allocation to the list
                    // `local_alloc_references`
                    for (_, node) in nodes.iter().enumerate() {
                        if let Alloc(..) = node {
                            local_alloc_references.push(node.get_dest());
                        }
                    }

                    loop {
                        // Set this variable to true whether a new critical node is found
                        let mut is_added = false;

                        // For each node in reverse order
                        for i in (0..nodes.len()).rev() {
                            let node = &nodes[i];

                            // If node `i` was already set as critical, skip it
                            if is_node_critical[i] {
                                continue;
                            }

                            // A store node is critical if the destination is not a local
                            // allocation
                            if let Store(..) = node {
                                if !local_alloc_references.contains(&node.get_dest()) {
                                    critical_registers.append(&mut node.get_src());
                                    critical_registers.push(node.get_dest());
                                    is_node_critical[i] = true;
                                    is_added = true;
                                }
                            }

                            match node {
                                // return nodes, call nodes, branch nodes and label nodes are
                                // always critical
                                Return(..) | Call(..) | Branch(..) | Label(..) => {
                                    // Add the sources to the critical registers
                                    critical_registers.append(&mut node.get_src());
                                    is_node_critical[i] = true;
                                    is_added = true;
                                }
                                // The other nodes are critical only if their  destination
                                // register is among the critical registers
                                _ => {
                                    if critical_registers.contains(&node.get_dest()) {
                                        // Add the sources to the critical registers
                                        critical_registers.append(&mut node.get_src());
                                        is_node_critical[i] = true;
                                        is_added = true;
                                    }
                                }
                            }
                        }
                        // Break if no changes are done anymore
                        if !is_added {
                            break;
                        }
                    }

                    // Create a new list of nodes removing the non-critical ones
                    for i in 0..nodes.len() {
                        if is_node_critical[i] {
                            new_nodes.push(nodes[i].clone())
                        }
                    }

                    if nodes.len() != new_nodes.len() {
                        is_changed = true;
                    }

                    new_functions_list.push(FunctionDeclaration(n, t, args, new_nodes));
                } else {
                    panic!("Provided node to `dead_code_removal` not of type FunctionDeclaration")
                }
            }

            return (Program(new_functions_list), is_changed);
        }
        panic!("Provided node to `dead_code_removal` not of type Program")
    }
}
