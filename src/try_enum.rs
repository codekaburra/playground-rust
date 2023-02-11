enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}
fn pay_by_credit(amount: u64) {
    println!("Processing credit payment for {}", amount);
}
fn pay_by_debit(amount: u64) {
    println!("Processing debit payment of {}", amount);
}
fn paypal_redirect(amount: u64) {
    println!("Redirecting to paypal for amount: {}", amount);
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount),
        }
    }
}
fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}

fn main() {
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(567);
}
