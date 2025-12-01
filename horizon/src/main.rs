use crate::{
    rendering::application::*,
};

mod rendering {
    pub mod application;
}

fn main() {
    let app: App = App::create(
        WindowProps::new(String::from("Horizon"), [0.45, 0.3, 0.65, 1.0])
    );

    let my_val: bool = true;

    app.main_loop();
}
