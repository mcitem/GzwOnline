use axum::response::Response;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use serde_json::json;
use serde_json::Value;
pub struct APIResponse {
    pub meta: Meta,
    pub data: Value,
}
pub struct Meta {
    pub code: Code,
    pub status: StatusCode,
    pub message: &'static str,
}
impl APIResponse {
    pub fn new() -> Self {
        APIResponse {
            meta: Meta {
                code: Code::OK,
                status: StatusCode::OK,
                message: Code::OK.msg(),
            },
            data: json!({}),
        }
    }
    pub fn code(mut self, code: Code) -> Self {
        self.meta.code = code;
        self
    }
    pub fn status(mut self, status: StatusCode) -> Self {
        self.meta.status = status;
        self
    }
    pub fn message(mut self, message: &'static str) -> Self {
        self.meta.message = message;
        self
    }
    pub fn data(mut self, data: Value) -> Self {
        self.data = data;
        self
    }
}
impl IntoResponse for APIResponse {
    fn into_response(self) -> Response {
        (
            self.meta.status,
            Json(json!({
                "meta": {
                    "code": self.meta.code.as_u16(),
                    "status": self.meta.status.as_u16(),
                    "message": self.meta.message,
                },
                "data": self.data,
            })),
        )
            .into_response()
    }
}
pub struct APIError {
    pub meta: Meta,
    pub err: Value,
    empty_data_type: EmptyDataType,
}
pub enum EmptyDataType {
    Null,
    EmptyArray,
    EmptyObject,
}
impl APIError {
    pub fn new_for_500() -> Self {
        APIError {
            meta: Meta {
                code: Code::INTERNAL_SERVER_ERROR,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: Code::INTERNAL_SERVER_ERROR.msg(),
            },
            err: json!({}),
            empty_data_type: EmptyDataType::EmptyObject,
        }
    }
    pub fn new_for_404() -> Self {
        APIError {
            meta: Meta {
                code: Code::NOT_FOUND,
                status: StatusCode::NOT_FOUND,
                message: Code::NOT_FOUND.msg(),
            },
            err: json!({}),
            empty_data_type: EmptyDataType::EmptyObject,
        }
    }
    pub fn new_for_bad_request() -> Self {
        APIError {
            meta: Meta {
                code: Code::BAD_REQUEST,
                status: StatusCode::BAD_REQUEST,
                message: Code::BAD_REQUEST.msg(),
            },
            err: json!({}),
            empty_data_type: EmptyDataType::EmptyObject,
        }
    }
    pub fn new_for_200(empty_data_type: EmptyDataType) -> Self {
        APIError {
            meta: Meta {
                code: Code::OK,
                status: StatusCode::OK,
                message: Code::OK.msg(),
            },
            err: json!({}),
            empty_data_type,
        }
    }
    pub fn code(mut self, code: Code) -> Self {
        self.meta.code = code;
        self
    }
    pub fn status(mut self, status: StatusCode) -> Self {
        self.meta.status = status;
        self
    }
    pub fn message(mut self, message: &'static str) -> Self {
        self.meta.message = message;
        self
    }
    pub fn err(mut self, err: Value) -> Self {
        self.err = err;
        self
    }
    pub fn empty_data_type(mut self, empty_data_type: EmptyDataType) -> Self {
        self.empty_data_type = empty_data_type;
        self
    }
}
impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        (
            self.meta.status,
            Json(json!({
                "meta": {
                    "code": self.meta.code.as_u16(),
                    "status": self.meta.status.as_u16(),
                    "message": self.meta.message,
                },
                "data": match self.empty_data_type {
                            EmptyDataType::Null => json!(null),
                            EmptyDataType::EmptyArray => json!([]),
                            EmptyDataType::EmptyObject => json!({}),
                        },
                "err": self.err,
            })),
        )
            .into_response()
    }
}
impl From<DbErr> for APIError {
    fn from(err: DbErr) -> Self {
        APIError::new_for_500().err(json!(err.to_string()))
    }
}
#[allow(non_camel_case_types)]
pub enum Code {
    OK,
    NOT_FOUND,
    BAD_REQUEST,
    INTERNAL_SERVER_ERROR,
}
impl Code {
    pub fn as_u16(&self) -> u16 {
        match self {
            Code::OK => 0,
            Code::NOT_FOUND => 404,
            Code::BAD_REQUEST => 400,
            Code::INTERNAL_SERVER_ERROR => 500,
        }
    }
    pub fn msg(&self) -> &'static str {
        match self {
            Code::OK => "操作成功",
            Code::NOT_FOUND => "访问的内容不存在",
            Code::BAD_REQUEST => "您的请求无效，请检查后重试",
            Code::INTERNAL_SERVER_ERROR => "当前服务不可用，请稍后再试",
        }
    }
}

pub type API = Result<APIResponse, APIError>;

#[macro_export]
macro_rules! OK {
    ($data:tt) => {
        Ok(crate::types::api::APIResponse::new().data(serde_json::json!($data)))
    };
}
