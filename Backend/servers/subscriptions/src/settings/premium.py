from coinbase_commerce.client import Client
import datetime
from subscription.encryption import decrypt, encrypt
from rest_framework import serializers, status
from settings.totp import generate_totp
from subscription.encryption import decrypt
from subscription.models import UserProfile
from subscription.paypal import show_sub_details, suspend_sub, activate_sub, cancel_sub
import stripe

class PremiumSerializer(serializers.Serializer):
    def does_subscriber_exist(self, user):
        try:
            subscriber = UserProfile.objects.get(user=user)
        except:
            subscriber = UserProfile.objects.create(user=user, method_id=self.payment_method('None'))
            subscriber.save()
        return subscriber
    
    def obtain_method(self, subscriber):
        if subscriber:
            method = str(subscriber.method)
        else:
            method = None
        return method
    
    def is_user_subscribed(self, user, subscriber):
        if subscriber:
            subscribed = subscriber.subscribed
        else:
            subscriber = UserProfile.objects.create(user=user, method_id=self.payment_method('None'))
            subscriber.save()
        return subscribed

    def build_stripe_checkout(self, subscriber, customer, success_url, cancel_url):
        prices = stripe.Price.list(
                lookup_keys=['Conjugat Premium'],
                expand=['data.product']
        )

        line_items=[
                    {
                        'price': prices.data[0].id,
                        'quantity': 1,
                    },
        ]

        checkout_kwargs = {
            'line_items' : line_items,
            'customer':customer,
            'mode':'subscription',
            'success_url':success_url,
            'cancel_url':cancel_url,
        }

        if not subscriber or subscriber.trial == True:
            checkout_kwargs['subscription_data'] = {'trial_period_days':7}

        checkout_session = stripe.checkout.Session.create(**checkout_kwargs)
        return checkout_session

    def build_coinbase_checkout(self, subscriber, success_url, cancel_url):
        client = Client(api_key=settings.COINBASE_COMMERCE_API_KEY)

        checkout_kwargs = {
            'name':'Conjugat Premium',
            'local_price': {
                'currency':'GBP'
            },
            'pricing_type':'fixed_price',
            'rediret_url':success_url,
            'cancel_url':cancel_url,
        }

        if not subscriber or subscriber.trial == True:
            checkout_kwargs['description'] = '1 Week of conjugat Premium'
            checkout_kwargs['local_price']['amount'] = '0.01'

        else:
            checkout_kwargs['description'] = '1 Month of conjugat Premium'
            checkout_kwargs['local_price']['amount'] = '3.00'

        charge = client.charge.create(**checkout_kwargs)
        return charge

    def payment_method(self, method):
        if method == 'Stripe':
            return 1
        elif method == 'Paypal':
            return 2
        elif method == 'Coinbase':
            return 3
        elif method == 'None':
            return 4

    def save_subscriber(self, user, subscriber, customer_id):
        if not subscriber:
            subscriber = UserProfile.objects.create(user=user, method_id=self.payment_method('Stripe'))
        subscriber.method_id=self.payment_method('Stripe')
        # Reset the subscription and customer ids
        subscriber.subscription_id = None
        subscriber.customer_id = encrypt(customer_id)
        subscriber.save()
    
    def build_stripe_portal(self, stripe, subscriber, return_url):
        customer = decrypt(subscriber.customer_id)
        portalSession = stripe.billing_portal.Session.create(
                        customer=customer,
                        return_url=return_url,
        )
        return portalSession

    def return_premium_status(self, data):
        user = self.context['user']
        stripe.api_key = settings.STRIPE_SECRET_KEY
        subscriber = self.does_subscriber_exist(user)
        method = self.obtain_method(subscriber)
        subscribed = self.is_user_subscribed(user, subscriber)
        if subscribed == False:
            if data['method'] == None:
                success_url = data["success_url"]
                cancel_url = data["cancel_url"]
                customer = stripe.Customer.create()
                stripe_url = self.build_stripe_checkout(subscriber, customer, success_url, cancel_url).url

                charge = self.build_coinbase_checkout(subscriber, success_url, cancel_url)
                coinbase_url = charge.hosted_url

                response = {
                    'subscribed':subscribed,
                    'trial':subscriber.trial,
                    'stripe_customer_id':customer.id,
                    'stripe_url':stripe_url,
                    'coinbase_url':coinbase_url
                }
                return response, True, status.HTTP_200_OK
            
            if data['method'] == 'Stripe':
                user = self.context['user']
                subscriber = self.does_subscriber_exist(user)
                subscribed = self.is_user_subscribed(user, subscriber)
                if subscribed == False:
                    customer_id = data['customer_id']
                    try:
                        self.save_subscriber(user, subscriber, customer_id)
                    except:
                        error = 'Stripe customer id was not found'
                        return error, False, status.HTTP_404_NOT_FOUND
                    response = "User created successfully"
                    return response, True, status.HTTP_200_OK

            if data['method'] == 'Paypal':
                user = self.context['user']
                subscriber = self.does_subscriber_exist(user)
                subscribed = self.is_user_subscribed(user, subscriber)
                if subscribed == False:
                    subscriber_id = data.get('subscriber_id')
                    try:
                        self.save_subscriber('Paypal', user, subscriber, subscriber_id)
                    except:
                        error = 'Paypal customer id was not found'
                        return error, False, status.HTTP_404_NOT_FOUND
                    response = "User created successfully"
                    return response, True, status.HTTP_200_OK
            
            if data['method'] == 'Coinbase':
                user = self.context['user']
                subscriber = self.does_subscriber_exist(user)
                subscribed = self.is_user_subscribed(user, subscriber)
                if subscribed == False:
                    charge_url = data['charge_url']
                    subscriber_id = charge_url.rsplit('/', 1)[1]
                    try:
                        self.save_subscriber('Coinbase', user, subscriber, subscriber_id)
                    except:
                        error = 'Coinbase id was not found'
                        return error, False, status.HTTP_404_NOT_FOUND
                    response = "User created successfully"
                    return response, True, status.HTTP_200_OK

        else:
            user = self.context['user']
            stripe.api_key = settings.STRIPE_SECRET_KEY
            subscriber = self.does_subscriber_exist(user)
            method = self.obtain_method(subscriber)
            subscribed = self.is_user_subscribed(user, subscriber)
            if subscribed == True:
                subscriber.url = None
                subscriber.status = None
                
                if method == 'Stripe':
                    return_url = data['return_url']
                    stripe_portal = self.build_stripe_portal(stripe, subscriber, return_url)
                    response = {
                        'method': method,
                        'subscribed': subscribed,
                        'url': stripe_portal.url
                    }
                    return response, True, status.HTTP_200_OK

                if method == 'Coinbase':
                    return_url = data['return_url']
                    client = Client(api_key=settings.COINBASE_COMMERCE_API_KEY)
                    charge_id = decrypt(subscriber.subscription_id)
                    charge = client.charge.retrieve(charge_id)
                    response = {
                        'method': method,
                        'subscribed': subscribed,
                        'url': charge.hosted_url
                    }
                    return response, True, status.HTTP_200_OK
                
                if method == 'Paypal':
                    action = data['action']
                    subscription_id = decrypt(subscriber.subscription_id)
                    if action == None:
                        details = show_sub_details(subscription_id)
                        subscriber.status = details['status']
                        response = {
                            'method': method,
                            'subscribed': subscribed,
                            'status': subscriber.status
                        }
                        return response, True, status.HTTP_200_OK
                    elif action == 'Stop':
                        suspend_sub(subscription_id)
                        details = show_sub_details(subscription_id)
                        subscriber.status = details['status']
                        response = {
                            'method': method,
                            'subscribed': subscribed,
                            'status': subscriber.status
                        }
                        return response, True, status.HTTP_200_OK
                    elif action == 'Re-start':
                        activate_sub(subscription_id)
                        details = show_sub_details(subscription_id)
                        subscriber.status = details['status']
                        response = {
                            'method': method,
                            'subscribed': subscribed,
                            'status': subscriber.status
                        }
                        return response, True, status.HTTP_200_OK