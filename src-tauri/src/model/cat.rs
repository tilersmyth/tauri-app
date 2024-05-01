use crate::ctx::Ctx;
use crate::{Error, Result};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

use super::bmc_base::bmc_get;

#[derive(TS, Deserialize, Serialize)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct Cat {
    id: String,
    name: String,
    hobby: String,
}

pub struct CatBmc;

impl CatBmc {
    const ENDPOINT: &'static str = "cats";

    pub async fn get(ctx: Arc<Ctx>, id: &str) -> Result<Cat> {
        bmc_get(ctx, Self::ENDPOINT, id).await
    }
}
