use thiserror::Error;

#[derive(Debug, Error)]
#[error("{kind} in field `{field}`")]
pub struct Error {
    field: String,
    kind: ErrorKind,
}

/// 這個錯誤類型不包含上下文信息，因此可以直接從 `ElementRef` 獲得。
#[derive(Debug, Error)]
pub enum ErrorKind {
    /// 選擇器解析錯誤。
    #[error("invalid selector provided")]
    InvalidSelector(String),
    /// 未找到元素。
    #[error("no element match the selector")]
    ElementNotFound,
    /// 未找到屬性
    #[error("the element does not contain attribute `{0}`")]
    AttributeNotFound(String),
    /// 類型轉換錯誤
    #[error("custom error")]
    Custom(#[source] Box<dyn std::error::Error>),
}

impl ErrorKind {
    ///
    pub fn custom<E>(err: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        Self::Custom(Box::new(err))
    }

    pub fn with_field(self, field: &str) -> Error {
        Error {
            field: field.to_string(),
            kind: self,
        }
    }
}
