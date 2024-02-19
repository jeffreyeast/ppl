//  Establishes the current cursor position in the graphics window

use crate::graphics::{gtk_context::GtkContext, draw::MoveTo};

use super::GraphicsRequest;


pub struct SetPoint {
    x: i32,
    y: i32,
}

impl GraphicsRequest for SetPoint {
    fn execute(&self, gtk_context: &mut GtkContext) -> Result<(),String> {
        let setpoint = Box::new(MoveTo::new(self.x, self.y));
        gtk_context.window_contents.push(setpoint);
        Ok(())
    }

    fn execute2(&self) -> Result<(),String> {
        panic!("Not implemented");
    }
}

impl SetPoint {
    pub fn new(x: i32, y: i32) -> Self {
        SetPoint { x: x, y: y }
    }
}