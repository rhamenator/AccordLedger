use accord_ledger::{SettlementInput, settle};

fn main() {
    let result = settle(SettlementInput {
        gross_sales_cents: 1_000_000,
        returns_cents: 75_000,
        chargebacks_cents: 25_000,
        rate_basis_points: 800,
        minimum_due_cents: 0,
        advance_balance_cents: 10_000,
    })
    .unwrap();
    println!(
        "net sales ${:.2}; payable ${:.2}",
        result.net_sales_cents as f64 / 100.0,
        result.payable_cents as f64 / 100.0
    );
}
