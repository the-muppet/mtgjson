use async_trait::async_trait;
use pyo3::prelude::*;
use reqwest::Response;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use crate::prices::MtgjsonPrices;
use super::{AbstractProvider, BaseProvider, ProviderError, ProviderResult};

macro_rules! create_provider {
    ($name:ident, $class_name:expr, $class_id:expr) => {
        #[pyclass(name = $class_name)]
        pub struct $name {
            base: BaseProvider,
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new() -> PyResult<Self> {
                let headers = HashMap::new();
                let base = BaseProvider::new($class_id.to_string(), headers);
                Ok(Self { base })
            }
        }

        #[async_trait]
        impl AbstractProvider for $name {
            fn get_class_id(&self) -> &str {
                &self.base.class_id
            }
            
            fn get_class_name(&self) -> &str {
                $class_name
            }
            
            fn build_http_header(&self) -> HashMap<String, String> {
                HashMap::new()
            }
            
            async fn download(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Value> {
                self.base.download_json(url, params).await
            }
            
            async fn download_raw(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<String> {
                self.base.download_text(url, params).await
            }
            
            fn log_download(&self, response: &Response) {
                println!("Downloaded {} (Status: {})", response.url(), response.status());
            }
            
            fn generic_generate_today_price_dict(
                &self,
                _third_party_to_mtgjson: &HashMap<String, HashSet<String>>,
                _price_data_rows: &[Value],
                _card_platform_id_key: &str,
                _default_prices_object: &MtgjsonPrices,
                _foil_key: &str,
                _retail_key: Option<&str>,
                _retail_quantity_key: Option<&str>,
                _buy_key: Option<&str>,
                _buy_quantity_key: Option<&str>,
                _etched_key: Option<&str>,
                _etched_value: Option<&str>,
            ) -> HashMap<String, MtgjsonPrices> {
                HashMap::new()
            }
        }
    };
}

// Create all provider implementations
create_provider!(CardMarketProvider, "CardMarketProvider", "mkm");
create_provider!(EdhrecProvider, "EdhrecProvider", "edhrec");
create_provider!(GathererProvider, "GathererProvider", "gatherer");
create_provider!(GitHubBoostersProvider, "GitHubBoostersProvider", "gh_boosters");
create_provider!(GitHubCardSealedProductsProvider, "GitHubCardSealedProductsProvider", "gh_sealed_products");
create_provider!(GitHubDecksProvider, "GitHubDecksProvider", "gh_decks");
create_provider!(GitHubMTGSqliteProvider, "GitHubMTGSqliteProvider", "gh_sqlite");
create_provider!(GitHubSealedProvider, "GitHubSealedProvider", "gh_sealed");
create_provider!(MTGBanProvider, "MTGBanProvider", "mtgban");
create_provider!(MtgWikiProvider, "MtgWikiProvider", "mtgwiki");
create_provider!(MultiverseBridgeProvider, "MultiverseBridgeProvider", "mb");
create_provider!(WhatsInStandardProvider, "WhatsInStandardProvider", "standard");
create_provider!(WizardsProvider, "WizardsProvider", "wizards");

// Re-export all providers
pub use self::{
    CardMarketProvider, EdhrecProvider, GathererProvider,
    GitHubBoostersProvider, GitHubCardSealedProductsProvider, GitHubDecksProvider,
    GitHubMTGSqliteProvider, GitHubSealedProvider, MTGBanProvider,
    MtgWikiProvider, MultiverseBridgeProvider, WhatsInStandardProvider,
    WizardsProvider,
};