use slint::SharedString;
slint::include_modules!();

const TAX_PERCENTAGE: f64 = 0.30;
const OWNER_PERCENTAGE: f64 = 0.55;
const PROFIT_PERCENTAGE: f64 = 0.05;
const OPERATION_EXPENSE_PERCENTAGE: f64 = 0.10;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |string: SharedString| {
            let ui = ui_handle.unwrap();
            let user_input: f64 = string.trim().parse().unwrap();
            let tax: f64 = user_input * TAX_PERCENTAGE;
            let owner: f64 = user_input * OWNER_PERCENTAGE;
            let profit: f64 = user_input * PROFIT_PERCENTAGE;
            let operating_expense: f64 = user_input * OPERATION_EXPENSE_PERCENTAGE;
            let result = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expense:\
                {:.2}\n", tax, owner, profit, operating_expense);

            ui.set_results(result.into());
        }
    });

    ui.run()
}
