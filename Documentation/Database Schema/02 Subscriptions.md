# Subscriptions Tables

Payment Method
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| method_id       | INT            | Primary key                | True      | True     | True   |
| method          | VARCHAR(20)    | Payment method name        | True      | True     | True   |

[comment]: # (methods: stripe, paypal)

Subscriber
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | Index  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id     | True      | True     |  |
| method_id       | INT            | Foreign key to method id   | True      | True     |  |
| is_subscribed   | BOOLEAN        | Subscription status        | False     | True     |  |
| has_trial       | BOOLEAN        | has a trial or not         | FALSE     | True     |  |
| start_date      | TIMESTAMP      | Subscription start date    | False     | False    |  |
| end_date        | TIMESTAMP      | Subscription end date      | False     | False    |  |
| customer_id     | VARCHAR(255)   | The id of the customer     | True      | False    |  |
| subscription_id | VARCHAR(255)   | The id of the subscription | True      | False    |  |

Previous subscriptions - needed?
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | Index  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id     | True      | True     |  |
| customer_id     | VARCHAR(255)[] | The id of the customer     | True      | False    |  |
| subscription_id | VARCHAR(255)[] | The id of the subscription | True      | False    |  |