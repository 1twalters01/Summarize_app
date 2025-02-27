Todo
#####
#. Choose either _enum or not in field names in database schema
#. Finish refactoring the views
#. Write tests for:
    #. datatypes
    #. middleware
        #. authentication
        #. logger
        #. rate limiter
        #. verified captcha
    #. models
    #. queries
    #. routes
        #. ping
        #. settings
        #. subscriptions
        #. webhooks
    #. services
        #. encryption service
        #. paypal service
        #. stripe service
    #. utils
        #. database connections
        #. validations
    #. views
        #. ping
        #. settings
        #. subscriptions
        #. webhooks
#. Test with real test keys
#. Test with real keys

Checklist
##########
1.  Subscription Management
    - [ ] Payment Processing - Support for:
        - [ ] Stripe
            - [ ] Credit/Debit Cards
            - [ ] Apple Pay
            - [ ] Google Pay
        - [ ] PayPal
        - [ ] Cryptocurrency
    - [ ] Tax Calculation & Compliance (e.g., VAT, GST)
    - [ ] Payment Failures & Retries
    - [ ] Subscription Creation - Users can subscribe through:
        - [ ] One-time purchases:
            - [ ] 1 month
            - [ ] 3 months
            - [ ] 1 year
        - [ ] Free trial
    - [ ] Subscription Plans - Support for:
        - [ ] Different plan lengths
            - [ ] Monthly
            - [ ] Yearly
        - [ ] Discount codes & promotions
        - [ ] Multiple tiers (None, Premium)
    - [ ] Subscription Cancellation - Users can:
        - [ ] Cancel anytime
        - [ ] Pause the subscriptions
        - [ ] Resume the subscriptions
        - [ ] Request refunds (within 1 week?)

2.  Invoices
    - [ ] Automated Invoicing - Generate and send invoices for:
        - [ ] Initial payments
        - [ ] Recurring billing
        - [ ] Refunds

3.  Subscription Lifecycle Management
    - [ ] Trial Handling
    - [ ] Grace Periods for failed payments
    - [ ] Proration Handling - Adjust charges when users upgrade/downgrade

4.  User Subscription Controls
    - [ ] View Active Subscription
    - [ ] Upgrade/Downgrade Plans
    - [ ] Add-ons & Extra Features

5.  Security & Fraud Prevention
    - Is this for stripe/paypal to handle?
    - [ ] Rate Limiting - Prevent abuse of free trials
    - [ ] Fraud Detection - Monitor suspicious payment activity
    - [ ] Subscription Sharing Limits

6.  Compliance
    - [ ] GDPR/CCPA Compliance
    - [ ] PCI-DSS Compliance for payment security
    - [ ] Data Retention & Deletion Policies

7.  Notifications & Alerts
    - [ ] Email Notifications for:
        - [ ] Payment confirmation
        - [ ] Subscription renewal
        - [ ] Payment failure alerts
        - [ ] Subscription expiry reminder

8.  APIs & Webhooks
    - [ ] Public APIs for managing subscriptions
    - [ ] Admin APIs for subscription analytics and management
    - [ ] Webhooks for real-time updates (For Stripe, PayPal, Crypto)
        - [ ] Subscription Events - Track user subscription status
            - [ ] Subscription Created - A user starts a subscription
            - [ ] Subscription Updated - Plan upgrades, downgrades, trial extensions, etc.
            - [ ] Subscription deleted - Subscription cancelled or expired
            - [ ] Trial will end - Trial is ending soon
        - [ ] Payment & Invoice Events - handle charges and renewals
            - [ ] Invoice Created
            - [ ] Invoice Finalised
            - [ ] Invoice Payment Payment Succeeded
            - [ ] Invoice. Payment Failed
            - [ ] Invoice Payment Action Required
        - [ ] Checkout and Payment Events - one time purchases
            - [ ] Checkout Session Completed - User successfully completed a checkout session
            - [ ] Payment Intent Succeeded - One-time payment was successful
            - [ ] Payment Failed - Payment attempt failed
        - [ ] Refund and Dispute - Handle Chargebacks and Refund
            - [ ] Charge Refunded
            - [ ] Charge Dispute Created - possible chargeback
            - [ ] Charge Dispute Closed - A dispute was resolved
        - [ ] Customer Handling
            - [ ] Customer Deleted
            - [ ] Customer Updated

9.  Administrative Tools
    - [ ] Dashboard for managing users & subscriptions
    - [ ] Revenue and churn analytics

10. Documentation & Support
    - [ ] User Guides for managing subscriptions
    - [ ] Developer API Documentation
