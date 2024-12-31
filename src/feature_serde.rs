use crate::GridBuf;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;

impl<T, Store: Serialize> Serialize for GridBuf<T, Store> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("GridBuf", 3)?;
        ser.serialize_field("width", &self.width)?;
        ser.serialize_field("height", &self.height)?;
        ser.serialize_field("store", &self.store)?;
        ser.end()
    }
}

impl<'de, T, S: Deserialize<'de>> Deserialize<'de> for GridBuf<T, S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename(deserialize = "GridBuf"))]
        struct GridBufDe<S> {
            width: usize,
            height: usize,
            store: S,
        }
        let GridBufDe {
            width,
            height,
            store,
        } = GridBufDe::deserialize(deserializer)?;
        Ok(Self {
            width,
            height,
            store,
            marker: PhantomData,
        })
    }
}
