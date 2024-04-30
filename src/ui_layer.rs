pub mod view_model {
    use slint::{ComponentHandle, SharedString};
    use crate::AppWindow;
    use crate::domain::use_case;
    use crate::income_percentage::percent::IncomePercentage;

    pub(crate) fn divide_income(app_window: AppWindow) {
        app_window.on_divide_income({

            let ui_handle = app_window.as_weak();

            move |user_input: SharedString| {

                let ui = ui_handle.unwrap();

                if !user_input.is_empty() {

                    let user_input: f64 = user_input.trim().parse().unwrap();

                    let income_percentage = IncomePercentage {
                        tax: use_case::calculate_tax_percentage(user_input),
                        owner: use_case::calculate_owner_percentage(user_input),
                        profit: use_case::calculate_profit_percentage(user_input),
                        operation_expense: use_case::calculate_operation_expense(user_input),
                    };

                    let result_string = use_case::format_results(income_percentage.tax,
                                                                 income_percentage.owner,
                                                                 income_percentage.profit,
                                                                 income_percentage.operation_expense);

                    ui.set_results(result_string.into());
                }
            }
        });
    }
}