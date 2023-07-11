use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Marker<T: Sized, M>(pub T, [M; 0]);

impl<T, M> Marker<T, M> {
    pub fn new(v: T) -> Self {
        Self(v, [])
    }
}

impl<T: PartialEq, M> PartialEq for Marker<T, M> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl<T: Serialize, M> Serialize for Marker<T, M> {
    // シリアライズの実装
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>, M> Deserialize<'de> for Marker<T, M> {
    // デシリアライズの実装
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Marker(T::deserialize(deserializer)?, []))
    }
}

impl<T: std::fmt::Debug, M> std::fmt::Debug for Marker<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
