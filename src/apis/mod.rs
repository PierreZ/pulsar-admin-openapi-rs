use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod bookies_api;
pub use self::bookies_api::{ BookiesApi, BookiesApiClient };
mod broker_stats_api;
pub use self::broker_stats_api::{ BrokerStatsApi, BrokerStatsApiClient };
mod brokers_api;
pub use self::brokers_api::{ BrokersApi, BrokersApiClient };
mod clusters_api;
pub use self::clusters_api::{ ClustersApi, ClustersApiClient };
mod namespaces_api;
pub use self::namespaces_api::{ NamespacesApi, NamespacesApiClient };
mod non_persistent_topic_api;
pub use self::non_persistent_topic_api::{ NonPersistentTopicApi, NonPersistentTopicApiClient };
mod persistent_topic_api;
pub use self::persistent_topic_api::{ PersistentTopicApi, PersistentTopicApiClient };
mod resource_quotas_api;
pub use self::resource_quotas_api::{ ResourceQuotasApi, ResourceQuotasApiClient };
mod schemas_api;
pub use self::schemas_api::{ SchemasApi, SchemasApiClient };
mod tenants_api;
pub use self::tenants_api::{ TenantsApi, TenantsApiClient };

pub mod configuration;
pub mod client;
