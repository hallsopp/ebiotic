use super::{AvailableReturnFormats, DataReturnFormats, EBI_DBFETCH_ENDPOINT};
use crate::core::{self, EbioticClient, EbioticHttpClient, Service};
use crate::errors::EbioticError;
use bio::io::fasta::Record;
use std::fmt::{Display, Formatter};

pub mod dbfetchdbs;

/// The `Dbfetch` struct is used to specify the parameters for the `Dbfetch` service.
#[derive(Debug, Clone)]
pub struct Dbfetch {
    pub(crate) client: EbioticClient,
    db: dbfetchdbs::DbfetchDbs,
    return_format: DataReturnFormats,
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

/// The `DbfetchStyle` enum is used to specify the style of the return data from the `Dbfetch` service.
pub enum DbfetchStyle {
    Raw,
    Html,
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
            client: EbioticClient::default(),
            db: dbfetchdbs::DbfetchDbs::EnaSequence,
            return_format: DataReturnFormats::Fasta,
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
        client: EbioticClient,
        db: dbfetchdbs::DbfetchDbs,
        return_format: DataReturnFormats,
        style: DbfetchStyle,
    ) -> Dbfetch {
        Dbfetch {
            client,
            db,
            return_format,
            style,
        }
    }

    pub fn set_db(&mut self, db: dbfetchdbs::DbfetchDbs) {
        self.db = db;
    }

    pub fn set_return_format(&mut self, format: DataReturnFormats) {
        self.return_format = format;
    }

    pub fn set_style(&mut self, style: DbfetchStyle) {
        self.style = style;
    }

    pub fn db(&self) -> &dbfetchdbs::DbfetchDbs {
        &self.db
    }

    pub fn format(&self) -> &DataReturnFormats {
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
        if !self
            .db
            .available_return_formats()
            .iter()
            .any(|x| x == &self.return_format)
        {
            return Err(EbioticError::ReturnFormatNotAvailable(
                self.return_format.to_string(),
                self.db.to_string(),
            ));
        }

        log::info!("Submitting DBfetch request");

        let res = self
            .client
            .get(&format!(
                "{}?db={}&format={}&style={}&id={}",
                EBI_DBFETCH_ENDPOINT, self.db, self.return_format, self.style, input
            ))
            .await?;

        Ok(DbfetchResult::new(res))
    }
}
