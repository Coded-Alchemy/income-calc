pub mod view_model {
    use slint::{ComponentHandle, SharedString};
    use crate::AppWindow;
    use crate::domain_layer::use_case;
    use crate::model_layer::model::CalculationResults;

    pub(crate) fn handle_user_input(app_window: AppWindow) {

        app_window.on_divide_income({

            let ui_handle = app_window.as_weak();

            // Get input from user.
            move |user_input: SharedString| {

                let ui = ui_handle.unwrap();

                // Check user input in not empty string and only contains numbers
                if !user_input.is_empty() && user_input.chars().all(char::is_numeric) {

                    let user_input: f64 = user_input.trim().parse().unwrap();

                    let calculations = CalculationResults {
                        tax: use_case::calculate_tax_percentage(user_input),
                        owner: use_case::calculate_owner_percentage(user_input),
                        profit: use_case::calculate_profit_percentage(user_input),
                        operation_expense: use_case::calculate_operation_expense(user_input),
                    };

                    let result_string = use_case::format_results(calculations);

                    ui.set_results(result_string.into());
                }
            }
        });
    }
}