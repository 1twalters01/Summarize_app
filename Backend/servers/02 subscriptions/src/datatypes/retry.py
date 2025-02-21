from src.datatypes.payment_providers import PaymentProviderEnum

class Retry():
    payment_provider: PaymentProviderEnum
    customer_id_token: str|None