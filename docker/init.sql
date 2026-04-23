CREATE TABLE payments (
    id SERIAL PRIMARY KEY,
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    amount DOUBLE PRECISION NOT NULL,
    currency VARCHAR(4) NOT NULL,
    payment_method VARCHAR(50) NOT NULL,
    payment_reason VARCHAR(255) NOT NULL,
    status enum('pending', 'completed', 'failed') NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);