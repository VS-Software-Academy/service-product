use serde::Deserialize;

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<Offset>,
    pub limit: Option<Limit>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Offset(pub i64);

#[derive(Debug, Deserialize, Default)]
pub struct Limit(pub i64);
