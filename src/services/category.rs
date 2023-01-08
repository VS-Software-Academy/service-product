use crate::app::repository::Repository;
use crate::utils::pagination::Offset;
use crate::{
    app::{error::AppError, service::Service},
    models::category::Category,
    utils::pagination::Limit,
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

    async fn read(&self, id: uuid::Uuid) -> Result<Option<Self::Entity>, AppError> {
        self.repository.read(id).await
    }

    async fn list(&self, limit: Limit, offset: Offset) -> Result<Vec<Self::Entity>, AppError> {
        self.repository.list(limit, offset).await
    }

    async fn create(&self, entity: Self::Entity) -> Result<Self::Entity, AppError> {
        self.repository.create(entity).await
    }

    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity, AppError> {
        self.repository.update(entity).await
    }

    async fn delete(&self, id: uuid::Uuid) -> Result<Uuid, AppError> {
        self.repository.delete(id).await
    }
}
