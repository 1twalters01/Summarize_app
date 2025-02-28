from src.queries import subscribers, subscription_history, payment_history, users
from src.services.encryption_service import EncryptionService

def create_subscription():
    pass

def pause_subscription(last_subscription: Subscription):
    pass

def resume_subscription(last_subscription: Subscription):
    pass

def cancel_subscription(last_subscription: Subscription):
    pass

def get_payment_provider_for_invoice_id_and_uuid(invoice_id, user_uuid):
    encryption_service = EncryptionService()
    encrypted_id = encryption_service.encrypt(invoice_id)
    try:
        history = payment_history.get.from_encrypted_payment_id(encrypted_id)
        if user_uuid != users.get.uuid_from_user_id(history.user_id):
            raise ValueError
        return history.payment_method_enum
    except:
        try:
            history = subscription_history.get.from_encrypted_subscription_id(encrypted_id)
            subscriber = subscriber.get.subscription_method_from_subscriber_id(history.subscriber_id)
            if user_uuid != users.get.uuid_from_user_id(subscriber.user_id):
                raise ValueError
            return subscriber.subscription_method_enum
        except:
            raise LookupError

def subscribers(user_uuid):
    return subscribers.get.subscribers_from_user_uuid(user_uuid)