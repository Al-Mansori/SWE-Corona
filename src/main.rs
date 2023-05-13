#![allow(unused)]

/// All functions related to user input / user output
mod menu;

/// The business login of the application
mod model;

/// How to pretty print classes to the user. Used in `menu`
mod view;

/// Entry point of the application
fn main() {
    let mut app = model::CoronaApplication::load();
    menu::main(&mut app);
    app.save();
}
