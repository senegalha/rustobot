use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Viewer,
    Announcer,
    Admin,
    #[serde(rename = "super_admin")]
    #[sqlx(rename = "super_admin")]
    SuperAdmin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "face_verification_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum FaceVerificationStatus {
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "service_category_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum ServiceCategory {
    Streaming,
    Cloud,
    Gaming,
    Education,
    Tools,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "periodicity_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum Periodicity {
    Monthly,
    Quarterly,
    Semiannual,
    Annual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "access_type_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum AccessType {
    #[serde(rename = "email_invite")]
    #[sqlx(rename = "email_invite")]
    EmailInvite,
    #[serde(rename = "activation_code")]
    #[sqlx(rename = "activation_code")]
    ActivationCode,
    Credentials,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "service_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum ServiceStatus {
    Available,
    #[serde(rename = "waiting_members")]
    #[sqlx(rename = "waiting_members")]
    WaitingMembers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "log_severity_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum LogSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub telegram_id: i64,
    pub telegram_username: String,
    pub name: String,
    pub surname: String,
    pub status: UserStatus,
    pub role: UserRole,
    pub face_verification_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FaceVerification {
    pub id: Uuid,
    pub telegram_id: i64,
    pub video_file_id: String,
    pub azure_face_id: Option<String>,
    pub verification_date: DateTime<Utc>,
    pub confidence_score: Option<i32>,
    pub status: FaceVerificationStatus,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServiceSuggestion {
    pub id: Uuid,
    pub service_name: String,
    pub normalized_name: String,
    pub category: ServiceCategory,
    pub icon_emoji: String,
    pub reference_price_per_slot: String,
    pub max_price_per_slot: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Service {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub service_suggestion_id: Option<Uuid>,
    pub service_name: String,
    pub category: ServiceCategory,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Announcement {
    pub id: Uuid,
    pub service_id: Uuid,
    pub telegram_message_id: i64,
    pub channel_id: i64,
    pub price_per_slot: String,
    pub periodicity: Periodicity,
    pub access_type: AccessType,
    pub status: ServiceStatus,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub details: Option<serde_json::Value>,
    pub telegram_message_id: Option<i64>,
    pub severity: LogSeverity,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub telegram_id: i64,
    pub telegram_username: String,
    pub name: String,
    pub surname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementCreateRequest {
    pub service_name: String,
    pub price_per_slot: String,
    pub periodicity: Periodicity,
    pub access_type: AccessType,
    pub status: ServiceStatus,
}
