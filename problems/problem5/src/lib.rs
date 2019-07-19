use std::cmp::Ordering;

// Write a mortgage calculator based on the following algorithm:
// * User borrows big money sum P from bank.
// * Bank sets interest rate R at percent - the speed of growth of the debt.
// * At the end of each month the debt is increased by R / 12 percent.
// * User sends to bank some predefined small sum M to decrease the debt.
// * Debt is considered settled when its value is reduced to zero.

pub struct Mortgage {
    original_principal: i64,
    down_payment: i64,
    interest_rate: f64,
    loan_term_months: i32,
}


// 1. Write a test for this formula 
fn compound_formula(principal: i64, interest_rate: f64) -> i64 {
    let principal = principal as f64;
    let interest = principal * interest_rate / 12.00;
    println!("interest: {}", interest);
    let new_principal = principal + interest ;
    new_principal.round() as i64
}


    
impl Mortgage {
    // 2. Write the constructor
    pub fn new(
        original_principal: i64, 
        interest_rate: f64, 
        loan_term_months: i32
    ) -> Mortgage {
        Mortgage {
            original_principal,
            interest_rate,
            loan_term_months,
        }
    }

    // 3. Calculate remaining principal after n months
    fn principal_remaining(&self, months: i32, payment: i64) -> i64 {
        let mut remaining_principal = self.original_principal;
        for month in (0..months) { 
            remaining_principal = compound_formula(remaining_principal, self.interest_rate);
            remaining_principal = remaining_principal - payment;
            remaining_principal = match remaining_principal.cmp(&0) { 
                Ordering::Greater => remaining_principal,
                _ => 0,
            };
            println!("Month {}: ${}", month + 1, remaining_principal);
        } 
        remaining_principal
    }
    
    // 4. Calculate minimum monthly payment required to pay the loan off on time.
    fn minimum_monthly_payment(&self) -> i64 {
        3000
    }
    
    // 5. Calculate the total amount paid after n months
    fn total_payout(&self, months: i32, monthly_payment: i64) -> i64 {
        1_000_000 
    } 
    
    // 6. Calculate time needed to pay off the loan given a monthly payment of x dollars
    fn months_until_payoff(&self, monthly_payment: i64) -> i32 {
        let mut months: i32 = 0;
        let mut new_principal = self.original_principal;
        while new_principal >= 0 {
            new_principal = compound_formula(new_principal, self.interest_rate);
            new_principal = new_principal - monthly_payment;
            months = months + 1;
        } 
        months 
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
        
        assert_eq!(mortgage.months_until_payoff(monthly_payment), 103);
        assert_eq!(mortgage.principal_remaining(2, monthly_payment), 787970);
        assert_eq!(mortgage.total_payout(103, monthly_payment), 1024209);
       
    }
}
