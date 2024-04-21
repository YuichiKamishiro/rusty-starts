use my_application::run;

mod my_application;
mod page_info;

use gtk::glib;
fn main() -> glib::ExitCode {
    run()
}
