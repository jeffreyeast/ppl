// Represents the context maintained between requests to GTK and Cairo

use std::sync::Arc;

use gtk::{ApplicationWindow, DrawingArea};

use super::{GraphicsContext, draw::{Drawable, MoveTo}};




pub struct GtkContext {
    pub graphics_context: Arc<GraphicsContext>,
    pub main_window: Option<ApplicationWindow>,
    pub drawing_area: Option<DrawingArea>,
    pub window_contents: Vec<Box<dyn Drawable>>,
    pub poll_counter: i32,
    pub line_width: i32,
}


impl GtkContext {
    pub fn new(graphics_context: Arc<GraphicsContext>) -> Self {
        GtkContext {
            graphics_context: graphics_context.clone(),
            main_window: None,
            drawing_area: None,
            window_contents: vec![Box::new(MoveTo::new(0, 0))],
            poll_counter: 0,
            line_width: 1,
        }
    }
}
