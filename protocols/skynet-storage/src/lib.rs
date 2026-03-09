#![warn(missing_docs)]
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use skynet_types::SkyNetResult;
use uuid::Uuid;

/// Storage object types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageType {
    /// File storage
    File,
    /// Binary blob storage
    Blob,
    /// Object storage (S3-like)
    Object,
    /// Key-value storage
    KeyValue,
    /// Custom storage type
    Custom,
}

/// Storage access permission level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageAccessLevel {
    /// Private access only
    Private,
    /// Internal access within organization
    Internal,
    /// Public access
    Public,
}

/// Storage metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    /// Unique identifier for the storage object
    pub id: Uuid,
    /// Storage object name
    pub name: String,
    /// Storage type
    pub storage_type: StorageType,
    /// Object size in bytes
    pub size: u64,
    /// MIME type (if applicable)
    pub mime_type: Option<String>,
    /// Storage path or URL
    pub path: String,
    /// Owner ID
    pub owner_id: Uuid,
    /// Organization ID (optional)
    pub org_id: Option<Uuid>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Access permission level
    pub access_level: StorageAccessLevel,
    /// Additional metadata (key-value pairs)
    pub metadata: std::collections::HashMap<String, String>,
}

/// Storage service trait, defining core storage management interfaces
#[async_trait]
pub trait StorageService: Send + Sync {
    /// Upload a storage object
    ///
    /// # Parameters
    /// - `name`: Object name
    /// - `data`: Object data
    /// - `storage_type`: Storage type
    /// - `owner_id`: Owner ID
    /// - `org_id`: Organization ID (optional)
    /// - `access_level`: Access permission level
    /// - `metadata`: Additional metadata (optional)
    ///
    /// # Returns
    /// - Storage information on success
    /// - Error on failure
    async fn upload(
        &self,
        name: &str,
        data: Vec<u8>,
        storage_type: StorageType,
        owner_id: Uuid,
        org_id: Option<Uuid>,
        access_level: StorageAccessLevel,
        metadata: Option<std::collections::HashMap<String, String>>,
    ) -> SkyNetResult<StorageInfo>;

    /// Get storage object information
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    ///
    /// # Returns
    /// - Storage information on success
    /// - Error on failure
    async fn get_info(&self, storage_id: Uuid) -> SkyNetResult<StorageInfo>;

    /// Download a storage object
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    ///
    /// # Returns
    /// - Object data on success
    /// - Error on failure
    async fn download(&self, storage_id: Uuid) -> SkyNetResult<Vec<u8>>;

    /// Get download URL
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    /// - `expiry_seconds`: URL expiration time in seconds
    ///
    /// # Returns
    /// - Download URL on success
    /// - Error on failure
    async fn get_download_url(&self, storage_id: Uuid, expiry_seconds: u32) -> SkyNetResult<String>;

    /// Delete a storage object
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    ///
    /// # Returns
    /// - Empty on success
    /// - Error on failure
    async fn delete(&self, storage_id: Uuid) -> SkyNetResult<()>;

    /// List storage objects owned by a user
    ///
    /// # Parameters
    /// - `owner_id`: Owner ID
    /// - `storage_type`: Filter by storage type (optional)
    /// - `limit`: Result limit
    /// - `offset`: Result offset
    ///
    /// # Returns
    /// - List of storage information on success
    /// - Error on failure
    async fn list_by_owner(
        &self,
        owner_id: Uuid,
        storage_type: Option<StorageType>,
        limit: u32,
        offset: u32,
    ) -> SkyNetResult<Vec<StorageInfo>>;

    /// List storage objects within an organization
    ///
    /// # Parameters
    /// - `org_id`: Organization ID
    /// - `storage_type`: Filter by storage type (optional)
    /// - `limit`: Result limit
    /// - `offset`: Result offset
    ///
    /// # Returns
    /// - List of storage information on success
    /// - Error on failure
    async fn list_by_org(
        &self,
        org_id: Uuid,
        storage_type: Option<StorageType>,
        limit: u32,
        offset: u32,
    ) -> SkyNetResult<Vec<StorageInfo>>;

    /// Update storage access permission
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    /// - `access_level`: New access permission level
    ///
    /// # Returns
    /// - Updated storage information on success
    /// - Error on failure
    async fn update_access_level(&self, storage_id: Uuid, access_level: StorageAccessLevel) -> SkyNetResult<StorageInfo>;

    /// Update storage metadata
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    /// - `metadata`: New metadata (replaces existing)
    ///
    /// # Returns
    /// - Updated storage information on success
    /// - Error on failure
    async fn update_metadata(
        &self,
        storage_id: Uuid,
        metadata: std::collections::HashMap<String, String>,
    ) -> SkyNetResult<StorageInfo>;

    /// Copy a storage object
    ///
    /// # Parameters
    /// - `storage_id`: Source storage object ID
    /// - `new_name`: New object name (optional)
    /// - `new_owner_id`: New owner ID (optional, defaults to current owner)
    ///
    /// # Returns
    /// - New storage information on success
    /// - Error on failure
    async fn copy(&self, storage_id: Uuid, new_name: Option<&str>, new_owner_id: Option<Uuid>) -> SkyNetResult<StorageInfo>;

    /// Move a storage object
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    /// - `new_org_id`: New organization ID (optional)
    ///
    /// # Returns
    /// - Updated storage information on success
    /// - Error on failure
    async fn move_to(&self, storage_id: Uuid, new_org_id: Option<Uuid>) -> SkyNetResult<StorageInfo>;

    /// Search storage objects
    ///
    /// # Parameters
    /// - `query`: Search query
    /// - `owner_id`: Owner ID filter (optional)
    /// - `org_id`: Organization ID filter (optional)
    /// - `storage_type`: Storage type filter (optional)
    /// - `limit`: Result limit
    ///
    /// # Returns
    /// - List of matching storage information on success
    /// - Error on failure
    async fn search(
        &self,
        query: &str,
        owner_id: Option<Uuid>,
        org_id: Option<Uuid>,
        storage_type: Option<StorageType>,
        limit: u32,
    ) -> SkyNetResult<Vec<StorageInfo>>;

    /// Check if storage object exists
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    ///
    /// # Returns
    /// - True if exists, false otherwise
    async fn exists(&self, storage_id: Uuid) -> SkyNetResult<bool>;

    /// Get storage object size
    ///
    /// # Parameters
    /// - `storage_id`: Storage object ID
    ///
    /// # Returns
    /// - Object size in bytes
    async fn get_size(&self, storage_id: Uuid) -> SkyNetResult<u64>;
}
