use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn deserialize_array<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v = Vec::<T>::deserialize(deserializer)?;
    v.try_into().map_err(|v: Vec<T>| {
        serde::de::Error::invalid_length(v.len(), &format!("{N} elements").as_str())
    })
}

pub fn serialize_array<S, T, const N: usize>(
    value: &[T; N],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: serde::Serialize,
{
    value.as_slice().serialize(serializer)
}
