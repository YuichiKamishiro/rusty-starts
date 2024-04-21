use crate::page_info;
use gtk::{
    gdk, glib, prelude::*, ApplicationWindow, CssProvider, Label, Notebook, ProgressBar,
    ScrolledWindow,
};
use sysinfo::System;

use adw::Application;

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id("ID").build();

    app.connect_startup(|_| load_css());
    app.connect_activate(draw_gui);

    app.run()
}

pub fn draw_gui(app: &Application) {
    let mem_bar = ProgressBar::new();
    let swap_bar = ProgressBar::new();
    let os_label = Label::new(Some(&page_info::os_info()));
    let disk_label = Label::new(Some(&page_info::disk_info()));

    os_label.add_css_class("text");
    disk_label.add_css_class("text");

    let listbox: gtk::ListBox = gtk::ListBox::new();
    listbox.append(&disk_label);

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(10)
        .hscrollbar_policy(gtk::PolicyType::Always)
        .child(&listbox)
        .build();

    let tabs = Notebook::new();
    tabs.append_page(
        &page_info::memory_page(&mem_bar, &swap_bar),
        Some(&Label::new(Some("MEM INFO"))),
    );
    tabs.append_page(&os_label, Some(&Label::new(Some("OS INFO"))));
    tabs.append_page(&scrolled_window, Some(&Label::new(Some("DISK INFO"))));

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

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
