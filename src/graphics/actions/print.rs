//  Prints text on the graphics screen

use gtk::prelude::*;
use crate::graphics::{draw::{Dot, Text}, gtk_context::GtkContext};

use super::GraphicsRequest;


pub struct Print {
    s: String,
}

impl GraphicsRequest for Print {
    fn execute(&self, gtk_context: &mut GtkContext) -> Result<(),String> {
        if self.s == "." {
            let dot = Dot::new();
            gtk_context.window_contents.push(Box::new(dot));
        } else {
            let text = Text::new(&self.s);
            gtk_context.window_contents.push(Box::new(text));
        }
        gtk_context.drawing_area.as_ref().unwrap().queue_draw();
        Ok(())
    }

    fn execute2(&self) -> Result<(),String> {
        panic!("Not implemented");
    }
}

impl Print {
    pub fn new(s: &str) -> Self {
        Print { s: String::from(s) }
    }
}