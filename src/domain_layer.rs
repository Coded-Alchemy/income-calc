// Module to hold functions related to performing calculations.
pub(crate) mod use_case {
    use crate::model_layer::model::CalculationResults;

    pub(crate) fn calculate_tax_percentage(input: f64) -> f64 {
        const TAX_PERCENTAGE: f64 = 0.30;
        input * TAX_PERCENTAGE
    }

    pub(crate) fn calculate_owner_percentage(input: f64) -> f64 {
        const OWNER_PERCENTAGE: f64 = 0.55;
        input * OWNER_PERCENTAGE
    }

    pub(crate) fn calculate_profit_percentage(input: f64) -> f64 {
        const PROFIT_PERCENTAGE: f64 = 0.05;
        input * PROFIT_PERCENTAGE
    }

    pub(crate) fn calculate_operation_expense(input: f64) -> f64 {
        const OPERATION_EXPENSE_PERCENTAGE: f64 = 0.10;
        input * OPERATION_EXPENSE_PERCENTAGE
    }

    pub(crate) fn format_results(income_percentage: CalculationResults) -> String {
        format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expense: {:.2}\n",
                income_percentage.tax, income_percentage.owner, income_percentage.profit,
                income_percentage.operation_expense)
    }
}