# Subscriptions Tables
## Payment Tier Enum
```sql
CREATE TYPE payment_tier_enum AS ENUM('none', 'premium');
```

none = unsubscribed

## Payment Type Enum
```sql
CREATE TYPE payment_type_enum AS ENUM(
    'Subscription_Monthly',
    'Subscription_Yearly',
    'Payment_1_Month',
    'Payment_3_Months',
    'Payment_1_Year'
);
```
## Prices
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| payment_tier_enum         | ENUM         | Account payment tiers          | False  | True     | True  |
| payment_type_enum         | ENUM         | Allowed Payment type           | False  | True     | True  |
| price_gbp                 | DECIMAL      | base price in gbp              | False  | True     | True  |

```sql
CREATE TYPE prices (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    payment_tier_enum PAYMENT_TIER_ENUM NOT NULL,
    payment_type_enum PAYMENT_TYPE_ENUM NOT NULL,
    price_gbp DECIMAL NOT NULL
)
```

## Payment Method Enum
```sql
CREATE TYPE payment_method_enum AS ENUM ('stripe', 'paypal', 'crypto', 'none');
```

## Subscribers
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                   | INT          | Foreign key to user id         | False  | True     | False |
| encrypted_customer_id     | VARCHAR(255) | The customer id of the payee   | True   | False    | True  |
| payment_method            | Enum         | Foreign key to payment methods | False  | True     | False |

```sql
CREATE TABLE subscribers (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    customer_id VARCHAR(255) UNIQUE DEFAULT NULL,
    payment_method PAYMENT_METHOD_ENUM NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)
```

## Subscription History
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| subscriber_id             | INT          | Foreign key to the subscriber  | True   | True     | True  |
| encrypted_subscription_id | VARCHAR(255) | The id of the subscription     | True   | True     | True  |
| payment_tier_enum         | ENUM         | The user's payment tier        | False  | True     | False |
| subscription_start_date   | TIMESTAMP    | Subscription start date        | False  | True     | False |
| subscription_end_date     | TIMESTAMP    | Subscription end date          | False  | False    | False |
| cancellation_date         | TIMESTAMP    | Subscription cancellation date | False  | False    | False |

```sql
CREATE TABLE subscription_history (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    subscriber_id INT NOT NULL,
    encrypted_subscription_id VARCHAR(255) UNIQUE NOT NULL,
    payment_tier_enum PAYMENT_TIER_ENUM NOT NULL DEFAULT 'premium',
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
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                   | INT          | Foreign key to user id         | False  | True     | False |
| payment_id                | VARCHAR(255) | The id of the payment          | True   | True     | True  |
| payment_method            | Enum         | Foreign key to payment methods | False  | True     | False |
| payment_tier_enum         | ENUM         | The user's payment tier        | False  | True     | False |
| payment_date              | TIMESTAMP    | Subscription cancellation date | False  | True     | False |
| duration                  | INTERVAL     | Premium duration               | False  | True     | False |
| payment_start_date        | TIMESTAMP    | Time premium was started       | False  | False    | False |
| payment_end_date          | TIMESTAMP    | Time premium was ended         | False  | False    | False |

```sql
CREATE TABLE payment_history (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    payment_id VARCHAR(255) UNIQUE NOT NULL,
    payment_method PAYMENT_METHOD_ENUM NOT NULL,
    payment_tier_enum PAYMENT_TIER_ENUM NOT NULL DEFAULT 'premium',
    payment_date TIMESTAMP NOT NULL,
    duration INTERVAL NOT NULL,
    payment_start_date TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)
```

## Subscription Metadata
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| user_id                   | INT          | Foreign key to user id         | True   | True     | False |
| payment_tier_enum         | ENUM         | The user's payment tier        | False  | True     | False |
| subscription_history_id   | INT          | ID of last used history item   | True   | False    | True  |
| payment_history_id        | INT          | ID of last used history item   | True   | False    | True  |
| has_trial                 | BOOLEAN      | Does user have a trial or not  | False  | True     | False |
| trial_start_date          | TIMESTAMP    | When did the trial start       | False  | False    | False |
| trial_end_date            | TIMESTAMP    | When did the trial end         | False  | False    | False |

```sql
CREATE TABLE subscription_metadata (
    user_id INT PRIMARY KEY,
    payment_tier_enum PAYMENT_TIER_ENUM NOT NULL DEFAULT 'none',
    subscription_history_id INT,
    payment_history_id INT,
    has_trial BOOLEAN NOT NULL DEFAULT TRUE,
    trial_start_date TIMESTAMP,
    trial_end_date TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT check_only_one_refund CHECK (
        (subscription_history_id IS NOT NULL AND payment_history_id IS NULL) OR
        (subscription_history_id IS NULL AND payment_history_id IS NOT NULL) OR
        (subscription_history_id IS NULL AND payment_history_id IS NULL)
    )
);
```

## Discount Codes
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| discount_code             | VARCHAR(20)  | The discount code in question  | True   | True     | True  |
| max_uses                  | INT          | Maximum number of code usages  | False  | False    | False |
| current_uses              | INT          | Current number of code usages  | False  | True     | False |
| created_at                | TIMESTAMP    | Time the code was created      | False  | True     | False |
| expires_at                | TIMESTAMP    | Time the code expires          | False  | False    | False |
| discount_value            | DECIMAL      | Value of the discount          | False  | True     | True  |
| discount_type             | ENUM         | Type of discount (% or fixed)  | False  | True     | False |

```sql
CREATE TYPE discount_type_enum AS ENUM('percentage', 'fixed');
CREATE TABLE discount_codes (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    discount_code VARCHAR(20) UNIQUE NOT NULL,
    max_uses INT DEFAULT NULL,
    current_uses INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP DEFAULT NULL,
    discount_value DECIMAL(10,2) NOT NULL,
    discount_type DISCOUNT_TYPE_ENUM NOT NULL,
)
```

## Discount Payment Types
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| discount_code_id          | INT          | Discount code foreign key      | False  | True     | True  |
| payment_type_enum         | ENUM         | Allowed Payment type           | False  | True     | True  |

Allowed payment types for each code

```sql
CREATE TABLE discount_payment_types (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    discount_code_id INT NOT NULL,
    payment_type PAYMENT_TYPE_ENUM NOT NULL,
)
```

## Applied Discounts
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                   | INT          | Foreign key to user id         | False  | True     | False |
| discount_code_id          | INT          | Discount code foreign key      | False  | True     | False |
| applied_at                | TIMESTAMP    | Discount code application date | False  | True     | False |
| payment_history_id        | INT          | Payment history id             | True   | False    | False |
| subscription_history_id   | INT          | Subscription history id        | True   | False    | False |

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
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| user_id                   | INT          | Foreign key to user id         | False  | True     | False |
| payment_history_id        | INT          | Payment history id             | True   | False    | False |
| subscription_history_id   | INT          | Subscription history id        | True   | False    | False |
| refund_date               | TIMESTAMP    | Date of refund                 | False  | False    | False |

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
