/*
 * Konnect Control Planes
 *
 * The API for Kong Konnect Control Planes.
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PaginatedMeta : returns the pagination information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedMeta {
    #[serde(rename = "page")]
    pub page: Box<models::PageMeta>,
}

impl PaginatedMeta {
    /// returns the pagination information
    pub fn new(page: models::PageMeta) -> PaginatedMeta {
        PaginatedMeta {
            page: Box::new(page),
        }
    }
}

