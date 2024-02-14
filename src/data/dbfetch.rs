use bio::io::fasta::Record;
use reqwest::Client;

use std::fmt::{Display, Formatter};

use super::EBI_DBFETCH_ENDPOINT;
use crate::core::{self, parse_fa_from_bufread, Service};
use crate::errors::EbioticError;

pub struct Dbfetch {
    db: DbfetchDbs,
    return_format: DbfetchReturnFormat,
    style: DbfetchStyle,
}

pub struct DbfetchIds {
    ids: Vec<String>,
}

pub enum DbfetchReturnFormat {
    Fasta,
    Json,
}

pub enum DbfetchStyle {
    Raw,
    Html,
}

pub enum DbfetchDbs {
    EnaSequence,
}

impl Display for DbfetchReturnFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbfetchReturnFormat::Fasta => write!(f, "Fasta"),
            DbfetchReturnFormat::Json => write!(f, "Json"),
        }
    }
}

impl Display for DbfetchDbs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbfetchDbs::EnaSequence => write!(f, "ena_sequence"),
        }
    }
}

impl Display for DbfetchIds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ids = String::new();
        for id in &self.ids {
            ids.push_str(id);
            ids.push_str(",");
        }
        write!(f, "{}", ids)
    }
}

impl Display for DbfetchStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbfetchStyle::Raw => write!(f, "raw"),
            DbfetchStyle::Html => write!(f, "html"),
        }
    }
}

impl DbfetchIds {
    pub fn new(ids: Vec<String>) -> DbfetchIds {
        DbfetchIds { ids }
    }

    pub fn set_ids(&mut self, ids: Vec<String>) {
        self.ids = ids;
    }

    pub fn ids(&self) -> &Vec<String> {
        &self.ids
    }
}

impl Dbfetch {
    pub fn new(db: DbfetchDbs, return_format: DbfetchReturnFormat, style: DbfetchStyle) -> Dbfetch {
        Dbfetch {
            db,
            return_format,
            style,
        }
    }

    pub fn default_fasta(db: DbfetchDbs) -> Dbfetch {
        Dbfetch {
            db,
            return_format: DbfetchReturnFormat::Fasta,
            style: DbfetchStyle::Raw,
        }
    }

    pub fn set_db(&mut self, db: DbfetchDbs) {
        self.db = db;
    }

    pub fn set_return_format(&mut self, format: DbfetchReturnFormat) {
        self.return_format = format;
    }

    pub fn set_style(&mut self, style: DbfetchStyle) {
        self.style = style;
    }

    pub fn db(&self) -> &DbfetchDbs {
        &self.db
    }

    pub fn format(&self) -> &DbfetchReturnFormat {
        &self.return_format
    }

    pub fn style(&self) -> &DbfetchStyle {
        &self.style
    }
}

impl Service for Dbfetch {
    type ResultType = String;
    type InputType = DbfetchIds;

    async fn run(&self, input: Self::InputType) -> Result<Self::ResultType, EbioticError> {
        let client = Client::new();
        let res = client
            .get(&format!(
                "{}?db={}&format={}&style={}&id={}",
                EBI_DBFETCH_ENDPOINT, self.db, self.return_format, self.style, input
            ))
            .send()
            .await?;

        return Ok(res.text().await?);
    }
}

impl Dbfetch {
    pub fn into_records(&self, response: String) -> Result<Vec<Record>, EbioticError> {
        parse_fa_from_bufread(&response)
    }
}
