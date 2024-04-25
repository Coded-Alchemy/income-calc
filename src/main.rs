use slint::SharedString;

slint::include_modules!();

// Struct for percentage division
struct IncomePercentage {
    tax: f64,
    owner: f64,
    profit: f64,
    operation_expense: f64,
}

// Main function, application entry point.
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |user_input: SharedString| {
            let ui = ui_handle.unwrap();
            let user_input: f64 = user_input.trim().parse().unwrap();

            let income_percentage = IncomePercentage {
                tax: calculate_tax_percentage(user_input),
                owner: calculate_owner_percentage(user_input),
                profit: calculate_profit_percentage(user_input),
                operation_expense: calculate_operation_expense(user_input),
            };

            let formated_calculation_result_string =
                format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expense: {:.2}\n",
                        income_percentage.tax, income_percentage.owner, income_percentage.profit,
                        income_percentage.operation_expense);

            ui.set_results(formated_calculation_result_string.into());
        }
    });

    ui.run()
}

fn calculate_tax_percentage(input: f64) -> f64 {
    const TAX_PERCENTAGE: f64 = 0.30;
    input * TAX_PERCENTAGE
}

fn calculate_owner_percentage(input: f64) -> f64 {
    const OWNER_PERCENTAGE: f64 = 0.55;
    input * OWNER_PERCENTAGE
}

fn calculate_profit_percentage(input: f64) -> f64 {
    const PROFIT_PERCENTAGE: f64 = 0.05;
    input * PROFIT_PERCENTAGE
}

fn calculate_operation_expense(input: f64) -> f64 {
    const OPERATION_EXPENSE_PERCENTAGE: f64 = 0.10;
    input * OPERATION_EXPENSE_PERCENTAGE
}


