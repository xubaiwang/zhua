use scraper::ElementRef;

use crate::error::ErrorKind;

/// 提取元素的 `text`
pub fn text(el: ElementRef) -> Result<String, ErrorKind>
where
{
    Ok(el.text().collect())
}

/// 提取元素的 `html`
pub fn html(el: ElementRef) -> Result<String, ErrorKind> {
    Ok(el.html())
}

/// 提取元素的 `html`
pub fn inner_html(el: ElementRef) -> Result<String, ErrorKind>
where
{
    Ok(el.inner_html())
}

/// 提取元素的 `attr`.
pub fn attr(el: ElementRef, attr: &str) -> Result<String, ErrorKind> {
    el.attr(attr)
        .map(ToString::to_string)
        .ok_or_else(|| ErrorKind::AttributeNotFound(attr.to_string()))
}
