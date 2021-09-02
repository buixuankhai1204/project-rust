use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};
use warp::{self, Filter};

fn check_password(password: &str) -> Result<(), ValidationError> {
    if (password.len() <= 5) || (password.len() > 50) {
        return Err(ValidationError::new("invalid_password"));
    }
    Ok(())
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4), custom = "check_password")]
    pub password: String,
}

impl Default for LoginRequest {
    fn default() -> Self {
        LoginRequest {
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

impl LoginRequest {
    pub fn queries() -> impl Filter<Extract = (LoginRequest,), Error = warp::Rejection> + Clone {
        warp::query::<LoginRequest>()
    }

    pub fn parameters() -> impl Filter<Extract = (LoginRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 8).and(warp::body::json())
    }
}
