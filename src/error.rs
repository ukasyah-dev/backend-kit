#[derive(Debug, thiserror::Error)]
#[error("{} {}", .code, .message)]
pub struct Error {
    pub code: u16,
    pub message: String,
}

pub fn already_exists() -> Error {
    Error {
        code: 409,
        message: "already exists".to_string(),
    }
}

pub fn already_exists_with_message(msg: &str) -> Error {
    Error {
        code: 409,
        message: msg.to_string(),
    }
}

pub fn internal() -> Error {
    Error {
        code: 500,
        message: "internal".to_string(),
    }
}

pub fn internal_with_message(msg: &str) -> Error {
    Error {
        code: 500,
        message: msg.to_string(),
    }
}

pub fn invalid_argument() -> Error {
    Error {
        code: 400,
        message: "invalid argument".to_string(),
    }
}

pub fn invalid_argument_with_message(msg: &str) -> Error {
    Error {
        code: 400,
        message: msg.to_string(),
    }
}

pub fn not_found() -> Error {
    Error {
        code: 404,
        message: "not found".to_string(),
    }
}

pub fn not_found_with_message(msg: &str) -> Error {
    Error {
        code: 404,
        message: msg.to_string(),
    }
}

pub fn permission_denied() -> Error {
    Error {
        code: 403,
        message: "permission denied".to_string(),
    }
}

pub fn permission_denied_with_message(msg: &str) -> Error {
    Error {
        code: 403,
        message: msg.to_string(),
    }
}

pub fn unauthenticated() -> Error {
    Error {
        code: 401,
        message: "unauthenticated".to_string(),
    }
}

pub fn unauthenticated_with_message(msg: &str) -> Error {
    Error {
        code: 401,
        message: msg.to_string(),
    }
}
