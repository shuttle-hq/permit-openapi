/*
 * Permit.io API
 *
 *  Authorization as a service 
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`bulk_create_relationship_tuples`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkCreateRelationshipTuplesError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_delete_relationship_tuples`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkDeleteRelationshipTuplesError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_relationship_tuple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRelationshipTupleError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_relationship_tuple`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRelationshipTupleError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_relationship_tuples`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRelationshipTuplesError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


pub async fn bulk_create_relationship_tuples(configuration: &configuration::Configuration, proj_id: &str, env_id: &str, relationship_tuple_create_bulk_operation: models::RelationshipTupleCreateBulkOperation) -> Result<serde_json::Value, Error<BulkCreateRelationshipTuplesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/facts/{proj_id}/{env_id}/relationship_tuples/bulk", local_var_configuration.base_path, proj_id=crate::apis::urlencode(proj_id), env_id=crate::apis::urlencode(env_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&relationship_tuple_create_bulk_operation);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BulkCreateRelationshipTuplesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn bulk_delete_relationship_tuples(configuration: &configuration::Configuration, proj_id: &str, env_id: &str, relationship_tuple_delete_bulk_operation: models::RelationshipTupleDeleteBulkOperation) -> Result<serde_json::Value, Error<BulkDeleteRelationshipTuplesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/facts/{proj_id}/{env_id}/relationship_tuples/bulk", local_var_configuration.base_path, proj_id=crate::apis::urlencode(proj_id), env_id=crate::apis::urlencode(env_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&relationship_tuple_delete_bulk_operation);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BulkDeleteRelationshipTuplesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a relationship between two resource instances using a relation.
pub async fn create_relationship_tuple(configuration: &configuration::Configuration, proj_id: &str, env_id: &str, relationship_tuple_create: models::RelationshipTupleCreate) -> Result<models::RelationshipTupleRead, Error<CreateRelationshipTupleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/facts/{proj_id}/{env_id}/relationship_tuples", local_var_configuration.base_path, proj_id=crate::apis::urlencode(proj_id), env_id=crate::apis::urlencode(env_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&relationship_tuple_create);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRelationshipTupleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a relationship between two resource instances.
pub async fn delete_relationship_tuple(configuration: &configuration::Configuration, proj_id: &str, env_id: &str, relationship_tuple_delete: models::RelationshipTupleDelete) -> Result<(), Error<DeleteRelationshipTupleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/facts/{proj_id}/{env_id}/relationship_tuples", local_var_configuration.base_path, proj_id=crate::apis::urlencode(proj_id), env_id=crate::apis::urlencode(env_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&relationship_tuple_delete);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteRelationshipTupleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists the relationship tuples defined within an environment.
pub async fn list_relationship_tuples(configuration: &configuration::Configuration, proj_id: &str, env_id: &str, detailed: Option<bool>, page: Option<i32>, per_page: Option<i32>, tenant: Option<&str>, subject: Option<&str>, relation: Option<&str>, object: Option<&str>, object_type: Option<&str>, subject_type: Option<&str>) -> Result<Vec<models::RelationshipTupleRead>, Error<ListRelationshipTuplesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/facts/{proj_id}/{env_id}/relationship_tuples", local_var_configuration.base_path, proj_id=crate::apis::urlencode(proj_id), env_id=crate::apis::urlencode(env_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = detailed {
        local_var_req_builder = local_var_req_builder.query(&[("detailed", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tenant {
        local_var_req_builder = local_var_req_builder.query(&[("tenant", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = subject {
        local_var_req_builder = local_var_req_builder.query(&[("subject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = relation {
        local_var_req_builder = local_var_req_builder.query(&[("relation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = object {
        local_var_req_builder = local_var_req_builder.query(&[("object", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = object_type {
        local_var_req_builder = local_var_req_builder.query(&[("object_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = subject_type {
        local_var_req_builder = local_var_req_builder.query(&[("subject_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListRelationshipTuplesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

