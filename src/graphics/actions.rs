//  This module holds the requests for the graphics worker thread

use super::gtk_context::GtkContext;

pub mod clear;
pub mod close;
pub mod prepare;
pub mod print;
pub mod setlinewidth;
pub mod setpoint;
pub mod vector;



pub trait GraphicsRequest {
    fn execute(&self, gtk_context: &mut GtkContext) -> Result<(),String>;
    fn execute2(&self) -> Result<(),String>;
}

#[derive(Debug)]
pub struct DeltaPosition {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct WindowConfiguration {
    pub window_width: i32,
    pub window_height: i32,
    pub title: String,
}

