use gtk::{
    gdk, glib, prelude::*, ApplicationWindow, Box, CssProvider, Label, Notebook, ProgressBar,
};
use sysinfo::System;
use sysinfo::Disks;

use adw::Application;

fn get_memory_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "Used {1}/{0}",
        sys.total_memory() / (1024 * 1024),
        sys.used_memory() / (1024 * 1024)
    )
}

fn get_os_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "System name {}\nKernel ver {}\nOs ver {}\nHostname {}",
        System::name().unwrap(),
        System::kernel_version().unwrap(),
        System::os_version().unwrap(),
        System::host_name().unwrap()
    )
}

fn get_disk_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut stringInfo = String::new();

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks{
        stringInfo.push_str(&format!("{disk:?}\n"));
    }


    stringInfo
}

fn draw_gui(app: &Application) {
    let mbox = Box::new(gtk::Orientation::Vertical, 0);
    let mem_label = Label::new(Some(get_memory_info().as_str()));

    let bar = ProgressBar::new();
    bar.set_fraction(0.0);

    let nlabel = Label::new(Some(""));
    nlabel.set_halign(gtk::Align::Start);
    nlabel.set_margin_start(20);
    nlabel.set_margin_top(50);

    mbox.append(&bar);
    mbox.append(&mem_label);
    mbox.append(&nlabel);

    let os_label = Label::new(Some(&get_os_info()));
    let disk_label = Label::new(Some(&get_disk_info()));

    let tabs = Notebook::new();
    tabs.append_page(&mbox, Some(&Label::new(Some("MEM INFO"))));
    tabs.append_page(&os_label, Some(&Label::new(Some("OS INFO"))));
    tabs.append_page(&disk_label, Some(&Label::new(Some("DISK INFO"))));

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

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(draw_gui);

    app.run()
}
