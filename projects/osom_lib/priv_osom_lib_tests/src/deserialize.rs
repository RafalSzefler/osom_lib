use serde::{Deserialize, Serialize, de::DeserializeSeed};

pub fn deserialize_json<'de, T>(text: &'de str) -> Result<T, serde_json::Error>
where
    T: Deserialize<'de>,
{
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let result = T::deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(result)
}

pub fn deserialize_json_with_seed<'de, T, TSeed>(text: &'de str, seed: TSeed) -> Result<T, serde_json::Error>
where
    TSeed: DeserializeSeed<'de, Value = T>,
{
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let result = seed.deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(result)
}

struct StringIOAdapter(String);

impl std::io::Write for StringIOAdapter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "byte buffer is not valid UTF-8"))?;
        self.0.push_str(s);
        Ok(s.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn serialize_json<T>(value: &T) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    let mut serializer = serde_json::Serializer::new(StringIOAdapter(String::new()));
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner().0)
}
