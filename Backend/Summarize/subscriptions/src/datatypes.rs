std::time::{Duration, instant};

pub struct Subscriber {
    pub method: Method,
    pub subscribed: bool,
    pub has_trial: bool,
    pub trial_duration: Duration,
    pub total_duration_subscribed: Option<Duration>,
    pub lifetime_amount_spent: LifetimeAmountSpent
}

pub enum Method {
    None,
    Paypal(PaypalInstance),
    Stripe(StripeInstance),
    Coinbase(CoinbaseInstance)
}

pub struct PaypalInstance {
    pub customer_id: String, // Create an actual thing?
    pub start_date: Option<instant>,
    pub end_date: Option<instant>,
    // url?
}

pub struct StripeInstance {
    pub customer_id: String,
    pub subscription_id: String,
    pub start_date: Option<instant>,
    pub end_date: Option<instant>,
}

pub struct CoinbaseInstance {
    pub customer_id: String,
    pub start_date: Option<instant>,
    pub end_date: Option<instant>,
}

pub struct LifetimeAmountSpent {
    pub paypal: f16,
    pub stripe: f16,
    pub coinbase: f16
}
