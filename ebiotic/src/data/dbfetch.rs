use bio::io::fasta::Record;
use reqwest::Client;

use std::fmt::{Display, Formatter};

use super::EBI_DBFETCH_ENDPOINT;
use crate::core::{self, Service};
use crate::errors::EbioticError;

pub mod dbfetchdbs;

/// The `Dbfetch` struct is used to specify the parameters for the `Dbfetch` service.
pub struct Dbfetch {
    db: dbfetchdbs::DbfetchDbs,
    return_format: DbfetchReturnFormat,
    style: DbfetchStyle,
}

/// The `DbfetchIds` struct is used to specify the IDs to be fetched from the `Dbfetch` service.
pub struct DbfetchIds {
    ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DbfetchResult {
    data: String,
}

/// The `DbfetchReturnFormat` enum is used to specify the return format of the `Dbfetch` service. This is dependent on the type of data available from the database.
pub enum DbfetchReturnFormat {
    Fasta,
    Json,
}

/// The `DbfetchStyle` enum is used to specify the style of the return data from the `Dbfetch` service.
pub enum DbfetchStyle {
    Raw,
    Html,
}

impl Display for DbfetchReturnFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbfetchReturnFormat::Fasta => write!(f, "Fasta"),
            DbfetchReturnFormat::Json => write!(f, "Json"),
        }
    }
}

impl Display for DbfetchIds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ids = String::new();
        for id in &self.ids {
            ids.push_str(id);
            ids.push(',');
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

impl Default for Dbfetch {
    fn default() -> Self {
        Dbfetch {
            db: dbfetchdbs::DbfetchDbs::EnaSequence,
            return_format: DbfetchReturnFormat::Fasta,
            style: DbfetchStyle::Raw,
        }
    }
}

impl DbfetchResult {
    fn new(data: String) -> DbfetchResult {
        DbfetchResult { data }
    }

    /// Convert the results of a `Dbfetch` service into a `Vec<Record>`.
    pub fn into_records(self) -> Result<Vec<Record>, EbioticError> {
        core::parse_fa_from_bufread(&self.data)
    }

    /// Get the raw data from the `Dbfetch` service. This is useful if you want to handle the data yourself.
    pub fn data(self) -> String {
        self.data
    }
}

impl DbfetchIds {
    /// Create a new `DbfetchIds` object with a list of IDs.
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
    pub fn new(
        db: dbfetchdbs::DbfetchDbs,
        return_format: DbfetchReturnFormat,
        style: DbfetchStyle,
    ) -> Dbfetch {
        Dbfetch {
            db,
            return_format,
            style,
        }
    }

    pub fn set_db(&mut self, db: dbfetchdbs::DbfetchDbs) {
        self.db = db;
    }

    pub fn set_return_format(&mut self, format: DbfetchReturnFormat) {
        self.return_format = format;
    }

    pub fn set_style(&mut self, style: DbfetchStyle) {
        self.style = style;
    }

    pub fn db(&self) -> &dbfetchdbs::DbfetchDbs {
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
    type ResultType = DbfetchResult;
    type InputType = DbfetchIds;

    /// Run the `Dbfetch` service with a list of IDs.
    async fn run(&self, input: Self::InputType) -> Result<Self::ResultType, EbioticError> {
        let client = Client::new();
        let res = client
            .get(&format!(
                "{}?db={}&format={}&style={}&id={}",
                EBI_DBFETCH_ENDPOINT, self.db, self.return_format, self.style, input
            ))
            .send()
            .await?;

        Ok(DbfetchResult::new(res.text().await?))
    }
}