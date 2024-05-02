use anyhow::Result;
use core::fmt;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl CmapMetrics {
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn debug(&self) -> Result<impl Into<String>> {
        Ok(format!("{:?}", self.data))
    }
}

impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug = self.debug().map_err(|_| fmt::Error {})?;
        write!(f, "{}", debug.into())
    }
}
