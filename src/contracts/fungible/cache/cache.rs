// RGB standard library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use lnpbp::rgb::prelude::*;

use super::sql::SqlCacheError;
use super::FileCacheError;
use crate::error::{BootstrapError, ServiceErrorDomain};
use crate::fungible::Asset;
use crate::util::file::FileMode;

pub trait Cache {
    type Error: ::std::error::Error + Into<ServiceErrorDomain>;

    fn assets(&self) -> Result<Vec<&Asset>, Self::Error>;
    fn asset(&self, id: ContractId) -> Result<&Asset, Self::Error>;
    fn has_asset(&self, id: ContractId) -> Result<bool, Self::Error>;
    fn add_asset(&mut self, asset: Asset) -> Result<bool, Self::Error>;
    fn remove_asset(&mut self, id: ContractId) -> Result<bool, Self::Error>;
}

#[derive(Clone, PartialEq, Eq, Debug, Display, Error)]
#[display(Debug)]
pub enum CacheError {
    Io(String),
    NotFound {
        id: String,
    },
    DataAccessError {
        id: String,
        mode: FileMode,
        details: Option<String>,
    },
    DataIntegrityError(String),

    Sqlite(String),
}

impl From<CacheError> for ServiceErrorDomain {
    fn from(_: CacheError) -> Self {
        ServiceErrorDomain::Cache
    }
}

impl From<CacheError> for BootstrapError {
    fn from(_: CacheError) -> Self {
        BootstrapError::CacheError
    }
}

impl From<FileCacheError> for CacheError {
    fn from(err: FileCacheError) -> Self {
        match err {
            FileCacheError::Io(e) => Self::Io(format!("{:?}", e)),
            FileCacheError::HashName => {
                Self::DataIntegrityError("File for a given hash id is not found".to_string())
            }
            FileCacheError::Encoding(e) => Self::DataIntegrityError(format!("{:?}", e)),
            FileCacheError::BrokenHexFilenames => {
                Self::DataIntegrityError("Broken filename structure in storage".to_string())
            }
            FileCacheError::SerdeJson(e) => Self::DataIntegrityError(format!("{:?}", e)),
            FileCacheError::SerdeYaml(e) => Self::DataIntegrityError(format!("{:?}", e)),
            FileCacheError::SerdeToml => {
                Self::DataIntegrityError(format!("TOML serialization/deserialization error"))
            }
            FileCacheError::NotFound => {
                Self::DataIntegrityError("Data file is not found".to_string())
            }
        }
    }
}

impl From<SqlCacheError> for CacheError {
    fn from(err: SqlCacheError) -> Self {
        match err {
            SqlCacheError::Io(e) => Self::Io(format!("{:?}", e)),
            SqlCacheError::Sqlite(e) => {
                Self::Sqlite(format!("Error from sqlite asset cache {}", e.to_string()))
            }
            SqlCacheError::HexDecoding => Self::DataIntegrityError(format!(
                "Wrong hex encoded data in sqlite asset cache table"
            )),
            SqlCacheError::Generic(e) => Self::DataIntegrityError(e),
            SqlCacheError::WrongChainData(e) => Self::DataIntegrityError(format!(
                "Wrong Chain data in sqlite asset cache table: {}",
                e
            )),
            SqlCacheError::NotFound => {
                Self::DataIntegrityError(format!("Asset cache sqlite database file not found"))
            }
            SqlCacheError::BlindKey(e) => Self::DataIntegrityError(format!(
                "Wrong amount blinding factor in asset cache sqlite database: {}",
                e
            )),
        }
    }
}
