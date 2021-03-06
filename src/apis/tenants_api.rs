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

pub struct TenantsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TenantsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TenantsApiClient<C> {
        TenantsApiClient {
            configuration: configuration,
        }
    }
}

pub trait TenantsApi {
    fn create_tenant(&self, tenant: &str, body: crate::models::TenantInfo) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_tenant(&self, tenant: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_tenant_admin(&self, tenant: &str) -> Box<Future<Item = crate::models::TenantInfo, Error = Error<serde_json::Value>>>;
    fn get_tenants(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn update_tenant(&self, tenant: &str, body: crate::models::TenantInfo) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>TenantsApi for TenantsApiClient<C> {
    fn create_tenant(&self, tenant: &str, body: crate::models::TenantInfo) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/tenants/{tenant}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_tenant(&self, tenant: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/tenants/{tenant}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn get_tenant_admin(&self, tenant: &str) -> Box<Future<Item = crate::models::TenantInfo, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/tenants/{tenant}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_tenants(&self, ) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/tenants".to_string())
            .execute(self.configuration.borrow())
    }

    fn update_tenant(&self, tenant: &str, body: crate::models::TenantInfo) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/tenants/{tenant}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

}
