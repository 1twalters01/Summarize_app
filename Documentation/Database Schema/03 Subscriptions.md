# Subscriptions Tables
## Subscription Metadata
| Field                   | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------------|--------------|--------------------------------|--------|----------|-------|
| user_id                 | INT          | Foreign key to user id         | True   | True     | False |
| is_subscribed           | BOOLEAN      | Subscription status            | False  | True     | False |
| has_trial               | BOOLEAN      | Does user have a trial or not  | False  | True     | False |
| trial_start_date        | TIMESTAMP    | When did the trial start       | False  | False    | False |
| trial_end_date          | TIMESTAMP    | When did the trial end         | False  | False    | False |

```sql
CREATE TABLE subscription_metadata (
    user_id INT PRIMARY KEY,
    is_subscribed BOOLEAN NOT NULL DEFAULT FALSE,
    has_trial BOOLEAN NOT NULL DEFAULT TRUE,
    trial_start_date TIMESTAMP,
    trial_end_date TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

## Payment method enum
```sql
CREATE TYPE payment_method_enum AS ENUM ('stripe', 'paypal', 'crypto', 'none');
```

## Subscribers
| Field                   | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------------|--------------|--------------------------------|--------|----------|-------|
| id                      | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                 | INT          | Foreign key to user id         | False  | True     | False |
| customer_id             | VARCHAR(255) | The customer id of the payee   | True   | False    | True  |
| payment_method          | Enum         | Foreign key to payment methods | False  | True     | False |

```sql
CREATE TABLE subscribers (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    customer_id VARCHAR(255) UNIQUE,
    payment_method PAYMENT_METHOD_ENUM NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)
```

## Subscription History
| Field                   | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------------|--------------|--------------------------------|--------|----------|-------|
| id                      | INT          | Primary key (internal)         | True   | True     | True  |
| subscriber_id           | INT          | Foreign key to the subscriber  | True   | True     | True  |
| subscription_id         | VARCHAR(255) | The id of the subscription     | True   | True     | True  |
| subscription_start_date | TIMESTAMP    | Subscription start date        | False  | True     | False |
| subscription_end_date   | TIMESTAMP    | Subscription end date          | False  | False    | False |
| cancellation_date       | TIMESTAMP    | Subscription cancellation date | False  | False    | False |

```sql
CREATE TABLE subscriber_history (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    subscriber_id INT,
    subscription_id VARCHAR(255) UNIQUE NOT NULL,
    subscription_start_date TIMESTAMP NOT NULL,
    subscription_end_date TIMESTAMP,
    cancellation_date TIMESTAMP,
    CONSTRAINT fk_subscribers FOREIGN KEY (subscriber_id)
        REFERENCES subscribers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)
```

## Payment History
| Field                   | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------------|--------------|--------------------------------|--------|----------|-------|
| id                      | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                 | INT          | Foreign key to user id         | False  | True     | False |
| payment_method          | Enum         | Foreign key to payment methods | False  | True     | False |
| payment_date            | TIMESTAMP    | Subscription cancellation date | False  | True     | False |
| duration                | INTERVAL     | Premium duration               | False  | True     | False |
| payment_start_date      | TIMESTAMP    | Time premium was started       | False  | False    | False |
| payment_end_date        | TIMESTAMP    | Time premium was ended         | False  | False    | False |

```sql
CREATE TABLE payment_history (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    payment_method PAYMENT_METHOD_ENUM NOT NULL,
    payment_date TIMESTAMP NOT NULL,
    duration INTERVAL NOT NULL,
    payment_start_date TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)
```

## Discount Codes
| Field                   | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------------|--------------|--------------------------------|--------|----------|-------|
| id                      | INT          | Primary key (internal)         | True   | True     | True  |
| code                    | VARCHAR(20)  | The code in question           | True   | True     | True  |
| max_uses                | INT          | Maximum number of code usages  | False  | False    | False |
| current_uses            | INT          | Current number of code usages  | False  | True     | False |
| created_at              | TIMESTAMP    | Time the code was created      | False  | True     | False |
| expires_at              | TIMESTAMP    | Time the code expires          | False  | False    | False |

```sql
CREATE TABLE discount_codes (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code VARCHAR(20) UNIQUE NOT NULL,
    max_uses INT DEFAULT NULL,
    current_uses INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP DEFAULT NULL
)
```

## Applied Discounts
| Field                   | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------------|--------------|--------------------------------|--------|----------|-------|
| id                      | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                 | INT          | Foreign key to user id         | False  | True     | False |
| discount_code_id        | INT          | Discount code foreign key      | False  | True     | False |
| applied_at              | TIMESTAMP    | Discount code application date | False  | True     | False |
| payment_history_id      | INT          | Payment history id             | True   | False    | False |
| subscription_history_id | INT          | Subscription history id        | True   | False    | False |

```sql
CREATE TABLE applied_discounts (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    discount_code_id INT NOT NULL,
    applied_at TIMESTAMP NOT NULL,
    payment_history_id INT,
    subscription_history_id INT,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_payment_history FOREIGN KEY (payment_history_id)
        REFERENCES payment_history (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_subscription_history FOREIGN KEY (subscription_history_id)
        REFERENCES subscription_history (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT check_only_one_refund CHECK (
        (subscription_history_id IS NOT NULL AND payment_history_id IS NULL) OR
        (subscription_history_id IS NULL AND payment_history_id IS NOT NULL)
    )
)
```

## Refund Requests
| Field                   | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|-------------------------|--------------|--------------------------------|--------|----------|-------|
| id                      | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                 | INT          | Foreign key to user id         | False  | True     | False |
| payment_history_id      | INT          | Payment history id             | True   | False    | False |
| subscription_history_id | INT          | Subscription history id        | True   | False    | False |
| refund_date             | TIMESTAMP    | Date of refund                 | False  | False    | False |

```sql
CREATE TYPE refund_status_enum AS ENUM ('pending', 'approved', 'rejected');
CREATE TABLE refund_requests (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    payment_history_id INT,
    subscription_history_id INT,
    refund_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    refund_status REFUND_STATUS_ENUM NOT NULL DEFAULT 'pending',
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_payment_history FOREIGN KEY (payment_history_id)
        REFERENCES payment_history (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_subscription_history FOREIGN KEY (subscription_history_id)
        REFERENCES subscription_history (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT check_only_one_refund CHECK (
        (subscription_history_id IS NOT NULL AND payment_history_id IS NULL) OR
        (subscription_history_id IS NULL AND payment_history_id IS NOT NULL)
    )
)
```
