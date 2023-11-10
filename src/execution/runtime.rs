//  This module holds the definitions of runtime structures. These are produced by the parser and interpreted during execution.
//
//  There are three series of numbers related to execution:
//
//      Line Numbers -- these are visible to the user, start at 1 and ascend by 1 at every newline character
//      Statement Index --  Origin-1 index into the current statement block's member statements vector. 
//      Node index -- Origin-0 index into the executable's node array.
//
//  Note that line numbers are unique to a statement (assuming we never allow multiple statements on a line), but that statement indices
//  are unique only within a statement block.
//
//  Node indici are relatively unrelated to either line numbers of statement indici, since their order if generation only roughly parallels
//  the order of statements.


pub mod debug;
pub mod executable;
pub mod invocation;
pub mod stack_usage;
pub mod statement;

