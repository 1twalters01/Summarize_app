from src.query.discount_codes import get

def get_discount_from_code(discount_code):
    discount = get.from_discount_code(discount_code)

# Change to join
def validate_code_and_payment_type(discount_code, payment_type):
    discount_code_id = db.query(DiscountCode).filter(DiscountCode.code == code).first().id
    db.query(DiscountPaymentType).filter(
        DiscountPaymentType.discount_code_id == discount_code_id
        &
        DiscountPaymentType.payment_type == payment_type
    )
