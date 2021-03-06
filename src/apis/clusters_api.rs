/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct ClustersApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ClustersApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ClustersApiClient<C> {
        ClustersApiClient {
            configuration: configuration,
        }
    }
}

pub trait ClustersApi {
    fn create_cluster(&self, cluster: &str, body: crate::models::ClusterData) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_cluster(&self, cluster: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_failure_domain(&self, cluster: &str, domain_name: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_namespace_isolation_policy(&self, cluster: &str, policy_name: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_broker_with_namespace_isolation_policy(&self, cluster: &str, broker: &str) -> Box<Future<Item = crate::models::BrokerNamespaceIsolationData, Error = Error<serde_json::Value>>>;
    fn get_brokers_with_namespace_isolation_policy(&self, cluster: &str) -> Box<Future<Item = Vec<crate::models::BrokerNamespaceIsolationData>, Error = Error<serde_json::Value>>>;
    fn get_cluster(&self, cluster: &str) -> Box<Future<Item = crate::models::ClusterData, Error = Error<serde_json::Value>>>;
    fn get_clusters(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn get_domain(&self, cluster: &str, domain_name: &str) -> Box<Future<Item = crate::models::FailureDomain, Error = Error<serde_json::Value>>>;
    fn get_failure_domains(&self, cluster: &str) -> Box<Future<Item = ::std::collections::HashMap<String, crate::models::FailureDomain>, Error = Error<serde_json::Value>>>;
    fn get_namespace_isolation_policies(&self, cluster: &str) -> Box<Future<Item = ::std::collections::HashMap<String, crate::models::NamespaceIsolationData>, Error = Error<serde_json::Value>>>;
    fn get_namespace_isolation_policy(&self, cluster: &str, policy_name: &str) -> Box<Future<Item = crate::models::NamespaceIsolationData, Error = Error<serde_json::Value>>>;
    fn get_peer_cluster(&self, cluster: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn set_failure_domain(&self, cluster: &str, domain_name: &str, body: crate::models::FailureDomain) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn set_namespace_isolation_policy(&self, cluster: &str, policy_name: &str, body: crate::models::NamespaceIsolationData) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn set_peer_cluster_names(&self, cluster: &str, body: Vec<String>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_cluster(&self, cluster: &str, body: crate::models::ClusterData) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>ClustersApi for ClustersApiClient<C> {
    fn create_cluster(&self, cluster: &str, body: crate::models::ClusterData) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/clusters/{cluster}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_cluster(&self, cluster: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/clusters/{cluster}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_failure_domain(&self, cluster: &str, domain_name: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/clusters/{cluster}/failureDomains/{domainName}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_path_param("domainName".to_string(), domain_name.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_namespace_isolation_policy(&self, cluster: &str, policy_name: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/clusters/{cluster}/namespaceIsolationPolicies/{policyName}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_path_param("policyName".to_string(), policy_name.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn get_broker_with_namespace_isolation_policy(&self, cluster: &str, broker: &str) -> Box<Future<Item = crate::models::BrokerNamespaceIsolationData, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}/namespaceIsolationPolicies/brokers/{broker}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_path_param("broker".to_string(), broker.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_brokers_with_namespace_isolation_policy(&self, cluster: &str) -> Box<Future<Item = Vec<crate::models::BrokerNamespaceIsolationData>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}/namespaceIsolationPolicies/brokers".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_cluster(&self, cluster: &str) -> Box<Future<Item = crate::models::ClusterData, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_clusters(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters".to_string())
            .execute(self.configuration.borrow())
    }

    fn get_domain(&self, cluster: &str, domain_name: &str) -> Box<Future<Item = crate::models::FailureDomain, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}/failureDomains/{domainName}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_path_param("domainName".to_string(), domain_name.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_failure_domains(&self, cluster: &str) -> Box<Future<Item = ::std::collections::HashMap<String, crate::models::FailureDomain>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}/failureDomains".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_namespace_isolation_policies(&self, cluster: &str) -> Box<Future<Item = ::std::collections::HashMap<String, crate::models::NamespaceIsolationData>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}/namespaceIsolationPolicies".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_namespace_isolation_policy(&self, cluster: &str, policy_name: &str) -> Box<Future<Item = crate::models::NamespaceIsolationData, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}/namespaceIsolationPolicies/{policyName}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_path_param("policyName".to_string(), policy_name.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_peer_cluster(&self, cluster: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/clusters/{cluster}/peers".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .execute(self.configuration.borrow())
    }

    fn set_failure_domain(&self, cluster: &str, domain_name: &str, body: crate::models::FailureDomain) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/clusters/{cluster}/failureDomains/{domainName}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_path_param("domainName".to_string(), domain_name.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn set_namespace_isolation_policy(&self, cluster: &str, policy_name: &str, body: crate::models::NamespaceIsolationData) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/clusters/{cluster}/namespaceIsolationPolicies/{policyName}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_path_param("policyName".to_string(), policy_name.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn set_peer_cluster_names(&self, cluster: &str, body: Vec<String>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/clusters/{cluster}/peers".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn update_cluster(&self, cluster: &str, body: crate::models::ClusterData) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/clusters/{cluster}".to_string())
            .with_path_param("cluster".to_string(), cluster.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

}
