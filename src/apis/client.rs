use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    bookies_api: Box<crate::apis::BookiesApi>,
    broker_stats_api: Box<crate::apis::BrokerStatsApi>,
    brokers_api: Box<crate::apis::BrokersApi>,
    clusters_api: Box<crate::apis::ClustersApi>,
    namespaces_api: Box<crate::apis::NamespacesApi>,
    non_persistent_topic_api: Box<crate::apis::NonPersistentTopicApi>,
    persistent_topic_api: Box<crate::apis::PersistentTopicApi>,
    resource_quotas_api: Box<crate::apis::ResourceQuotasApi>,
    schemas_api: Box<crate::apis::SchemasApi>,
    tenants_api: Box<crate::apis::TenantsApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            bookies_api: Box::new(crate::apis::BookiesApiClient::new(rc.clone())),
            broker_stats_api: Box::new(crate::apis::BrokerStatsApiClient::new(rc.clone())),
            brokers_api: Box::new(crate::apis::BrokersApiClient::new(rc.clone())),
            clusters_api: Box::new(crate::apis::ClustersApiClient::new(rc.clone())),
            namespaces_api: Box::new(crate::apis::NamespacesApiClient::new(rc.clone())),
            non_persistent_topic_api: Box::new(crate::apis::NonPersistentTopicApiClient::new(rc.clone())),
            persistent_topic_api: Box::new(crate::apis::PersistentTopicApiClient::new(rc.clone())),
            resource_quotas_api: Box::new(crate::apis::ResourceQuotasApiClient::new(rc.clone())),
            schemas_api: Box::new(crate::apis::SchemasApiClient::new(rc.clone())),
            tenants_api: Box::new(crate::apis::TenantsApiClient::new(rc.clone())),
        }
    }

    pub fn bookies_api(&self) -> &crate::apis::BookiesApi{
        self.bookies_api.as_ref()
    }

    pub fn broker_stats_api(&self) -> &crate::apis::BrokerStatsApi{
        self.broker_stats_api.as_ref()
    }

    pub fn brokers_api(&self) -> &crate::apis::BrokersApi{
        self.brokers_api.as_ref()
    }

    pub fn clusters_api(&self) -> &crate::apis::ClustersApi{
        self.clusters_api.as_ref()
    }

    pub fn namespaces_api(&self) -> &crate::apis::NamespacesApi{
        self.namespaces_api.as_ref()
    }

    pub fn non_persistent_topic_api(&self) -> &crate::apis::NonPersistentTopicApi{
        self.non_persistent_topic_api.as_ref()
    }

    pub fn persistent_topic_api(&self) -> &crate::apis::PersistentTopicApi{
        self.persistent_topic_api.as_ref()
    }

    pub fn resource_quotas_api(&self) -> &crate::apis::ResourceQuotasApi{
        self.resource_quotas_api.as_ref()
    }

    pub fn schemas_api(&self) -> &crate::apis::SchemasApi{
        self.schemas_api.as_ref()
    }

    pub fn tenants_api(&self) -> &crate::apis::TenantsApi{
        self.tenants_api.as_ref()
    }

}
