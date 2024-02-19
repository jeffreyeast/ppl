//  Draws a vector on the screen

use gtk::prelude::*;
use crate::graphics::{draw::{Line, LineType}, gtk_context::GtkContext};

use super::GraphicsRequest;


pub struct Vector {
    dx: i32,
    dy: i32,
    format: LineType,
}

impl GraphicsRequest for Vector {
    fn execute(&self, gtk_context: &mut GtkContext) -> Result<(),String> {
        let line = Line::new ((self.dx, self.dy), self.format, gtk_context.line_width);
        gtk_context.window_contents.push(Box::new(line));
        gtk_context.drawing_area.as_ref().unwrap().queue_draw();
        Ok(())
    }

    fn execute2(&self) -> Result<(),String> {
        panic!("Not implemented");
    }
}

impl Vector {
    pub fn as_dotted(dx: i32, dy: i32) -> Self {
        Vector { dx: dx, dy: dy, format: LineType::Dotted}
    }

    pub fn as_solid(dx: i32, dy: i32) -> Self {
        Vector { dx: dx, dy: dy, format: LineType::Solid}
    }
}