use super::{ebisearchdomains::EbiSearchDomains, AccessionIds, DataReturnFormats};
use crate::core::EbioticResult;
use crate::errors::EbioticError;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub type EbiSearchFilters = Vec<EbiSearchFilter>;

/// The `EbiSearchQuery` struct is used to build a query to the EBI Search service.
#[derive(Debug, Clone)]
pub struct EbiSearchQuery {
    query: Vec<QueryCommand>,
    filters: Option<EbiSearchFilters>,
}

/// The `EbiSearchFilter` enum is used to specify the filters to be applied to the EBI Search query.
#[derive(Debug, Clone)]
pub enum EbiSearchFilter {
    Filter(HashMap<String, String>),
    Size(u32),
    Start(u32),
    Fields(Vec<String>),
    Sort(HashMap<String, SortOrder>),
    Format(DataReturnFormats),
}

/// The `SortOrder` enum is used to specify the sort order of the EBI Search filter under `EbiSearchFilter::Sort`.
#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// The `QueryCommand` enum is used to specify the arguments to the EBI Search query.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum QueryCommand {
    QueryStr(String),
    Xref(Option<EbiSearchDomains>),
    Entry(Option<AccessionIds>),
    AutoComplete(String),
    TopTerms(String),
    SeqToolResults(String, String),
    Download,
    MoreLikeThis(Option<EbiSearchDomains>),
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
            QueryCommand::QueryStr(query) => write!(f, "query={}", query),
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
            QueryCommand::AutoComplete(term) => write!(f, "autocomplete?term={}", term),
            QueryCommand::TopTerms(fieldid) => write!(f, "topterms/{}", fieldid),
            QueryCommand::SeqToolResults(toolid, jobid) => {
                write!(f, "seqtoolresults?toolid={}&jobid={}", toolid, jobid)
            }
            QueryCommand::Download => write!(f, "download"),
            QueryCommand::MoreLikeThis(domain) => {
                if let Some(domain) = domain {
                    write!(f, "morelikethis/{}", domain)
                } else {
                    write!(f, "morelikethis")
                }
            }
        }
    }
}

impl Display for EbiSearchFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EbiSearchFilter::Filter(filters) => {
                let filt = filters
                    .iter()
                    .map(|(key, value)| format!("{}:{}", key, value))
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "filters={}", filt)
            }
            EbiSearchFilter::Size(size) => write!(f, "size={}", size),
            EbiSearchFilter::Start(start) => write!(f, "start={}", start),
            EbiSearchFilter::Fields(fields) => {
                write!(f, "fields={}", fields.join(","))
            }
            EbiSearchFilter::Sort(sort) => {
                let sorts = sort
                    .iter()
                    .map(|(key, value)| format!("{}:{}", key, value))
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "sort={}", sorts)
            }
            EbiSearchFilter::Format(format) => write!(f, "format={}", format),
        }
    }
}

impl EbiSearchQuery {
    // Laying the groundwork here for runtime checks on queries.
    // Some fields are required, some are optional, and some are mutually exclusive.
    // This will be a good place to implement those checks.

    pub fn new(
        query: Vec<QueryCommand>,
        filters: Option<EbiSearchFilters>,
    ) -> EbioticResult<EbiSearchQuery> {
        if query.len() > 4 {
            return Err(EbioticError::TooManyQueryCommands);
        } else if query.is_empty() {
            return Err(EbioticError::EmptyEbiSearchQuery);
        }

        // TODO: We can check for mutually exclusive and unique filters here.

        Ok(EbiSearchQuery { query, filters })
    }

    /// Add a command to the query.
    pub fn add_command(&mut self, command: QueryCommand) {
        self.query.push(command);
    }

    /// Add a filter to the query.
    pub fn add_filter(&mut self, filter: EbiSearchFilter) {
        if let Some(filters) = &mut self.filters {
            filters.push(filter);
        } else {
            self.filters = Some(vec![filter]);
        }
    }

    /// Compile the query into a URL - checks the query for correctness.
    pub fn build(&self) -> EbioticResult<String> {
        let mut url = String::new();

        for (i, command) in self.query.iter().enumerate() {
            match command {
                QueryCommand::QueryStr(_) => {
                    if i != self.query.len() - 1 {
                        return Err(EbioticError::QueryStrOrTermNotFirst);
                    }
                    url.push_str(&format!("{}", command));
                }
                _ => {
                    url.push_str(&format!("{}/", command));
                }
            }
        }

        if let Some(filters) = &self.filters {
            let filter_str = filters
                .iter()
                .map(|filter| format!("{}", filter))
                .collect::<Vec<String>>()
                .join("&");
            url.push_str(&filter_str);
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn build_query_with_single_command() {
        let mut query = Vec::new();
        query.push(QueryCommand::QueryStr("test".to_string()));
        let search_query = EbiSearchQuery::new(query, None).unwrap();
        let result = search_query.build().unwrap();
        assert_eq!(result, "?query=test");
    }

    #[test]
    #[should_panic]
    fn build_query_with_multiple_commands() {
        let mut query = Vec::new();
        query.push(QueryCommand::QueryStr("test".to_string()));
        query.push(QueryCommand::AutoComplete("lol".to_string()));
        let search_query = EbiSearchQuery::new(query, None).unwrap();
        let result = search_query.build().unwrap();
        assert_eq!(result, "?query=testautocomplete?term=lol");
    }

    #[test]
    fn build_query_with_filters() {
        let mut query = Vec::new();
        query.push(QueryCommand::QueryStr("test".to_string()));
        let mut search_query = EbiSearchQuery::new(query, None).unwrap();
        search_query.add_filter(EbiSearchFilter::Size(10));
        let result = search_query.build().unwrap();
        assert_eq!(result, "?query=test&size=10");
    }

    #[test]
    fn build_query_with_sort_order() {
        let mut query = Vec::new();
        query.push(QueryCommand::QueryStr("test".to_string()));
        let mut search_query = EbiSearchQuery::new(query, None).unwrap();
        let mut sort = HashMap::new();
        sort.insert("field".to_string(), SortOrder::Ascending);
        search_query.add_filter(EbiSearchFilter::Sort(sort));
        let result = search_query.build().unwrap();
        assert_eq!(result, "?query=test&sort=field:ascending");
    }

    #[test]
    fn build_query_with_empty_query() {
        let query = Vec::new();
        let search_query = EbiSearchQuery::new(query, None);
        assert!(search_query.is_err());
    }

    #[test]
    fn build_query_with_too_many_commands() {
        let mut query = Vec::new();
        for _ in 0..5 {
            query.push(QueryCommand::QueryStr("test".to_string()));
        }
        let search_query = EbiSearchQuery::new(query, None);
        assert!(search_query.is_err());
    }

    #[test]
    fn check_cross_ref_search() {
        let mut query = Vec::new();
        let ids = AccessionIds::from(vec!["P12345".to_string(), "P1234567".to_string()]);
        query.push(QueryCommand::Entry(Some(ids)));
        query.push(QueryCommand::Xref(Some(EbiSearchDomains::Ena)));
        let search_query = EbiSearchQuery::new(query, None).unwrap().build().unwrap();
        assert_eq!(search_query, "entry/P12345,P1234567/xref/ena/");
    }
}
