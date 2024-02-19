//  This module contains the drawing structures and code

use gtk::cairo::Context;

pub trait Drawable {
    fn draw(&self, context: &Context);
}



#[derive(Debug)]
pub struct Dot {
    radius: i32,
}

impl Drawable for Dot {
    fn draw(&self, context: &Context) {
        let (x, y) = context.current_point().unwrap();
        context.save().expect("save failed");

        context.set_source_rgb(0f64, 0f64, 0f64);
        context.new_sub_path();
        context.arc(x, y, self.radius as f64, 0f64, 2.0 * 3.14159f64);
        context.fill().expect("fill failed");

        context.restore().expect("restore failed");
        context.move_to(x, y);
    }
}

impl Dot {
    pub fn new() -> Self {
        Dot { radius: 2 }
    }
}




#[derive(Debug,Clone, Copy)]
pub enum LineType {
    Dotted,
    Solid,
}

#[derive(Debug)]
pub struct Line {
    delta: (i32, i32),
    format: LineType,
    width: i32,
}

impl Drawable for Line {
    fn draw(&self, context: &Context) {
        match self.format {
            LineType::Dotted => context.set_dash(&vec![5f64, 3f64], 0f64),
            LineType::Solid => context.set_dash(&Vec::<f64>::new(), 0f64),
        }
        context.set_line_width(self.width as f64);
        let (x, y) = context.current_point().unwrap();
        context.line_to(x + self.delta.0 as f64, y + self.delta.1 as f64);
        let (x1, y1) = context.current_point().unwrap();
        context.stroke().expect("stroke failed");
        context.move_to(x1, y1);
    }
}

impl Line {
    pub fn new (delta: (i32, i32), format: LineType, width: i32) -> Self {
        Line { delta: delta, format: format, width }
    }
}



#[derive(Debug)]
pub struct MoveTo {
    position: (i32, i32),
}

impl Drawable for MoveTo {
    fn draw(&self, context: &Context) {
        context.move_to(self.position.0 as f64, self.position.1 as f64);
    }
}

impl MoveTo {
    pub fn new(x: i32, y: i32) -> Self {
        MoveTo { position: (x, y) }
    }
}



#[derive(Debug)]
pub struct Text {
    s: String,
}

impl Drawable for Text {
    fn draw(&self, context: &Context) {
        context.save().expect("save failed");

        let transform_matrix = gtk::cairo::Matrix::new(1.0, 0.0, 0.0, 1.0, 600.0, 600.0);
        context.set_matrix(transform_matrix);

        context.show_text(self.s.as_str()).expect("show_text failed");
        let (x1, y1) = context.current_point().unwrap();
        context.stroke().expect("stroke failed");

        context.restore().expect("restore failed");
        context.move_to(x1, -y1);
    }
}

impl Text {
    pub fn new(s: &String) -> Self {
        Text { s: s.clone() }
    }
}
