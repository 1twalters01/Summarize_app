# Change to real sql
def get_discount_from_code(discount_code):
    discount = db.query(DiscountCode).filter(DiscountCode.code == code).first()

# Change to join
def validate_code_and_payment_type(discount_code, payment_type):
    discount_code_id = db.query(DiscountCode).filter(DiscountCode.code == code).first().id
    db.query(DiscountPaymentType).filter(
        DiscountPaymentType.discount_code_id == discount_code_id
        &
        DiscountPaymentType.payment_type == payment_type
    )