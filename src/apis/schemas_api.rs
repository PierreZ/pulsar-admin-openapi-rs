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

pub struct SchemasApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SchemasApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SchemasApiClient<C> {
        SchemasApiClient {
            configuration: configuration,
        }
    }
}

pub trait SchemasApi {
    fn delete_schema(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::DeleteSchemaResponse, Error = Error<serde_json::Value>>>;
    fn get_schema(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::GetSchemaResponse, Error = Error<serde_json::Value>>>;
    fn get_schema_0(&self, tenant: &str, namespace: &str, topic: &str, version: &str, authoritative: bool) -> Box<Future<Item = crate::models::GetSchemaResponse, Error = Error<serde_json::Value>>>;
    fn post_schema(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool, body: crate::models::PostSchemaPayload) -> Box<Future<Item = crate::models::PostSchemaResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>SchemasApi for SchemasApiClient<C> {
    fn delete_schema(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::DeleteSchemaResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/schemas/{tenant}/{namespace}/{topic}/schema".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_schema(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::GetSchemaResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/schemas/{tenant}/{namespace}/{topic}/schema".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_schema_0(&self, tenant: &str, namespace: &str, topic: &str, version: &str, authoritative: bool) -> Box<Future<Item = crate::models::GetSchemaResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/schemas/{tenant}/{namespace}/{topic}/schema/{version}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("version".to_string(), version.to_string())
            .execute(self.configuration.borrow())
    }

    fn post_schema(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool, body: crate::models::PostSchemaPayload) -> Box<Future<Item = crate::models::PostSchemaResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/schemas/{tenant}/{namespace}/{topic}/schema".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_body_param(body)
            .execute(self.configuration.borrow())
    }

}
