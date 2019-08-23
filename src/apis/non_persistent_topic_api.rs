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

pub struct NonPersistentTopicApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NonPersistentTopicApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NonPersistentTopicApiClient<C> {
        NonPersistentTopicApiClient {
            configuration: configuration,
        }
    }
}

pub trait NonPersistentTopicApi {
    fn compact(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn compaction_status(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::LongRunningProcessStatus, Error = Error<serde_json::Value>>>;
    fn create_non_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, body: i32) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_subscription(&self, tenant: &str, namespace: &str, topic: &str, subscription_name: &str, authoritative: bool, replicated: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, force: bool, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_subscription(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_topic(&self, tenant: &str, namespace: &str, topic: &str, force: bool, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn expire_messages_for_all_subscriptions(&self, tenant: &str, namespace: &str, topic: &str, expire_time_in_seconds: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn expire_topic_messages(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, expire_time_in_seconds: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_backlog(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::PersistentOfflineTopicStats, Error = Error<serde_json::Value>>>;
    fn get_internal_stats(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::PersistentTopicInternalStats, Error = Error<serde_json::Value>>>;
    fn get_last_message_id(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn get_list(&self, tenant: &str, namespace: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn get_list_from_bundle(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn get_managed_ledger_info(&self, tenant: &str, namespace: &str, topic: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_partitioned_metadata(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::PartitionedTopicMetadata, Error = Error<serde_json::Value>>>;
    fn get_partitioned_stats(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_partitioned_topic_list(&self, tenant: &str, namespace: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn get_permissions_on_topic(&self, tenant: &str, namespace: &str, topic: &str) -> Box<Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_stats(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::TopicStats, Error = Error<serde_json::Value>>>;
    fn get_subscriptions(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn grant_permissions_on_topic(&self, tenant: &str, namespace: &str, topic: &str, role: &str, body: Vec<String>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn offload_status(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::OffloadProcessStatus, Error = Error<serde_json::Value>>>;
    fn peek_nth_message(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, message_position: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn reset_cursor(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, timestamp: i64, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn reset_cursor_on_position(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, authoritative: bool, message_id: crate::models::MessageIdImpl) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn revoke_permissions_on_topic(&self, tenant: &str, namespace: &str, topic: &str, role: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn skip_all_messages(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn skip_messages(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, num_messages: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn terminate(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn trigger_offload(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn unload_topic(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, body: i32) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>NonPersistentTopicApi for NonPersistentTopicApiClient<C> {
    fn compact(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/non-persistent/{tenant}/{namespace}/{topic}/compaction".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn compaction_status(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::LongRunningProcessStatus, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/compaction".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn create_non_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/non-persistent/{tenant}/{namespace}/{topic}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn create_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, body: i32) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/non-persistent/{tenant}/{namespace}/{topic}/partitions".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn create_subscription(&self, tenant: &str, namespace: &str, topic: &str, subscription_name: &str, authoritative: bool, replicated: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subscriptionName}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_query_param("replicated".to_string(), replicated.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subscriptionName".to_string(), subscription_name.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, force: bool, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/non-persistent/{tenant}/{namespace}/{topic}/partitions".to_string())
            .with_query_param("force".to_string(), force.to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_subscription(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subName".to_string(), sub_name.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_topic(&self, tenant: &str, namespace: &str, topic: &str, force: bool, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/non-persistent/{tenant}/{namespace}/{topic}".to_string())
            .with_query_param("force".to_string(), force.to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn expire_messages_for_all_subscriptions(&self, tenant: &str, namespace: &str, topic: &str, expire_time_in_seconds: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/all_subscription/expireMessages/{expireTimeInSeconds}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("expireTimeInSeconds".to_string(), expire_time_in_seconds.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn expire_topic_messages(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, expire_time_in_seconds: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/expireMessages/{expireTimeInSeconds}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subName".to_string(), sub_name.to_string())
            .with_path_param("expireTimeInSeconds".to_string(), expire_time_in_seconds.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn get_backlog(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::PersistentOfflineTopicStats, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/backlog".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_internal_stats(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::PersistentTopicInternalStats, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/internalStats".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_last_message_id(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/lastMessageId".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_list(&self, tenant: &str, namespace: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_list_from_bundle(&self, tenant: &str, namespace: &str, bundle: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{bundle}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("bundle".to_string(), bundle.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_managed_ledger_info(&self, tenant: &str, namespace: &str, topic: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/internal-info".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn get_partitioned_metadata(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::PartitionedTopicMetadata, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/partitions".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_partitioned_stats(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/partitioned-stats".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn get_partitioned_topic_list(&self, tenant: &str, namespace: &str) -> Box<Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/partitioned".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_permissions_on_topic(&self, tenant: &str, namespace: &str, topic: &str) -> Box<Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/permissions".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_stats(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::TopicStats, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/stats".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_subscriptions(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/subscriptions".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn grant_permissions_on_topic(&self, tenant: &str, namespace: &str, topic: &str, role: &str, body: Vec<String>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/permissions/{role}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("role".to_string(), role.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn offload_status(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = crate::models::OffloadProcessStatus, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/offload".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn peek_nth_message(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, message_position: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/position/{messagePosition}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subName".to_string(), sub_name.to_string())
            .with_path_param("messagePosition".to_string(), message_position.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn reset_cursor(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, timestamp: i64, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/resetcursor/{timestamp}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subName".to_string(), sub_name.to_string())
            .with_path_param("timestamp".to_string(), timestamp.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn reset_cursor_on_position(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, authoritative: bool, message_id: crate::models::MessageIdImpl) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/resetcursor".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subName".to_string(), sub_name.to_string())
            .with_body_param(message_id)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn revoke_permissions_on_topic(&self, tenant: &str, namespace: &str, topic: &str, role: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/non-persistent/{tenant}/{namespace}/{topic}/permissions/{role}".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("role".to_string(), role.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn skip_all_messages(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/skip_all".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subName".to_string(), sub_name.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn skip_messages(&self, tenant: &str, namespace: &str, topic: &str, sub_name: &str, num_messages: i32, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/skip/{numMessages}".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_path_param("subName".to_string(), sub_name.to_string())
            .with_path_param("numMessages".to_string(), num_messages.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn terminate(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/terminate".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .execute(self.configuration.borrow())
    }

    fn trigger_offload(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/non-persistent/{tenant}/{namespace}/{topic}/offload".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn unload_topic(&self, tenant: &str, namespace: &str, topic: &str, authoritative: bool) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/non-persistent/{tenant}/{namespace}/{topic}/unload".to_string())
            .with_query_param("authoritative".to_string(), authoritative.to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn update_partitioned_topic(&self, tenant: &str, namespace: &str, topic: &str, body: i32) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/non-persistent/{tenant}/{namespace}/{topic}/partitions".to_string())
            .with_path_param("tenant".to_string(), tenant.to_string())
            .with_path_param("namespace".to_string(), namespace.to_string())
            .with_path_param("topic".to_string(), topic.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

}
