use sysinfo::{Components, System};
use gtk::{gio::ApplicationFlags, prelude::*, Application, ApplicationWindow, Label, Notebook};

fn get_info() ->  String{
    let mut sys = System::new_all();
    sys.refresh_all();

    format!("Total mem {} mb", sys.total_memory() / (1024 * 1024)) 
}

fn draw_gui(app: &Application) {
    let mem_label = Label::new(Some(get_info().as_str()));

    let tabs = Notebook::new();
    tabs.append_page(&mem_label, Some(&Label::new(Some("MEM INFO"))));
    
    let window = ApplicationWindow::builder()
        .application(app)
        .default_height(500)
        .default_width(500)
        .child(&tabs)
        .build();

    window.present();
}

const ID: &str = "ID";

fn main() {
    let app = Application::builder()
        .application_id(ID)
        .build();
    app.connect_activate(draw_gui);

    app.run();
}
