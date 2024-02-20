use slint::SharedString;
slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPAY: f64 = 0.55;
const PROFIT: f64 = 0.05;
const OPERATIONS: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string: SharedString| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNERPAY;
        let profit: f64 = num * PROFIT;
        let opex: f64 = num * OPERATIONS;
        let result: String = format!(
            "Taxes: {:.2}\nSalary: {:.2}\nProfit: {:.2}\nOperational Expense: {:.2}",
            tax, owner, profit, opex
        );
        ui.set_results(result.into());
    });

    ui.run()
}
