use sysinfo::System;

use gtk::{gdk, glib, CssProvider, prelude::*, ApplicationWindow, Label, Notebook};

use adw::Application;

fn get_info() ->  String{
    let mut sys = System::new_all();
    sys.refresh_all();

    format!("Total mem {} MB\nFree mem {} MB\nUsed mem {} MB",
        sys.total_memory() / (1024 * 1024),
        sys.free_memory() / (1024 * 1024),
        sys.used_memory() / (1024 * 1024))
}

fn draw_gui(app: &Application){
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

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() -> glib::ExitCode{
    let app = Application::builder()
        .application_id(ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(draw_gui);
    
    app.run()
}
