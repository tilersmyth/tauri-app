use tauri::{command, AppHandle, Wry};

use super::{GetParams, IpcResponse};
use crate::ctx::{self, Ctx};
use crate::{Error, Result};
use reqwest;
use std::sync::Arc;

use crate::model::{Cat, CatBmc};

#[command]
pub async fn get_cat(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Cat> {
    match Ctx::from_app(app) {
        Ok(ctx) => CatBmc::get(ctx, &params.id).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}
