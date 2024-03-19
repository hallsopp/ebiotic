use crate::errors::EbioticError;
use std::collections::HashMap;
use std::fmt::Display;

pub struct EbiSearchQuery {
    query: Vec<QueryCommand>,
    filter: Option<HashMap<String, String>>,
    size: Option<u16>,
    start: Option<u32>,
    fields: Option<Vec<String>>,
    sort: Option<HashMap<String, SortOrder>>,
}

pub enum SortOrder {
    Ascending,
    Descending,
}

pub enum QueryCommand {
    Query(String),
    Xref,
    Entry,
    AutoComplete,
    TopTerms,
    SeqToolResults,
    Download,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => write!(f, "ascending"),
            SortOrder::Descending => write!(f, "descending"),
        }
    }
}

impl Display for QueryCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryCommand::Query(query) => write!(f, "?query={}", query),
            QueryCommand::Xref => write!(f, "xref"),
            QueryCommand::Entry => write!(f, "entry"),
            QueryCommand::AutoComplete => write!(f, "autoComplete"),
            QueryCommand::TopTerms => write!(f, "topTerms"),
            QueryCommand::SeqToolResults => write!(f, "seqToolResults"),
            QueryCommand::Download => write!(f, "download"),
        }
    }
}

impl EbiSearchQuery {
    // Laying the groundwork here for runtime checks on queries.
    // Some fields are required, some are optional, and some are mutually exclusive.
    // This will be a good place to implement those checks.

    pub fn new(query: Vec<QueryCommand>) -> Result<EbiSearchQuery, EbioticError> {
        Ok(EbiSearchQuery {
            query: query,
            filter: None,
            size: None,
            start: None,
            fields: None,
            sort: None,
        })
    }

    pub fn build(&self, query: String) -> Result<String, EbioticError> {
        if &self.query.is_empty() {
            return Err(EbioticError::EmptyEbiSearchQuery);
        }

        let mut url = String::new();

        if &self.query.

        if let Some(filter) = &self.filter {
            url.push_str("&filter=");
            for (key, value) in filter {
                url.push_str(&format!("{}:{}", key, value));
            }
        }

        if let Some(size) = &self.size {
            url.push_str(&format!("&size={}", size));
        }

        if let Some(start) = &self.start {
            url.push_str(&format!("&start={}", start));
        }

        if let Some(fields) = &self.fields {
            url.push_str("&fields=");
            for field in fields {
                url.push_str(&format!("{},", field));
            }
        }

        if let Some(sort) = &self.sort {
            url.push_str("&sort=");
            for (key, value) in sort {
                url.push_str(&format!("{}:{}", key, value));
            }
        }

        Ok(url)
    }

    pub fn set_filter(&mut self, filter: HashMap<String, String>) {
        self.filter = Some(filter);
    }

    pub fn set_size(&mut self, size: u16) {
        self.size = Some(size);
    }

    pub fn set_start(&mut self, start: u32) {
        self.start = Some(start);
    }

    pub fn set_fields(&mut self, fields: Vec<String>) {
        self.fields = Some(fields);
    }

    pub fn set_sort(&mut self, sort: HashMap<String, SortOrder>) {
        self.sort = Some(sort);
    }

    pub fn query(&self) -> &Vec<QueryCommand> {
        &self.query
    }

    pub fn filter(&self) -> &Option<HashMap<String, String>> {
        &self.filter
    }

    pub fn size(&self) -> &Option<u16> {
        &self.size
    }

    pub fn start(&self) -> &Option<u32> {
        &self.start
    }

    pub fn fields(&self) -> &Option<Vec<String>> {
        &self.fields
    }

    pub fn sort(&self) -> &Option<HashMap<String, SortOrder>> {
        &self.sort
    }
}
