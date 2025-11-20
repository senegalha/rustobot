CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    telegram_id BIGINT UNIQUE NOT NULL,
    telegram_username VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    surname VARCHAR(100) NOT NULL,
    status user_status_enum NOT NULL DEFAULT 'active',
    role user_role_enum NOT NULL DEFAULT 'viewer',
    face_verification_id UUID UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_telegram_id ON users(telegram_id);
CREATE INDEX idx_users_username ON users(telegram_username);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_role ON users(role);

CREATE TABLE face_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    telegram_id BIGINT NOT NULL,
    video_file_id VARCHAR(255) NOT NULL,
    azure_face_id VARCHAR(255) UNIQUE,
    verification_date TIMESTAMP NOT NULL,
    confidence_score DECIMAL(5,2),
    status face_verification_status_enum NOT NULL,
    rejection_reason TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_face_verifications_telegram_id ON face_verifications(telegram_id);
CREATE INDEX idx_face_verifications_azure_face_id ON face_verifications(azure_face_id);
CREATE INDEX idx_face_verifications_status ON face_verifications(status);

ALTER TABLE users
    ADD CONSTRAINT fk_users_face_verification
    FOREIGN KEY (face_verification_id)
    REFERENCES face_verifications(id)
    ON DELETE SET NULL;

CREATE TABLE service_suggestions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_name VARCHAR(255) UNIQUE NOT NULL,
    normalized_name VARCHAR(255) UNIQUE NOT NULL,
    category service_category_enum NOT NULL,
    icon_emoji VARCHAR(10) NOT NULL,
    reference_price_per_slot NUMERIC(10,2) NOT NULL CHECK (reference_price_per_slot > 0),
    max_price_per_slot NUMERIC(10,2) NOT NULL CHECK (max_price_per_slot >= reference_price_per_slot),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_service_suggestions_normalized ON service_suggestions(normalized_name);
CREATE INDEX idx_service_suggestions_category ON service_suggestions(category);
CREATE INDEX idx_service_suggestions_name_trgm ON service_suggestions USING gin(normalized_name gin_trgm_ops);

CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL,
    service_suggestion_id UUID,
    service_name VARCHAR(255) NOT NULL,
    category service_category_enum NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_services_owner FOREIGN KEY (owner_id)
        REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_services_suggestion FOREIGN KEY (service_suggestion_id)
        REFERENCES service_suggestions(id) ON DELETE SET NULL,
    CONSTRAINT uq_user_service UNIQUE (owner_id, service_name)
);

CREATE INDEX idx_services_owner ON services(owner_id);
CREATE INDEX idx_services_suggestion ON services(service_suggestion_id);
CREATE INDEX idx_services_category ON services(category);
CREATE INDEX idx_services_name_search ON services USING gin(service_name gin_trgm_ops);

CREATE TABLE announcements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_id UUID NOT NULL,
    telegram_message_id BIGINT UNIQUE NOT NULL,
    channel_id BIGINT NOT NULL,
    price_per_slot NUMERIC(10,2) NOT NULL CHECK (price_per_slot > 0),
    periodicity periodicity_enum NOT NULL,
    access_type access_type_enum NOT NULL,
    status service_status_enum NOT NULL DEFAULT 'available',
    expiration_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_announcements_service FOREIGN KEY (service_id)
        REFERENCES services(id) ON DELETE CASCADE
);

CREATE INDEX idx_announcements_service ON announcements(service_id);
CREATE INDEX idx_announcements_expiration ON announcements(expiration_date);
CREATE INDEX idx_announcements_status ON announcements(status);

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    action_type VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50),
    entity_id UUID,
    details JSONB,
    telegram_message_id BIGINT,
    severity log_severity_enum NOT NULL DEFAULT 'info',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_audit_logs_user FOREIGN KEY (user_id)
        REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action_type);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type);
CREATE INDEX idx_audit_logs_severity ON audit_logs(severity);
CREATE INDEX idx_audit_logs_created ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_logs_details ON audit_logs USING gin(details);
