use scraper::Html;
use zhua::{Extract, Extractable, attr, text};

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
#[derive(Debug, PartialEq, Extract)]
pub struct Author {
    #[zhua(".name", text)]
    pub name: String,
    #[zhua(".affiliation", text)]
    pub affiliation: Option<String>,
    #[zhua(".doi", text)]
    pub doi: String,
}
