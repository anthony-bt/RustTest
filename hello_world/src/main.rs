extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button, ButtonBox, Orientation};

fn main() {
    let application = Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);
        window.set_resizable(true);

        // Button group
        let button_box = ButtonBox::new(Orientation::Vertical);

        // Button 1
        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        button_box.add(&button);

        // Button 2
        let button2 = Button::new_with_label("Click me 2!");
        button2.connect_clicked(|_| {
            println!("Clicked 2!");
        });
        button_box.add(&button2);

        window.add(&button_box);

        window.show_all();
    });

    application.run(&[]);
}
