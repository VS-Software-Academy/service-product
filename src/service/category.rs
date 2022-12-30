use crate::app::repository::Repository;
use crate::util::pagination::Offset;
use crate::{
    app::{error::Error, service::Service},
    model::category::Category,
    util::pagination::Limit,
};
use async_trait::async_trait;
use uuid::Uuid;

pub struct CategoryService {
    repository: Box<dyn Repository<Entity = Category> + Send + Sync + 'static>,
}

impl CategoryService {
    pub fn new(repository: impl Repository<Entity = Category> + Send + Sync + 'static) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }
}

#[async_trait]
impl Service for CategoryService {
    type Entity = Category;

    async fn read(&self, id: uuid::Uuid) -> Result<Option<Self::Entity>, Error> {
        self.repository.read(id).await
    }

    async fn list(&self, limit: Limit, offset: Offset) -> Result<Vec<Self::Entity>, Error> {
        self.repository.list(limit, offset).await
    }

    async fn create(&self, entity: Self::Entity) -> Result<Self::Entity, Error> {
        self.repository.create(entity).await
    }

    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity, Error> {
        self.repository.update(entity).await
    }

    async fn delete(&self, id: uuid::Uuid) -> Result<Uuid, Error> {
        self.repository.delete(id).await
    }
}
