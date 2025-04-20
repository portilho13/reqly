use std::collections::HashMap;


enum RequestType {
    GET,
    POST,
    PUT
}

// Add missing fields
pub struct Http {
    url: String,
    request_type: Option<RequestType>,
    params: Option<HashMap<String, String>>,
    headers: Option<HashMap<String, String>>,
}