use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor};

use std::fmt;
use std::marker::PhantomData;

use super::{LazyCell, AtomicLazyCell};

impl<T: Serialize> Serialize for LazyCell<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.borrow() {
            Some(val) => serializer.serialize_some(val),
            None => serializer.serialize_none()
        }
    }
}


impl<T: Serialize> Serialize for AtomicLazyCell<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.borrow() {
            Some(val) => serializer.serialize_some(val),
            None => serializer.serialize_none()
        }
    }
}

struct LazyCellVisitor<T>(PhantomData<*const T>);
impl<'de, T: Deserialize<'de>> Visitor<'de> for LazyCellVisitor<T> {
    type Value = LazyCell<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a LazyCell")
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        let mut cell = LazyCell::new();
        cell.replace(T::deserialize(deserializer)?);
        Ok(cell)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(LazyCell::new())
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for LazyCell<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_option(LazyCellVisitor(PhantomData))
    }
}


struct AtomicLazyCellVisitor<T>(PhantomData<*const T>);
impl<'de, T: Deserialize<'de>> Visitor<'de> for AtomicLazyCellVisitor<T> {
    type Value = AtomicLazyCell<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an AtomicLazyCell")
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        let mut cell = AtomicLazyCell::new();
        cell.replace(T::deserialize(deserializer)?);
        Ok(cell)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(AtomicLazyCell::new())
    }
}


impl<'de, T: Deserialize<'de>> Deserialize<'de> for AtomicLazyCell<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_option(AtomicLazyCellVisitor(PhantomData))
    }
}