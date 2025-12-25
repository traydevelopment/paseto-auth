use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Claims {
    pub user_id: String,        // User ID
    pub level: i32,
    pub user_name: String,
    pub company_id: String,
    pub company_name: String,
    pub exp: i64,           // Unix timestamp (saniye)
    pub iat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>, // Optional
}

impl Claims {
    pub fn new(user_id: String, level: i32, user_name: String, company_id: String, company_name: String, ttl_seconds: u64) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            user_id,
            level,
            user_name ,
            company_id,
            company_name,
            iat: now,
            exp: now + ttl_seconds as i64,
            scope: None,
        }
    }

    pub fn with_user_name(mut self, user_name: String) -> Self {
        self.user_name = user_name;
        self
    }

    pub fn with_compnay_id(mut self, company_id: String) -> Self {
        self.company_id = company_id;
        self
    }

    pub fn with_compnay_name(mut self, company_name: String) -> Self {
        self.company_name = company_name;
        self
    }

    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        self.exp > now && self.iat <= now
    }
}

impl Claims {
    /// Struct'ı JSON string'e dönüştürür.
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// JSON string'den SampleModel oluşturur.
    pub fn from_json_string(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap_or_default()
    }
}
