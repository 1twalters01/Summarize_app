# Subscriptions Tables
## Payment Methods
CREATE TABLE IF NOT EXISTS payment_methods(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    payment_method VARCHAR(20) UNIQUE NOT NULL,
);

Payment Method
| Field              | Type              | Description                    | IS UNIQUE | NOT NULL | INDEX  |
|--------------------|-------------------|--------------------------------|-----------|----------|--------|
| id                 | INT               | Primary key                    | True      | True     | True   |
| payment_method     | VARCHAR(20)       | Payment method name            | True      | True     | True   |

[comment]: # (methods: stripe, paypal)

## Subscribers
CREATE TABLE IF NOT EXISTS subscribers(
    user_id INT UNIQUE NOT NULL,
    payment_method_id INT NOT NULL,
    is_subscribed BOOL NOT NULL DEFAULT FALSE,
    has_trial BOOL NOT NULL DEFAULT TRUE,
    start_date TIMESTAMP NOT NULL DEFAULT NOW(),
    end_date TIMESTAMP,
    customer_id VARCHAR(255) UNIQUE,
    subscription_id VARCHAR(255) UNIQUE,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_methods FOREIGN KEY (payment_method_id)
        REFERENCES payment_methods (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT check_date_validity CHECK (end_date IS NULL OR start_date < end_date)
);

Subscriber
| Field              | Type              | Description                    | IS UNIQUE | NOT NULL | Index  |
|--------------------|-------------------|--------------------------------|-----------|----------|--------|
| user_id            | INT               | Foreign key to user id         | True      | True     |  |
| payment_method_id  | INT               | Foreign key to method id       | True      | True     |  |
| is_subscribed      | BOOLEAN           | Subscription status            | False     | True     | True |
| has_trial          | BOOLEAN           | has a trial or not             | FALSE     | True     |  |
| start_date         | TIMESTAMP         | Subscription start date        | False     | True     |  |
| end_date           | TIMESTAMP         | Subscription end date          | False     | False    |  |
| customer_id        | VARCHAR(255)      | The id of the customer         | True      | False    |  |
| subscription_id    | VARCHAR(255)      | The id of the subscription     | True      | False    |  |

## Subscription History
CREATE TABLE IF NOT EXISTS subscription_history(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    customer_id VARCHAR(255),
    subscription_id VARCHAR(255),
    payment_method_id INT,
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_methods FOREIGN KEY (method_id)
        REFERENCES payment_methods (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT check_date_validity CHECK (start_date < end_date)
)

Previous subscriptions - needed?
| Field              | Type              | Description                    | IS UNIQUE | NOT NULL | Index  |
|--------------------|-------------------|--------------------------------|-----------|----------|--------|
| id                 | INT               | Primary key                    | True      | True     |  |
| user_id            | INT               | Foreign key to user id         | True      | True     |  |
| customer_id        | VARCHAR(255)      | The id of the customer         | False     | False    |  |
| subscription_id    | VARCHAR(255)      | The id of the subscription     | False     | False    |  |
| payment_method_id  | INT[]             | Foreign key to payment methods | False     | False    |  |
| start_date         | TIMESTAMP         | Subscription start date        | False     | TRUE     |  |
| end_date           | TIMESTAMP         | Subscription end date          | False     | False    |  |
