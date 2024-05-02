use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!("{:?}", e))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!("{:?}", e))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self.data.read().map_err(|e| anyhow!("{:?}", e))?.clone())
    }

    pub fn debug(&self) -> Result<impl Into<String>> {
        let data = self.data.read().map_err(|e| anyhow!("{:?}", e))?;
        Ok(format!("{:?}", data))
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug = self.debug().map_err(|_| fmt::Error {})?;
        write!(f, "{}", debug.into())
    }
}
