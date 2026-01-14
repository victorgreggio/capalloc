use crate::domain::Asset;
use std::error::Error;

/// Repository for loading asset data
pub trait AssetRepository {
    fn load_all(&self) -> Result<Vec<Asset>, Box<dyn Error>>;
}

/// CSV-based implementation of AssetRepository
pub struct CsvAssetRepository {
    file_path: String,
}

impl CsvAssetRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl AssetRepository for CsvAssetRepository {
    fn load_all(&self) -> Result<Vec<Asset>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(&self.file_path)?;
        let mut assets = Vec::new();

        for result in rdr.deserialize() {
            let asset: Asset = result?;
            assets.push(asset);
        }

        Ok(assets)
    }
}
