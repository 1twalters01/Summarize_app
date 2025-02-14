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
    user_id INT,
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
    user_id INT,
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
