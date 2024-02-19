//  This module holds the command loop for the spawned graphics thread

use std::{sync::Arc, cell::RefCell, time::Duration};

use super::{GraphicsContext, gtk_context::GtkContext, GraphicsStates, draw::MoveTo};
use gtk::{Application, ApplicationWindow, glib, prelude::*, DrawingArea,
    cairo::Context, ScrolledWindow};

const APP_ID: &str = "com.github.jeffreyeast.ppl";

thread_local! {
    static  MAIN_WINDOW_WIDTH: RefCell<i32> = RefCell::new(0);
    static  MAIN_WINDOW_HEIGHT: RefCell<i32> = RefCell::new(0);
    static  MAIN_WINDOW_TITLE: RefCell<String> = RefCell::new(String::new());
    pub static  GTK_CONTEXT: RefCell<Option<GtkContext>> = RefCell::new(None);
}



// The graphics thread's lifetime matches that of the program -- otherwise GTK will complain if it's
// accessed on more than one thread

pub fn command_loop(graphics_context: Arc<GraphicsContext>) {
    let gtk_context = GtkContext::new(graphics_context.clone());
    GTK_CONTEXT.with(|context| { *context.borrow_mut() = Some(gtk_context) });

    loop {
        let request = graphics_context.worker_receive_request().expect("Worker thread failed recv");
        request.execute2().expect("Prepare failed");
    }
}

pub fn create_application(width: i32, height: i32, title: &str) -> Result<(),String> {
    //  Create new app

    let app = Application::builder().application_id(APP_ID).build();

    MAIN_WINDOW_WIDTH.with(|main_window_width| { *main_window_width.borrow_mut() = width; });
    MAIN_WINDOW_HEIGHT.with(|main_window_height| { *main_window_height.borrow_mut() = height; });
    MAIN_WINDOW_TITLE.with(|main_window_title| { *main_window_title.borrow_mut() = String::from(title); });

    //  Handle the activate signal for the app

    app.connect_activate(create_application_window);

    //  And run it

    app.run();

    reset();

    Ok(())
}

fn create_application_window(app: &Application) {

    //  All the drawing is done in a DrawingArea

    let drawing_area = DrawingArea::builder()
        .build();
    drawing_area.set_content_height(MAIN_WINDOW_HEIGHT.with(|height| { *height.borrow() } ));
    drawing_area.set_content_width(MAIN_WINDOW_WIDTH.with(|width| { *width.borrow() }));
    drawing_area.set_draw_func(draw_window_contents);
    
    let scrolled_window = ScrolledWindow::builder()
        .child(&drawing_area)
        .build();

    //  Create window with title

    let window = ApplicationWindow::builder()
        .application(app)
        // .width_request(MAIN_WINDOW_WIDTH.with(|main_window_width| { *main_window_width.borrow() }))
        // .height_request(MAIN_WINDOW_HEIGHT.with(|main_window_height| { *main_window_height.borrow() }))
        .width_request(200)
        .height_request(200)
        .title(MAIN_WINDOW_TITLE.with(|main_window_title| { main_window_title.borrow().clone() }))
        .hexpand(true)
        .vexpand(true)
        .child(&scrolled_window)
        .build();
 
    // And present it

    window.present();

    GTK_CONTEXT.with(|gtk_context| {
        if let Some(gtk_context) = &mut *gtk_context.borrow_mut() {
            gtk_context.main_window = Some(window);
            gtk_context.drawing_area = Some(drawing_area);
            *gtk_context.graphics_context.graphics_state.lock().unwrap() = GraphicsStates::Ardmode;
            let request_complete = gtk_context.graphics_context.request_complete.clone();
            request_complete.signal();
            }
    });

    // Start up a timer to periodically poll for a command from the main thread

    glib::source::timeout_add_local(Duration::from_millis(100), move || {
        poll_for_request_periodically()
    });
}

fn draw_window_contents(_drawing_area: &DrawingArea, context: &Context, _x: i32, _y: i32) {
    let transform_matrix = gtk::cairo::Matrix::new(1.0, 0.0, 0.0, -1.0, 600.0, 600.0);
    context.set_matrix(transform_matrix);

    GTK_CONTEXT.with(|gtk_context| {
        if let Some(gtk_context) = &mut *gtk_context.borrow_mut() {
            for drawable in &gtk_context.window_contents {
                drawable.draw(context);
            }
        }
    })
}

fn poll_for_request_periodically() -> glib::ControlFlow {
    return GTK_CONTEXT.with(|gtk_context| {
        if let Some(gtk_context) = &mut *gtk_context.borrow_mut() {
            if *gtk_context.graphics_context.graphics_state.lock().unwrap() != GraphicsStates::Idle {
                let optional_request = gtk_context.graphics_context.graphics_thread_request_channel.lock().unwrap().try_recv();
                match optional_request {
                    Ok(request) => {
                        request.execute(gtk_context).expect("Graphics thread execute failed");
                        gtk_context.graphics_context.request_complete.signal();
                        glib::source::idle_add_local(move || {
                            poll_for_request_continuously()
                        });
                        return glib::ControlFlow::Break;
                    },
                    Err(_) => {
                        return glib::ControlFlow::Continue;
                    },
                }
            }
        }
        return glib::ControlFlow::Break
    });
}

fn poll_for_request_continuously() -> glib::ControlFlow {
    return GTK_CONTEXT.with(|gtk_context| {
        if let Some(gtk_context) = &mut *gtk_context.borrow_mut() {
            if *gtk_context.graphics_context.graphics_state.lock().unwrap() != GraphicsStates::Idle {
                let optional_request = gtk_context.graphics_context.graphics_thread_request_channel.lock().unwrap().try_recv();
                match optional_request {
                    Ok(request) => {
                        request.execute(gtk_context).expect("Graphics thread execute failed");
                        gtk_context.graphics_context.request_complete.signal();
                        gtk_context.poll_counter = 0;
                        return glib::ControlFlow::Continue
                    },
                    Err(_) => {
                        if gtk_context.poll_counter > 1000 {
                            glib::source::timeout_add_local(Duration::from_millis(100), move || {
                                poll_for_request_periodically()
                            });
                            return glib::ControlFlow::Break
                        } else {
                            gtk_context.poll_counter += 1;
                            return glib::ControlFlow::Continue
                        }
                    },
                }
            }
        }
        return glib::ControlFlow::Break
    });
}

fn reset() {
    GTK_CONTEXT.with(|gtk_context| {
        if let Some(gtk_context) = &mut *gtk_context.borrow_mut() {
            *gtk_context.graphics_context.graphics_state.lock().unwrap() = GraphicsStates::Idle;
            gtk_context.window_contents.clear();
            gtk_context.window_contents.push(Box::new(MoveTo::new(0, 0)));
            gtk_context.poll_counter = 0;
            gtk_context.line_width = 1;
            gtk_context.drawing_area = None;
            gtk_context.main_window = None;
        }
    });
}
