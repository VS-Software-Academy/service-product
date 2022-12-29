use crate::{
    core::repository::Repository,
    entity::category::Category,
    util::pagination::{Limit, Offset},
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub trait CategoryRepo: Repository<Entity = Category> {}

pub struct DbCategoryRepo {
    pool: Pool<Postgres>,
}

impl DbCategoryRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for DbCategoryRepo {
    type Entity = Category;

    async fn read(&self, id: Uuid) -> Result<Option<Self::Entity>, crate::core::error::Error> {
        let result = sqlx::query_as!(
            Category,
            r#"SELECT * FROM "product_category" WHERE ID = $1"#,
            id
        )
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
            Category,
            r#"SELECT * FROM "product_category" OFFSET $1 LIMIT $2"#,
            limit.0,
            offset.0
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
            r#"INSERT INTO "product_category" ("id", "description", "created_at") VALUES ($1, $2, $3)"#,
            entity.id,
            entity.description,
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
            r#"UPDATE "product_category" SET "description" = $1, "created_at" = $2 WHERE "id" = $3"#,
            entity.description,
            entity.created_at,
            entity.id,
        )
        .execute(&self.pool)
        .await?;
        Ok(entity)
    }

    async fn delete(&self, id: Uuid) -> Result<Uuid, crate::core::error::Error> {
        sqlx::query!(r#"DELETE FROM "product_category" WHERE "id" = $1"#, id,)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }
}
