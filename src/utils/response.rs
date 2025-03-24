use serde::Serialize;

// 数据返回类型
#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    pub(crate) code: i32,
    pub(crate) result: T,
    pub(crate) message: String,
}
