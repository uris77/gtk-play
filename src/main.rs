use gtk::{prelude::*, Application, ApplicationWindow, Button};
use listings::custom_button::CustomButton;

mod listings;

const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    //app.connect_activate(listings::memory_mgmt::build_ui);
    app.connect_activate(build_custom_btn);

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

#[allow(dead_code)]
pub fn build_custom_btn(app: &Application) {
    let button = CustomButton::with_label("Press me!");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Tutorial")
        .child(&button)
        .build();

    window.present();
}
