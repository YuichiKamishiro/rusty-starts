mod my_application;
mod page_info;

use my_application::my_application::run;

use gtk::glib;
fn main() -> glib::ExitCode{
    run()
}
