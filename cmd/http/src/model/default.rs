use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultBody {
    pub protocol: String,
    pub name: String,
}

impl Default for DefaultBody {
    fn default() -> Self {
        DefaultBody {
            protocol: "HTTP".to_string(),
            name: "fdglapi_http".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: Option<String>,
}

impl Default for ErrorResponse {
    fn default() -> Self {
        ErrorResponse {
            code: 1,
            message: Some("Something went wrong!".to_string()),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShipmentErrorResponse<T> {
    pub code: u16,
    pub message: Option<String>,
    pub extra_info: Option<T>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BodyResponse<T> {
    pub code: i32,
    pub message: Option<String>,
    pub payload: Option<T>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UploadResponse {
    pub url: String,
    pub user_id: i64,
    pub user_hash_id: String,
}

impl Default for UploadResponse {
    fn default() -> Self {
        UploadResponse {
            url: "".to_string(),
            user_id: 0,
            user_hash_id: "".to_string(),
        }
    }
}
