use std::{sync::Arc, path::PathBuf,};
use async_trait::async_trait;

/// Any struct implementing this Asset can be Stored inside the Manager
#[async_trait]
pub trait Asset
where
    Self: Sized,
{
    async fn load<P: AsRef<async_std::path::Path> + Send>(path: P) -> Result<Self, std::io::Error>;
}

#[derive(Clone)]
pub(crate) struct AssetHandle<A> 
where A: Asset
{
    pub(crate) path: PathBuf,
    asset: Option<Arc<A>>,
}

impl<A: Asset> AssetHandle<A> {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path, asset: None }
    }

    pub(crate) async fn load(&mut self) -> Result<Arc<A>, std::io::Error> {
        if self.asset.is_none() {
            self.asset = Some(Arc::new(A::load(&self.path).await?));
        }
        Ok(self.asset.clone().unwrap())
    }
    pub(crate) async fn load_unloaded_raw(&self) -> Result<A, std::io::Error> {
        Ok(A::load(&self.path).await?)
    }
    pub(crate) fn set_raw(&mut self, asset:A) {
        self.asset = Some(Arc::new(asset))
    }
    pub(crate) fn unload(&mut self) {
        self.asset = None;
    }
    pub(crate) fn get(&self) -> Option<&Arc<A>> {
        self.asset.as_ref()
    }
}