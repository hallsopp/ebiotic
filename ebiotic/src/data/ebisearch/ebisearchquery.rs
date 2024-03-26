use super::{ebisearchdomains::EbiSearchDomains, AccessionIds};
use crate::errors::EbioticError;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct EbiSearchQuery {
    query: Vec<QueryCommand<'_>>,
    filters: Option<EbiSearchFilters>,
}

#[derive(Debug, Clone)]
pub struct EbiSearchFilters {
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

pub enum QueryCommand<'a> {
    QueryStr(String),
    Xref(Option<EbiSearchDomains>),
    Entry(Option<&'a AccessionIds>),
    AutoComplete,
    TopTerms,
    SeqToolResults,
    Download,
    MoreLikeThis,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => write!(f, "ascending"),
            SortOrder::Descending => write!(f, "descending"),
        }
    }
}

impl Display for QueryCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryCommand::QueryStr(query) => write!(f, "?query={}", query),
            QueryCommand::Xref(domain) => {
                if let Some(domain) = domain {
                    write!(f, "xref/{}", domain)
                } else {
                    write!(f, "xref")
                }
            }
            QueryCommand::Entry(ids) => {
                if let Some(ids) = ids {
                    write!(f, "entry/{}", ids)
                } else {
                    write!(f, "entry")
                }
            }
            QueryCommand::AutoComplete => write!(f, "autoComplete"),
            QueryCommand::TopTerms => write!(f, "topTerms"),
            QueryCommand::SeqToolResults => write!(f, "seqToolResults"),
            QueryCommand::Download => write!(f, "download"),
            QueryCommand::MoreLikeThis => write!(f, "moreLikeThis"),
        }
    }
}

impl EbiSearchQuery {
    // Laying the groundwork here for runtime checks on queries.
    // Some fields are required, some are optional, and some are mutually exclusive.
    // This will be a good place to implement those checks.

    pub fn new(query: Vec<QueryCommand>) -> Result<EbiSearchQuery, EbioticError> {
        if query.len() > 4 {
            return Err(EbioticError::TooManyQueryCommands);
        } else if query.is_empty() {
            return Err(EbioticError::EmptyEbiSearchQuery);
        }

        Ok(EbiSearchQuery {
            query: query,
            filters: None,
        })
    }

    pub fn build(&self) -> Result<String, EbioticError> {
        let mut url = String::new();

        if let Some(filters) = &self.filters {
            if let Some(f) = &filters.filter {
                url.push_str("&filter=");
                for (key, value) in f {
                    url.push_str(&format!("{}:{}", key, value));
                }
            }

            if let Some(size) = &filters.size {
                url.push_str(&format!("&size={}", size));
            }

            if let Some(start) = &filters.start {
                url.push_str(&format!("&start={}", start));
            }

            if let Some(fields) = &filters.fields {
                url.push_str("&fields=");
                for field in fields {
                    url.push_str(&format!("{},", field));
                }
            }

            if let Some(sort) = &filters.sort {
                url.push_str("&sort=");
                for (key, value) in sort {
                    url.push_str(&format!("{}:{}", key, value));
                }
            }
        }

        Ok(url)
    }
}

impl EbiSearchFilters {
    pub fn new() -> EbiSearchFilters {
        EbiSearchFilters {
            filter: None,
            size: None,
            start: None,
            fields: None,
            sort: None,
        }
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
