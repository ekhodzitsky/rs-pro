trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

struct CreditCardPayment;

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) {
        println!("Payment amount {} via credit card.", amount);
    }
}

struct PayPalPayment;

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) {
        println!("Payment amount {} via PayPal.", amount);
    }
}

struct ShoppingCart<T: PaymentStrategy> {
    payment_strategy: T,
}

impl<T: PaymentStrategy> ShoppingCart<T> {
    fn new(payment_strategy: T) -> Self {
        ShoppingCart { payment_strategy }
    }

    fn checkout(&self, amount: f64) {
        self.payment_strategy.pay(amount);
    }
}

pub fn demo() {
    println!("Strategy");

    let with_credit_card = ShoppingCart::new(CreditCardPayment);
    with_credit_card.checkout(100.0);

    let with_paypal = ShoppingCart::new(PayPalPayment);
    with_paypal.checkout(150.0);

    println!();
}
