def validate_paypal_customer_id(paypal_customer_id):
    if paypal_customer_id.len() < 5:
        return False
    return True

def validate_stripe_customer_id(stripe_customer_id):
    if stripe_customer_id.len() < 5:
        return False
    return True

