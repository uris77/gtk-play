use gtk::{prelude::*, Application, ApplicationWindow, Button};

mod listings;

const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(listings::memory_mgmt::build_ui);

    app.run();
}

#[allow(dead_code)]
fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}
