use crate::repository::{category::DbCategoryRepo, product::DbProductRepo};
use std::sync::Arc;

struct InnerAppState {
    db_product_repo: DbProductRepo,
    db_category_repo: DbCategoryRepo,
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<InnerAppState>,
}

impl AppState {
    pub fn new(db_product_repo: DbProductRepo, db_category_repo: DbCategoryRepo) -> Self {
        Self {
            inner: Arc::new(InnerAppState {
                db_product_repo,
                db_category_repo,
            }),
        }
    }

    pub fn db_product_repo(&self) -> &DbProductRepo {
        &self.inner.db_product_repo
    }

    pub fn db_category_repo(&self) -> &DbCategoryRepo {
        &self.inner.db_category_repo
    }
}
