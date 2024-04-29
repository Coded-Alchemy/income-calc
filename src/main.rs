mod income_calc;
mod income_percentage;
mod use_case;

use slint::SharedString;
use income_calc::use_case;
use income_percentage::percent::IncomePercentage;

slint::include_modules!();

// Main function, application entry point.
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |user_input: SharedString| {
            let ui = ui_handle.unwrap();

            if (!user_input.is_empty()) {
                let user_input: f64 = user_input.trim().parse().unwrap();

                let income_percentage = IncomePercentage {
                    tax: use_case::calculate_tax_percentage(user_input),
                    owner: use_case::calculate_owner_percentage(user_input),
                    profit: use_case::calculate_profit_percentage(user_input),
                    operation_expense: use_case::calculate_operation_expense(user_input),
                };

                let formated_calculation_result_string =
                    format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expense: {:.2}\n",
                            income_percentage.tax, income_percentage.owner, income_percentage.profit,
                            income_percentage.operation_expense);

                ui.set_results(formated_calculation_result_string.into());
            }
        }
    });

    ui.run()
}



