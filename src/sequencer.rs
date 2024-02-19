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
                {
                    workspace.get_execution_sentinal_mut().clear_stop_requested();
                }
                invocation.set_execution_state(crate::execution::runtime::invocation::ExecutionState::Stopped);
                return Err(format!("interrupt!"));
            }

            //  Get the next node to execute and do so

            if let Some((index, ref node)) = invocation.next_node() {
                let current_statement = invocation.as_executable().get_statement_from_node_index(index);

                if let Some(statement) = &current_statement {
                    if invocation.get_execution_state() != ExecutionState::Resumed && statement.is_stop_set() && invocation.as_executable().is_first_executable_node(statement, index) {
                        invocation.set_execution_state(ExecutionState::Stopped);
                        return Err(format!("Stop requested"));
                    }
                }

                if workspace.debug_options.borrow().is_set(&DebugOption::Execution) ||
                   workspace.debug_options.borrow().is_set(&DebugOption::NodeInvocation) {
                    match &current_statement {
                        Some(statement) => println!("Invoking: {}) {}: [{}]{:?}", statement.as_line_number(), convert_escape_sequences(statement.as_source().as_str()), index, node),
                        None => println!("Invoking: n/a) n/a: [{}]{:?}", index, node),
                    }
                }
                if workspace.debug_options.borrow().is_set(&DebugOption::StackUsage) {
                    println!("\t\t{}) {:?}\t{}", index, node, workspace.get_stack_size());
                }

                invocation.set_execution_state(crate::execution::runtime::invocation::ExecutionState::Executing);
                execute(node.as_ref(), workspace)?;
                invocation.set_execution_state(crate::execution::runtime::invocation::ExecutionState::NotExecuting);

                if let Some(statement) = &current_statement {
                    if statement.is_trace_set() && invocation.as_executable().is_last_executable_node(statement, index) {
                        let statement_value = workspace.get_last_statement_value();
                        println!("[{}] {}", statement.as_line_number(), statement_value.unwrap_or(Value::Empty));
                    }
                }

                if workspace.debug_options.borrow().is_set(&DebugOption::Execution) || 
                   workspace.debug_options.borrow().is_set(&DebugOption::ValueStack) {
                    workspace.dump_value_stack();
                }

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

pub fn start_execution(workspace: &WorkSpace) -> Result<Option<Value>,String> {
    execute_nodes(workspace)
}

pub fn prepare_execution(executable: &Rc<Executable>, workspace: &WorkSpace) {
    workspace.start_immediate_mode(executable);
}
