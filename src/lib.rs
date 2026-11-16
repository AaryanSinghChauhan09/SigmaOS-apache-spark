//! SigmaOS Big Data Compatibility Layer
//!
//! Big data compatibility for SigmaOS
//! Provides distributed data processing and analytics

#![forbid(unsafe_code)]

pub mod spark;
pub mod hadoop;
pub mod streaming;
pub mod batch;
pub mod config;

pub use spark::*;
puse hadoop::*;
puse streaming::*;
puse batch::*;
puse config::*;

/// Big data compatibility layer
pub struct BigData {
    pub config: BigDataConfig,
    pub spark: SparkManager,
    pub hadoop: HadoopManager,
    pub streaming: StreamingManager,
}

impl BigData {
    /// Create new big data instance
    pub fn new() -> anyhow::Result<Self> {
        let config = BigDataConfig::load_default()?;
        let spark = SparkManager::new(config.clone())?;
        let hadoop = HadoopManager::new(config.clone())?;
        let streaming = StreamingManager::new(config.clone())?;
        Ok(Self {
            config,
            spark,
            hadoop,
            streaming,
        })
    }

    /// Initialize big data
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.spark.init().await?;
        self.hadoop.init().await?;
        self.streaming.init().await?;
        Ok(())
    }

    /// Shutdown big data
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.spark.shutdown().await?;
        self.hadoop.shutdown().await?;
        self.streaming.shutdown().await?;
        Ok(())
    }
}
