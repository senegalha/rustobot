CREATE TYPE user_status_enum AS ENUM ('active', 'blocked');

CREATE TYPE user_role_enum AS ENUM ('viewer', 'announcer', 'admin', 'super_admin');

CREATE TYPE face_verification_status_enum AS ENUM ('pending', 'verified', 'rejected');

CREATE TYPE service_category_enum AS ENUM ('streaming', 'cloud', 'gaming', 'education', 'tools', 'other');

CREATE TYPE periodicity_enum AS ENUM ('monthly', 'quarterly', 'semiannual', 'annual');

CREATE TYPE access_type_enum AS ENUM ('email_invite', 'activation_code', 'credentials');

CREATE TYPE service_status_enum AS ENUM ('available', 'waiting_members');

CREATE TYPE log_severity_enum AS ENUM ('info', 'warning', 'error', 'critical');
