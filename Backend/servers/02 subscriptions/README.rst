Subscriptions microservice created in python.

Why Python
###########
* Lots of 3rd party apis use python
* The speed bottleneck is in 3rd party apis so language speed is not an issue
* Is extremely easy to work with and has lots of libraries/web development frameworks
* Will have relatively low usage compared to other micro-services so memory is not an issue

Overview
#########
* Payment providers are speed bottlenecks
* Python was chosen as it is extremely simple to code in and the speed was not an issue
* FastAPI is the framework of choice as it is fast and has good async capabilities

Details
########
* Stripe and Paypal are currently in use
* Webhooks for each are created
* Has settings to control subscriptions
