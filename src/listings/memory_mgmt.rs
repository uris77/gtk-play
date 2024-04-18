use glib::clone;
use std::{cell::Cell, rc::Rc};

use gtk::{self, glib, prelude::*, Application, ApplicationWindow, Button};

#[allow(dead_code)]
pub fn build_ui(application: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease => move |_| {
     number.set(number.get() + 1);
     button_decrease.set_label(&number.get().to_string());
    }));

    button_decrease.connect_clicked(clone!(@weak button_increase => move |_| {
     number.set(number.get() - 1);
     button_increase.set_label(&number.get().to_string());
    }));

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.present();
}
