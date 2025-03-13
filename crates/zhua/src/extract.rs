use scraper::selectable::Selectable;

use crate::error::Error;

// MARK: trait `Extract`

/// `Extract` 代表被提取的類型。
pub trait Extract: Sized {
    fn extract_from<'a, T>(extractable: T) -> Result<Self, Error>
    where
        T: Extractable<'a>;
}

// MARK: trait `Extractable`

/// `Extractable` 代表可以提取信息的內容，包括 `Html` 和 `ElementRef`.
///
/// `Extractable` 只提供默認方法，`required` 在 `Selectable` 裏已經實現。
pub trait Extractable<'a>: Selectable<'a> + Copy {
    fn extract<T: Extract>(self) -> Result<T, Error> {
        T::extract_from(self)
    }
}

/// 爲所有 `Selectable` 自動實現 `Extractable`.
impl<'a, T> Extractable<'a> for T where T: Selectable<'a> + Copy {}
