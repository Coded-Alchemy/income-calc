mod domain_layer;
mod model_layer;
mod ui_layer;

use ui_layer::view_model;

slint::include_modules!();

// Main function, application entry point.
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    view_model::handle_user_input(ui.clone_strong());

    ui.run()
}



