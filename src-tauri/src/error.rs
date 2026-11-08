use serde::Serialize;

/// Errors returned from Tauri commands. Serialized as `{ kind, message }` so the
/// UI can branch on `kind` and show `message`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Serialize, specta::Type)]
pub struct ErrorPayload {
    pub kind: &'static str,
    pub message: String,
}

impl AppError {
    fn kind(&self) -> &'static str {
        match self {
            AppError::Db(_) => "db",
            AppError::Migrate(_) => "migrate",
            AppError::Io(_) => "io",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        ErrorPayload {
            kind: self.kind(),
            message: self.to_string(),
        }
        .serialize(s)
    }
}

impl specta::Type for AppError {
    fn definition(types: &mut specta::Types) -> specta::datatype::DataType {
        ErrorPayload::definition(types)
    }
}

pub type AppResult<T> = Result<T, AppError>;
