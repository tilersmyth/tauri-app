use crate::Result;
use reqwest::{Client, ClientBuilder};
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};

pub struct Ctx {
    app_handle: AppHandle<Wry>,
    http_client: Client,
}

impl Ctx {
    pub fn new(app_handle: AppHandle<Wry>, http_client: Client) -> Self {
        Ctx {
            app_handle,
            http_client,
        }
    }

    pub fn from_app(app: AppHandle<Wry>) -> Result<Arc<Ctx>> {
        let http_client = ClientBuilder::new().build()?;
        Ok(Arc::new(Ctx::new(app, http_client)))
    }

    pub fn get_http_client(&self) -> Client {
        self.http_client.clone()
    }
}
