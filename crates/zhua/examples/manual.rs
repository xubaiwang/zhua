use scraper::Html;
use zhua::{
    extract::{Extract, Extractable},
    from::FromSelect,
    utils::{attr, text},
};

const HTML: &str = r#"
    <p class="name">
        foo
    </p>
    <p class="doi" href="https://example.org">
        doi
    </p>
"#;

fn main() {
    let doc = Html::parse_fragment(&HTML);
    let author: Author = doc.extract().unwrap();
    dbg!(author);
}

/// 「作者信息」數據結構。
#[derive(Debug, PartialEq)]
pub struct Author {
    pub name: String,
    pub affiliation: Option<String>,
    pub doi: String,
}

/// 定義提取方式
impl Extract for Author {
    fn extract_from<'a, T>(able: T) -> Result<Self, zhua::error::Error>
    where
        T: zhua::extract::Extractable<'a>,
    {
        let name = <String>::from_selectable(able, ".name", text)
            .map(|x| x.trim().to_string())
            .map_err(|err| err.with_field("name"))?;
        let affiliation = <Option<String>>::from_selectable(able, ".affiliation", text)
            .map_err(|err| err.with_field("affiliation"))?;
        let doi = String::from_selectable(able, ".doi", |el| attr(el, "href"))
            .map_err(|err| err.with_field("doi"))?;

        Ok(Self {
            name,
            affiliation,
            doi,
        })
    }
}
