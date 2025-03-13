use scraper::{ElementRef, Selector, selectable::Selectable};

use crate::error::ErrorKind;

// MARK: trait `FromSelect` 定義

/// `FromSelect` 代表能從 `Select` 中提取信息。
///
/// 相比於 `Extract`, `FromSelect` 不包含 `selector`, `attr` 等元信息，
/// 而是由函數 `F` 提供。
///
/// 函數稱爲值函數 value function.
pub trait FromSelect<T>: Sized {
    fn from_select<'a, S, F>(select: S, f: F) -> Result<Self, ErrorKind>
    where
        S: Iterator<Item = ElementRef<'a>>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>;

    /// 直接從 selector 導出
    fn from_selectable<'a, S, F>(selectable: S, selector: &str, f: F) -> Result<Self, ErrorKind>
    where
        S: Selectable<'a>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>,
    {
        let selector =
            Selector::parse(selector).map_err(|err| ErrorKind::InvalidSelector(err.to_string()))?;
        let select = selectable.select(&selector);
        Self::from_select(select, f)
    }
}

// MARK: 單個 `FromSelect` 實現 (select_one)

/// 爲 `T` 類型本身實現。
/// 如果錯誤會直接報錯。
impl<T> FromSelect<T> for T {
    fn from_select<'a, S, F>(mut select: S, f: F) -> Result<Self, ErrorKind>
    where
        S: Iterator<Item = ElementRef<'a>>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>,
    {
        // 根據 select 中是否由元素
        match select.next() {
            None => Err(ErrorKind::ElementNotFound),
            Some(el) => f(el),
        }
    }
}

/// 爲 `Option<T>` 實現。
/// 如果報錯會處理爲 `None` 而非直接返回錯誤。
/// 如果需要保留錯誤信息請看 Result.
impl<T> FromSelect<T> for Option<T> {
    fn from_select<'a, S, F>(mut select: S, f: F) -> Result<Self, ErrorKind>
    where
        S: Iterator<Item = ElementRef<'a>>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>,
    {
        match select.next() {
            None => Ok(None),
            Some(el) => Ok(f(el).ok()),
        }
    }
}

/// 保留錯誤信息的
impl<T> FromSelect<T> for Result<T, ErrorKind> {
    fn from_select<'a, S, F>(mut select: S, f: F) -> Result<Self, ErrorKind>
    where
        S: Iterator<Item = ElementRef<'a>>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>,
    {
        match select.next() {
            None => Ok(Err(ErrorKind::ElementNotFound)),
            Some(el) => Ok(f(el)),
        }
    }
}

// MARK: select_many 實現

/// 爲 `Vec<T>` 實現
impl<T> FromSelect<T> for Vec<T> {
    fn from_select<'a, S, F>(select: S, f: F) -> Result<Self, ErrorKind>
    where
        S: Iterator<Item = ElementRef<'a>>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>,
    {
        select.map(|el| f(el)).collect()
    }
}

/// `Vec<Option<T>>`
impl<T> FromSelect<T> for Vec<Option<T>> {
    fn from_select<'a, S, F>(select: S, f: F) -> Result<Self, ErrorKind>
    where
        S: Iterator<Item = ElementRef<'a>>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>,
    {
        select.map(|el| Ok(f(el).ok())).collect()
    }
}

/// `Vec<Result<T>>`
impl<T> FromSelect<T> for Vec<Result<T, ErrorKind>> {
    fn from_select<'a, S, F>(select: S, f: F) -> Result<Self, ErrorKind>
    where
        S: Iterator<Item = ElementRef<'a>>,
        F: Fn(ElementRef<'a>) -> Result<T, ErrorKind>,
    {
        select.map(|el| Ok(f(el))).collect()
    }
}
