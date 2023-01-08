use super::error::AppError;
use crate::utils::pagination::Limit;
use crate::utils::pagination::Offset;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Service {
    type Entity;

    async fn read(&self, id: Uuid) -> Result<Option<Self::Entity>, AppError>;
    async fn list(&self, limit: Limit, offset: Offset) -> Result<Vec<Self::Entity>, AppError>;
    async fn create(&self, entity: Self::Entity) -> Result<Self::Entity, AppError>;
    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity, AppError>;
    async fn delete(&self, id: Uuid) -> Result<Uuid, AppError>;
}
