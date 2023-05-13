#![allow(unused)]

mod menu;
mod model;
mod view;

fn main() {
    let mut app = model::CoronaApplication::load();
    menu::main(&mut app);
    app.save();
}
