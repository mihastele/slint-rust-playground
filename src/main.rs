slint::include_modules!();

const TAX_PERCENT: f64 = 0.30;
const OWNER_PERCENT: f64 = 0.55;
const PROFIT_PERCENT: f64 = 0.05;
const OPERATING_EXPENSES: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income(move |string| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax = num * TAX_PERCENT;
        let owner = num * OWNER_PERCENT;
        let profit = num * PROFIT_PERCENT;
        let operating_expenses = num * OPERATING_EXPENSES;
        let result = format!("Tax: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expenses: {:.2}", tax, owner, profit, operating_expenses);
        ui.set_results(result.into());
 });

    ui.run()
}
