use forceps::Cache;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_binary::binary_stream::Endian;

use crate::error::BoxResult;

pub struct Database {
    cache: Cache,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub async fn open() -> BoxResult<Self> {
        Ok(Database {
            cache: Cache::new("database").build().await?,
        })
    }

    pub async fn insert<K: AsRef<[u8]>, T: Serialize>(
        &self,
        key: K,
        value: &T,
    ) -> BoxResult<forceps::Metadata> {
        let vec_bytes = serde_binary::to_vec(value, Endian::Little)?;
        Ok(self.cache.write(key, vec_bytes).await?)
    }

    pub async fn get<K: AsRef<[u8]>, T: DeserializeOwned>(&self, key: K) -> BoxResult<T> {
        let value = self.cache.read(key).await?;
        Ok(serde_binary::from_vec(value.to_vec(), Endian::Little)?)
    }
}
