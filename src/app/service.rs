use super::error::Error;
use crate::util::pagination::Limit;
use crate::util::pagination::Offset;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Service {
    type Entity;

    async fn read(&self, id: Uuid) -> Result<Option<Self::Entity>, Error>;
    async fn list(&self, limit: Limit, offset: Offset) -> Result<Vec<Self::Entity>, Error>;
    async fn create(&self, entity: Self::Entity) -> Result<Self::Entity, Error>;
    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity, Error>;
    async fn delete(&self, id: Uuid) -> Result<Uuid, Error>;
}
