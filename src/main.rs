mod domain;
mod income_percentage;
mod ui_layer;

use ui_layer::view_model;

slint::include_modules!();

// Main function, application entry point.
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    view_model::divide_income(ui.clone_strong());

    ui.run()
}



