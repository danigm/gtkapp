//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

mod custom;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::new_with_label("Click me!");
    button.connect_clicked(|_btn| { println!("CLICKED!") });

    window.add(&button);

    window.show_all();

    custom::test();
}

fn main() {
    let application = gtk::Application::new("net.danigm.gtkApp",
                                            Default::default())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
