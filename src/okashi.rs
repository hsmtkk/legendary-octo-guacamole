use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResponseSchema {
    item: Vec<JsonOkashi>,
}

#[derive(Serialize, Deserialize)]
struct JsonOkashi {
    id: String,
    price: String,
}

impl JsonOkashi {
    fn new(
        id: &str,
        price: &str,
    ) -> JsonOkashi {
        JsonOkashi {
            id:id.to_string(),
            price:price.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Okashi {
    id: u32,
    price: u32,
}

impl Okashi {
    fn new(id:u32, price:u32) -> Okashi {
        Okashi{id, price}
    }
}

pub trait OkashiSearcher {
    fn search_okashi(&self) -> Result<()>;
}

struct SearcherImpl {}

impl SearcherImpl {
    fn new() -> SearcherImpl {
        SearcherImpl {}
    }
}

impl OkashiSearcher for SearcherImpl {
    fn search_okashi(&self) -> Result<()> {
        unimplemented!()
    }
}

trait Parser {
    fn parse_json(&self, json:&str) -> Result<Vec<Okashi>>;
}

struct ParserImpl {}

fn new_parser() -> impl Parser {
    ParserImpl::new()
}

impl ParserImpl {
    fn new() -> ParserImpl {
        ParserImpl {}
    }
}

impl Parser for ParserImpl {
    fn parse_json(&self, json:&str) -> Result<Vec<Okashi>> {
        let rs: ResponseSchema = serde_json::from_str(json).with_context(|| "failed to parse JSON")?;
        let mut results = Vec::new();
        for jsonOkashi in rs.item {
            let sid = jsonOkashi.id;
            let sprice = jsonOkashi.price;
            let id = sid.parse::<u32>()?;
            let price = sprice.parse::<u32>()?;
            results.push(Okashi::new(id, price));
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::Okashi;
    use super::Parser;
    #[test]
    fn test_parse() {
        let mut f = File::open("./src/example.json").unwrap();
        let mut json = String::new();
        f.read_to_string(&mut json).unwrap();
        let want = Okashi::new(9789, 198);
        let parser = super::new_parser();
        let items = parser.parse_json(&json).unwrap();
        assert_eq!(5, items.len());
        assert_eq!(want, items[0]);
    }
}
