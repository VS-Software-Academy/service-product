use crate::app::repository::Repository;
use crate::util::pagination::Offset;
use crate::{
    app::{error::Error, service::Service},
    model::product::Product,
    util::pagination::Limit,
};
use async_trait::async_trait;
use rust_decimal_macros::dec;
use uuid::Uuid;

pub struct ProductService {
    repository: Box<dyn Repository<Entity = Product> + Send + Sync + 'static>,
}

impl ProductService {
    pub fn new<R: Repository<Entity = Product> + Send + Sync + 'static>(repository: R) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }

    fn validate(&self, entity: &Product) -> Result<(), Error> {
        if entity.price == dec!(0) {
            return Err(Error::Validation(String::from("price can't be zero")));
        }
        Ok(())
    }
}

#[async_trait]
impl Service for ProductService {
    type Entity = Product;

    async fn read(&self, id: uuid::Uuid) -> Result<Option<Self::Entity>, Error> {
        self.repository.read(id).await
    }

    async fn list(&self, limit: Limit, offset: Offset) -> Result<Vec<Self::Entity>, Error> {
        self.repository.list(limit, offset).await
    }

    async fn create(&self, entity: Self::Entity) -> Result<Self::Entity, Error> {
        self.validate(&entity)?;
        self.repository.create(entity).await
    }

    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity, Error> {
        self.validate(&entity)?;
        self.repository.update(entity).await
    }

    async fn delete(&self, id: uuid::Uuid) -> Result<Uuid, Error> {
        self.repository.delete(id).await
    }
}
