use crate::core::PollableService;
use crate::errors::EbioticError;

pub trait EbioticHttpClient: Default + Send + Clone {
    async fn post_form(
        &self,
        endpoint: &str,
        body: &[(&str, &str)],
    ) -> Result<String, EbioticError>;
    async fn get(&self, endpoint: &str) -> Result<String, EbioticError>;
    async fn poll<F>(
        &self,
        endpoint: &str,
        post_body: Option<&[(&str, &str)]>,
        method_caller: &F,
    ) -> Result<String, EbioticError>
    where
        F: PollableService;
}
