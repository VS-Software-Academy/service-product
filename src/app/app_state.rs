use crate::service::category::CategoryService;
use crate::service::product::ProductService;
use std::sync::Arc;

struct InnerAppState {
    product_service: ProductService,
    category_service: CategoryService,
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<InnerAppState>,
}

impl AppState {
    pub fn new(product_service: ProductService, category_service: CategoryService) -> Self {
        Self {
            inner: Arc::new(InnerAppState {
                product_service,
                category_service,
            }),
        }
    }

    pub fn product_service(&self) -> &ProductService {
        &self.inner.product_service
    }

    pub fn category_service(&self) -> &CategoryService {
        &self.inner.category_service
    }
}
