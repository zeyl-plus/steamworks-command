use serde::Serialize;

// 数据返回类型
#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    pub(crate) code: i32,
    pub(crate) result: T,
    pub(crate) message: String,
}

impl<T> DataResponse<T> {
    pub fn new<S: Into<String>>(code: i32, result: T, message: S) -> Self {
        DataResponse {
            code,
            result,
            message: message.into(),
        }
    }

    // 成功
    pub fn success(result: T) -> Self {
        DataResponse {
            code: 0,
            result,
            message: "成功".into(),
        }
    }
    // 失败
    pub fn error<S: Into<String>>(result: T, message: S) -> Self {
        DataResponse {
            code: -1,
            result,
            message: message.into(),
        }
    }
}
