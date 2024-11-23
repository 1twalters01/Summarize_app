# Subscriptions Tables
## Payment Methods
| Field             | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------|--------------|--------------------------------|--------|----------|-------|
| id                | INT          | Primary key                    | True   | True     | True  |
| payment_method    | VARCHAR(20)  | Payment method name            | True   | True     | False |

```sql
CREATE TYPE payment_method_enum AS ENUM ('stripe', 'paypal');
```

## Subscribers
| Field             | Type         | Description                    | UNIQUE | NOT NULL | Index |
|-------------------|--------------|--------------------------------|--------|----------|-------|
| user_id           | INT          | Foreign key to user id         | True   | True     | False |
| customer_id       | VARCHAR(255) | The id of the customer         | True   | False    | True  |
| subscription_id   | VARCHAR(255) | The id of the subscription     | True   | False    | True  |
| payment_method    | ENUM         | Payment method Enum            | True   | True     | False |
| is_subscribed     | BOOLEAN      | Subscription status            | False  | True     | False |
| has_trial         | BOOLEAN      | has a trial or not             | False  | True     | False |
| start_date        | TIMESTAMP    | Subscription start date        | False  | True     | False |
| end_date          | TIMESTAMP    | Subscription end date          | False  | False    | False |

```sql
CREATE TABLE IF NOT EXISTS subscribers(
    user_id INT UNIQUE NOT NULL,
    customer_id VARCHAR(255) UNIQUE,
    subscription_id VARCHAR(255) UNIQUE,
    payment_method payment_method_enum NOT NULL,
    is_subscribed BOOL NOT NULL DEFAULT FALSE,
    has_trial BOOL NOT NULL DEFAULT TRUE,
    start_date TIMESTAMP NOT NULL DEFAULT NOW(),
    end_date TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT check_date_validity CHECK (end_date IS NULL OR start_date < end_date)
);
CREATE INDEX IF NOT EXISTS idx_subscribers_customer_id ON subscribers (customer_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_subscription_id ON subscribers (subscription_id);
```

## Subscription History
| Field             | Type         | Description                    | UNIQUE | NOT NULL | Index |
|-------------------|--------------|--------------------------------|--------|----------|-------|
| id                | INT          | Primary key                    | True   | True     | True  |
| user_id           | INT          | Foreign key to user id         | True   | True     | True  |
| customer_id       | VARCHAR(255) | The id of the customer         | False  | False    | False |
| subscription_id   | VARCHAR(255) | The id of the subscription     | False  | False    | False |
| payment_method_id | INT[]        | Foreign key to payment methods | False  | False    | False |
| start_date        | TIMESTAMP    | Subscription start date        | False  | TRUE     | False |
| end_date          | TIMESTAMP    | Subscription end date          | False  | False    | False |

```sql
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
        ON UPDATE CASCADE,
    CONSTRAINT check_date_validity CHECK (start_date < end_date)
);
CREATE INDEX IF NOT EXISTS idx_subscription_history_uid ON subscription_history (user_id);
```
