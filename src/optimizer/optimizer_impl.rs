use crate::lirgen::irnode::IrNode;

use IrNode::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Optimizer {
    opt: u32,
}

impl Optimizer {
    pub fn new(opt: u32) -> Optimizer {
        Optimizer { opt }
    }

    pub fn optimize(&mut self, mut ir: IrNode) -> IrNode {
        let mut is_changed: bool;
        loop {
            (ir, _) = self.dead_code_removal(ir.clone());
            (ir, is_changed) = self.control_flow_removal(ir.clone());
            if !is_changed {
                break;
            }
        }
        return ir;
    }

    fn control_flow_removal(&mut self, ir: IrNode) -> (IrNode, bool) {
        let mut is_changed = false;
        if let Program(functions_list) = ir {
            let mut new_functions_list: Vec<IrNode> = vec![];
            for function in functions_list {
                let mut new_nodes: Vec<IrNode> = vec![];
                if let FunctionDeclaration(n, t, args, nodes) = function {
                    // Cannot optimize init function
                    if n == "init" {
                        new_functions_list.push(FunctionDeclaration(n, t, args, nodes.clone()));
                        continue;
                    }

                    let mut to_remove = vec![false; nodes.len()];

                    for i in (0..nodes.len()).rev() {
                        if let Branch(_, _, _, _, label) = nodes[i] {
                            for j in (i + 1)..nodes.len() {
                                if let Label(l) = &nodes[j] {
                                    if *l == label {
                                        to_remove[i] = true;
                                        to_remove[j] = true;
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    for i in 0..nodes.len() {
                        if !to_remove[i] {
                            new_nodes.push(nodes[i].clone())
                        }
                    }

                    if nodes.len() != new_nodes.len() {
                        is_changed = true;
                    }

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
    /// presence of loops, due to the lack of control dependance.
    ///
    /// @in ir [IrNode] -> Program to optimize
    /// @return [(IrNode, bool)] -> Program optimized, whether something has changed or not
    fn dead_code_removal(&mut self, ir: IrNode) -> (IrNode, bool) {
        let mut is_changed = false;
        if let Program(functions_list) = ir {
            let mut new_functions_list: Vec<IrNode> = vec![];
            for function in functions_list {
                let mut new_nodes: Vec<IrNode> = vec![];
                if let FunctionDeclaration(n, t, args, nodes) = function {
                    // Cannot optimize init function
                    if n == "init" {
                        new_functions_list.push(FunctionDeclaration(n, t, args, nodes.clone()));
                        continue;
                    }

                    let mut is_node_critical = vec![false; nodes.len()];
                    let mut critical_registers: Vec<u32> = vec![];
                    let mut local_alloc_references: Vec<u32> = vec![];

                    for (_, node) in nodes.iter().enumerate() {
                        if let Alloc(..) = node {
                            local_alloc_references.push(node.get_dest());
                        }
                    }

                    let mut added;

                    loop {
                        added = false;
                        for i in (0..nodes.len()).rev() {
                            let node = &nodes[i];

                            if is_node_critical[i] {
                                continue;
                            }

                            if let Store(..) = node {
                                if !local_alloc_references.contains(&node.get_dest()) {
                                    critical_registers.append(&mut node.get_src());
                                    critical_registers.push(node.get_dest());
                                    is_node_critical[i] = true;
                                    added = true;
                                }
                            }

                            match node {
                                Return(..) | Call(..) | Branch(..) | Label(..) => {
                                    critical_registers.append(&mut node.get_src());
                                    is_node_critical[i] = true;
                                    added = true;
                                }
                                _ => {
                                    if critical_registers.contains(&node.get_dest()) {
                                        critical_registers.append(&mut node.get_src());
                                        is_node_critical[i] = true;
                                        added = true;
                                    }
                                }
                            }
                        }
                        if !added {
                            break;
                        }
                    }

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
