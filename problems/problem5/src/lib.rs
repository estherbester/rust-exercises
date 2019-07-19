use std::cmp::Ordering;

// Write a mortgage payoff calculator based on the following algorithm:
// * User borrows big money sum P from bank.
// * Bank sets interest rate R at percent - the speed of growth of the debt.
// * At the end of each month the debt is increased by R / 12 percent.
// * User sends to bank some predefined small sum M to decrease the debt.
// * Debt is considered settled when its value is reduced to zero.

// 1. Write a test for this formula 
fn compound_formula(principal: i64, interest_rate: f64) -> i64 {
    let principal = principal as f64;
    let interest = principal * interest_rate / 12.00;
    println!("interest: {}", interest);
    let new_principal = principal + interest ;
    new_principal.round() as i64
}


#[derive(Default)]
pub struct Mortgage {
    original_principal: i64,
    down_payment: i64,
    interest_rate: f64,
    loan_term_months: i32,
}

impl Mortgage {
    // https://rust-lang.github.io/rust-clippy/master/index.html#new_without_default
    pub fn new() -> Self {
        Mortgage
    }

    // 2. Calculate remaining principal after n months
    fn principal_remaining(&self, months: i32, payment: i64) -> i64 {
        100
    }
    
    // 3. Calculate minimum monthly payment required to pay the loan off on time.
    fn minimum_monthly_payment(&self) -> i64 {
        3000
    }
    
    // 4. Calculate the total amount paid after n months
    fn total_payout(&self, months: i32, monthly_payment: i64) -> i64 {
        1_000_000 
    } 
    
    // 5. Calculate time needed to pay off the loan given a monthly payment of x dollars
    fn months_until_payoff(&self, monthly_payment: i64) -> i32 {
        12
    } 
}

// BONUS PROBLEM:
// What if the mortgage were based on a variable interest rate?

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mortgage() {
        let mortgage = Mortgage {
            original_principal:800_000, 
            down_payment: 0,
            interest_rate: 0.06, 
            loan_term_months: 12 * 30,
        };
        
        let monthly_payment = 10000;
        
        assert_eq!(mortgage.principal_remaining(2, monthly_payment), 787970);
        assert_eq!(mortgage.minimum_monthly_payment(), 4,796);
        assert_eq!(mortgage.total_payout(103, monthly_payment), 1024209);
        assert_eq!(mortgage.months_until_payoff(monthly_payment), 103);
       
    }
}
