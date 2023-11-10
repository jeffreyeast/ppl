//  This module holds the sequencer that controls interpretation of the nodes in the executable.

use std::rc::Rc;

use crate::{execution::{runtime::{executable::Executable, invocation::ExecutionState}, 
    value::{Value, sequence::SequenceInstance}, execute}, 
    workspace::{WorkSpace, debug::DebugOption}, utility::convert_escape_sequences};

fn execute_nodes(workspace: &WorkSpace) -> Result<Option<Value>,String> {
    loop {

        //  Get the most recent invocation context

        if let Some(invocation) = workspace.current_invocation() {

            if invocation.get_execution_state() == ExecutionState::Stopped {
                break;
            }

            //  If the user pressed ^C, stop execution

            if workspace.get_execution_sentinal().is_stop_requested() {
                invocation.set_execution_state(crate::execution::runtime::invocation::ExecutionState::Stopped);
                return Err(format!("interrupt!"));
            }

            //  Get the next node to execute and do so

            if let Some((index, ref node)) = invocation.next_node() {
                let current_statement = invocation.as_executable().get_statement_from_node_index(index);

                if let Some(ref statement) = current_statement {
                    if invocation.get_execution_state() != ExecutionState::Resumed && statement.is_stop_set() && invocation.as_executable().is_first_executable_node(statement, index) {
                        invocation.set_execution_state(ExecutionState::Stopped);
                        return Err(format!("Stop requested"));
                    }
                }

                if workspace.debug_options.borrow().is_set(&DebugOption::Execution) {
                    match current_statement {
                        Some(statement) => println!("{}) {}: [{}]{:?}", statement.as_line_number(), convert_escape_sequences(statement.as_source().as_str()), index, node),
                        None => println!("{:?}", node),
                    }
                    
                    workspace.dump_value_stack();
                }
                if workspace.debug_options.borrow().is_set(&DebugOption::StackUsage) {
                    println!("\t\t{}) {:?}\t{}", index, node, workspace.get_stack_size());
                }

                invocation.set_execution_state(crate::execution::runtime::invocation::ExecutionState::Executing);
                execute(node.as_ref(), workspace)?;
                invocation.set_execution_state(crate::execution::runtime::invocation::ExecutionState::NotExecuting);

            } else {

                //  We're done executing this invocation, so return to the preceeding invocation

                workspace.end_invocation();
            }
        } else {
            break;
        }
    }

    //  Return the most recent statement's value to the caller

    match workspace.get_last_statement_value() {
        Some(value) => return Ok(Some(value)),
        None => return Ok(Some(SequenceInstance::construct_string_sequence(""))),
    }
}

pub fn start_execution(executable: &Rc<Executable>, workspace: &WorkSpace) -> Result<Option<Value>,String> {
    workspace.start_immediate_mode(executable);
    execute_nodes(workspace)
}
