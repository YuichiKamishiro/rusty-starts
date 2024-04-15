use sysinfo::System;
use chrono::Local;
use gtk::{gdk, glib, CssProvider, prelude::*, ApplicationWindow, Label, Notebook, Box, FlowBox, ProgressBar};

use adw::Application;

fn get_info() ->  String{
    let mut sys = System::new_all();
    sys.refresh_all();

    format!("Used {}/{}",
        sys.total_memory() / (1024 * 1024),
        sys.used_memory() / (1024 * 1024))
}

fn draw_gui(app: &Application){
    let mbox = Box::new(gtk::Orientation::Vertical, 0);
    let mem_label = Label::new(Some(get_info().as_str()));
    
    let bar = ProgressBar::new();
    bar.set_fraction(0.0);
    
    let nlabel = Label::new(Some("Coming soon...info about mem"));
    nlabel.set_halign(gtk::Align::Start);
    nlabel.set_margin_start(20);
    nlabel.set_margin_top(50);

    mbox.append(&bar);
    mbox.append(&mem_label);
    mbox.append(&nlabel);
    
    let tabs = Notebook::new();
    tabs.append_page(&mbox, Some(&Label::new(Some("MEM INFO"))));

    let tick = move || {
        let mut sys = System::new_all();
        sys.refresh_all();

        let used = (sys.used_memory() / (1024 * 1024)) as f64;
        let total = (sys.total_memory() / (1024 * 1024)) as f64;

        let fr: f64 = used / total;
        bar.set_fraction(fr);
        glib::ControlFlow::Continue
    };

    glib::timeout_add_seconds_local(0, tick);

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
