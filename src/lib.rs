#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettlementInput {
    pub gross_sales_cents: i64,
    pub returns_cents: i64,
    pub chargebacks_cents: i64,
    pub rate_basis_points: u32,
    pub minimum_due_cents: i64,
    pub advance_balance_cents: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settlement {
    pub net_sales_cents: i64,
    pub earned_cents: i64,
    pub due_before_advance_cents: i64,
    pub advance_applied_cents: i64,
    pub payable_cents: i64,
}

pub fn settle(input: SettlementInput) -> Result<Settlement, &'static str> {
    if input.gross_sales_cents < 0
        || input.returns_cents < 0
        || input.chargebacks_cents < 0
        || input.minimum_due_cents < 0
        || input.advance_balance_cents < 0
        || input.rate_basis_points > 100_000
    {
        return Err("amounts must be non-negative and rate must not exceed 1000%");
    }
    let net_sales_cents =
        (input.gross_sales_cents - input.returns_cents - input.chargebacks_cents).max(0);
    let earned_cents = rounded_basis_points(net_sales_cents, input.rate_basis_points);
    let due_before_advance_cents = earned_cents.max(input.minimum_due_cents);
    let advance_applied_cents = due_before_advance_cents.min(input.advance_balance_cents);
    Ok(Settlement {
        net_sales_cents,
        earned_cents,
        due_before_advance_cents,
        advance_applied_cents,
        payable_cents: due_before_advance_cents - advance_applied_cents,
    })
}

fn rounded_basis_points(cents: i64, basis_points: u32) -> i64 {
    (i128::from(cents) * i128::from(basis_points) + 5_000).div_euclid(10_000) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settlement_nets_returns_chargebacks_minimum_and_advance() {
        let result = settle(SettlementInput {
            gross_sales_cents: 1_000_000,
            returns_cents: 100_000,
            chargebacks_cents: 50_000,
            rate_basis_points: 750,
            minimum_due_cents: 70_000,
            advance_balance_cents: 20_000,
        })
        .unwrap();
        assert_eq!(result.net_sales_cents, 850_000);
        assert_eq!(result.earned_cents, 63_750);
        assert_eq!(result.payable_cents, 50_000);
    }
}
