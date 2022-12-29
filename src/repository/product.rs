use crate::{
    core::repository::Repository,
    entity::product::Product,
    util::pagination::{Limit, Offset},
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub trait ProductRepo: Repository<Entity = Product> {}

pub struct DbProductRepo {
    pool: Pool<Postgres>,
}

impl DbProductRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for DbProductRepo {
    type Entity = Product;

    async fn read(&self, id: Uuid) -> Result<Option<Self::Entity>, crate::core::error::Error> {
        let result = sqlx::query_as!(Product, r#"SELECT * FROM "product" WHERE ID = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result)
    }

    async fn list(
        &self,
        limit: Limit,
        offset: Offset,
    ) -> Result<Vec<Self::Entity>, crate::core::error::Error> {
        let list = sqlx::query_as!(
            Product,
            r#"SELECT * FROM "product" OFFSET $1 LIMIT $2"#,
            offset.0,
            limit.0
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(list)
    }

    async fn create(
        &self,
        entity: Self::Entity,
    ) -> Result<Self::Entity, crate::core::error::Error> {
        sqlx::query!(
            r#"INSERT INTO "product" ("id", "description", "category", "price", "created_at")
VALUES ($1, $2, $3, $4, $5)"#,
            entity.id,
            entity.description,
            entity.category,
            entity.price,
            entity.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(entity)
    }

    async fn update(
        &self,
        entity: Self::Entity,
    ) -> Result<Self::Entity, crate::core::error::Error> {
        sqlx::query!(
            r#"UPDATE "product" SET "description" = $1, "category" = $2, "price" = $3,
"created_at" = $4 WHERE "id" = $5"#,
            entity.description,
            entity.category,
            entity.price,
            entity.created_at,
            entity.id,
        )
        .execute(&self.pool)
        .await?;
        Ok(entity)
    }

    async fn delete(&self, id: Uuid) -> Result<Uuid, crate::core::error::Error> {
        sqlx::query!(r#"DELETE FROM "product" WHERE "id" = $1"#, id,)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }
}
