use gtk::{
    gdk, glib, prelude::*, ApplicationWindow, ScrolledWindow, Box, CssProvider, Label, Notebook, ProgressBar,
};
use gtk::Scrollbar;
use sysinfo::Disks;
use sysinfo::System;

use adw::Application;

fn get_memory_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "Used mem {1}/{0}",
        sys.total_memory() / (1024 * 1024),
        sys.used_memory() / (1024 * 1024)
    )
}

fn get_swap_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "Used swap {1}/{0}",
        sys.total_swap() / (1024 * 1024),
        sys.used_swap() / (1024 * 1024)
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

    let mut string_info = String::new();

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        string_info.push_str(&format!(
            "Disk: {}\n\tSpace used {} / {}\n\tMount point {}\n",
            disk.name().to_str().unwrap(),
            disk.available_space() / (1024 * 1024),
            disk.total_space() / (1024 * 1024),
            disk.mount_point().to_str().unwrap()
        ));
    }

    string_info
}

fn get_memory_page(mem_bar: &ProgressBar, swap_bar: &ProgressBar) -> Box {
    let mbox = Box::new(gtk::Orientation::Vertical, 0);
    let mem_label = Label::new(Some(&get_memory_info()));
    mem_label.add_css_class("text");

    let swap_label = Label::new(Some(&get_swap_info()));
    swap_label.add_css_class("text");

    mbox.append(mem_bar);
    mbox.append(&mem_label);
    mbox.append(swap_bar);
    mbox.append(&swap_label);
    mbox
}

fn draw_gui(app: &Application) {
    let mem_bar = ProgressBar::new();
    let swap_bar = ProgressBar::new();

    let os_label = Label::new(Some(&get_os_info()));
    let disk_label = Label::new(Some(&get_disk_info()));
    os_label.add_css_class("text");
    disk_label.add_css_class("text");

    let scrolled_window = ScrolledWindow::builder()
    .min_content_width(10)
    .hscrollbar_policy(gtk::PolicyType::Never)
    .child(&disk_label)
    .build();


    let tabs = Notebook::new();
    tabs.append_page(&get_memory_page(&mem_bar, &swap_bar), Some(&Label::new(Some("MEM INFO"))));
    tabs.append_page(&os_label, Some(&Label::new(Some("OS INFO"))));
    tabs.append_page(&scrolled_window, Some(&Label::new(Some("!DISK INFO"))));

    let tick = move || {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut used = (sys.used_memory() / (1024 * 1024)) as f64;
        let mut total = (sys.total_memory() / (1024 * 1024)) as f64;

        let mut fr: f64 = used / total;
        mem_bar.set_fraction(fr);

        used = (sys.used_swap() / (1024 * 1024)) as f64;
        total = (sys.total_swap() / (1024 * 1024)) as f64;

        fr = used / total;
        swap_bar.set_fraction(fr);
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
