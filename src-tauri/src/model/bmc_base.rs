use std::sync::Arc;

use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::{Error, Result};

use crate::ctx::Ctx;

use super::Cat;

pub(super) async fn bmc_get<E: DeserializeOwned>(
    ctx: Arc<Ctx>,
    endpoint: &'static str,
    id: &str,
) -> Result<E> {
    let test = ctx
        .get_http_client()
        .get(format!("http://localhost:3000/{}/{}", endpoint, id))
        .send()
        .await?
        .json::<E>()
        .await?;

    Ok(test)
}
