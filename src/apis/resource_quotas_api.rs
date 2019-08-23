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

pub struct ResourceQuotasApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ResourceQuotasApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ResourceQuotasApiClient<C> {
        ResourceQuotasApiClient {
            configuration: configuration,
        }
    }
}

pub trait ResourceQuotasApi {
    fn get_default_resource_quota(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn get_namespace_bundle_resource_quota(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = crate::models::ResourceQuota, Error = Error<serde_json::Value>>>;
    fn remove_namespace_bundle_resource_quota(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn set_default_resource_quota(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn set_namespace_bundle_resource_quota(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>ResourceQuotasApi for ResourceQuotasApiClient<C> {
    fn get_default_resource_quota(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/resource-quotas".to_string())
            .execute(self.configuration.borrow())
    }

    fn get_namespace_bundle_resource_quota(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = crate::models::ResourceQuota, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/resource-quotas/{tenant}/{namespace}/{bundle}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("bundle".to_string(), bundle.to_string())
            .execute(self.configuration.borrow())
    }

    fn remove_namespace_bundle_resource_quota(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/resource-quotas/{tenant}/{namespace}/{bundle}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("bundle".to_string(), bundle.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn set_default_resource_quota(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/resource-quotas".to_string())
            .execute(self.configuration.borrow())
    }

    fn set_namespace_bundle_resource_quota(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/resource-quotas/{tenant}/{namespace}/{bundle}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("bundle".to_string(), bundle.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

}
