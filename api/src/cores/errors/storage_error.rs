use bcrypt::BcryptError;
use std::fmt;

pub enum StoreError {
    HashError(BcryptError),
    DBError(migration::DbErr),
    PasswordNotMatch(String),
    WrongPassword(String),
}

impl From<BcryptError> for StoreError {
    fn from(error: BcryptError) -> Self {
        StoreError::HashError(error)
    }
}

impl From<migration::DbErr> for StoreError {
    fn from(error: migration::DbErr) -> Self {
        StoreError::DBError(error)
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::HashError(error) => write!(f, "{}", error),
            StoreError::DBError(error) => write!(f, "{}", error),
            StoreError::PasswordNotMatch(error) => write!(f, "{}", error),
            StoreError::WrongPassword(error) => write!(f, "{}", error),
        }
    }
}
