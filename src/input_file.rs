//! Structs for handling and uploading files

use std::fmt;
use std::path::{Path, PathBuf};

use bytes::Bytes;
use serde::de::{Error as DeError, IgnoredAny, MapAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a new file to be uploaded via `multipart/form-data`.
///
/// See <https://core.telegram.org/bots/api#inputfile>.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputFile {
    Path(PathBuf),
    Memory { file_name: String, data: Bytes },
}

impl InputFile {
    /// 以本地路径构建文件。
    pub fn from_path<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::Path(path.into())
    }

    /// 以内存数据构建文件。
    pub fn memory<N, D>(file_name: N, data: D) -> Self
    where
        N: Into<String>,
        D: Into<Vec<u8>>,
    {
        let bytes: Vec<u8> = data.into();
        let data = Bytes::from(bytes);
        Self::Memory {
            file_name: file_name.into(),
            data,
        }
    }
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

impl<'de> Deserialize<'de> for InputFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InputFileVisitor;

        impl<'de> Visitor<'de> for InputFileVisitor {
            type Value = InputFile;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a file path string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(InputFile::from(PathBuf::from(value)))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(InputFile::from(PathBuf::from(value)))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Err(DeError::invalid_type(Unexpected::Unit, &self))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Err(DeError::invalid_type(Unexpected::Option, &self))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut path: Option<PathBuf> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "path" => {
                            path = Some(map.next_value()?);
                        }
                        "file_name" => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                        "data" => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }

                path.map(InputFile::from)
                    .ok_or_else(|| DeError::missing_field("path"))
            }
        }

        deserializer.deserialize_any(InputFileVisitor)
    }
}

impl From<PathBuf> for InputFile {
    fn from(path: PathBuf) -> Self {
        Self::from_path(path)
    }
}

impl From<&Path> for InputFile {
    fn from(path: &Path) -> Self {
        Self::from_path(path.to_path_buf())
    }
}

impl From<String> for InputFile {
    fn from(path: String) -> Self {
        Self::from_path(PathBuf::from(path))
    }
}

impl From<&str> for InputFile {
    fn from(path: &str) -> Self {
        Self::from_path(PathBuf::from(path))
    }
}

impl From<(String, Vec<u8>)> for InputFile {
    fn from((file_name, data): (String, Vec<u8>)) -> Self {
        Self::memory(file_name, data)
    }
}

impl<'a> From<(&'a str, Vec<u8>)> for InputFile {
    fn from((file_name, data): (&'a str, Vec<u8>)) -> Self {
        Self::memory(file_name, data)
    }
}

impl<'a> From<(&'a str, &'a [u8])> for InputFile {
    fn from((file_name, data): (&'a str, &'a [u8])) -> Self {
        Self::memory(file_name, data.to_vec())
    }
}

/// Represents different approaches of sending files.
///
/// See <https://core.telegram.org/bots/api#sending-files>.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum FileUpload {
    /// `file_id` to send a file that exists on the Telegram servers (recommended) or pass an HTTP URL for Telegram to get a file from the Internet
    String(String),
    /// upload a new file using `multipart/form-data`
    InputFile(InputFile),
}

impl From<String> for FileUpload {
    fn from(file: String) -> Self {
        Self::String(file)
    }
}

impl From<PathBuf> for FileUpload {
    fn from(path: PathBuf) -> Self {
        Self::InputFile(InputFile::from_path(path))
    }
}

impl From<InputFile> for FileUpload {
    fn from(file: InputFile) -> Self {
        Self::InputFile(file)
    }
}

impl From<(String, Vec<u8>)> for FileUpload {
    fn from(value: (String, Vec<u8>)) -> Self {
        Self::InputFile(InputFile::from(value))
    }
}

impl<'a> From<(&'a str, Vec<u8>)> for FileUpload {
    fn from(value: (&'a str, Vec<u8>)) -> Self {
        Self::InputFile(InputFile::from(value))
    }
}

impl<'a> From<(&'a str, &'a [u8])> for FileUpload {
    fn from(value: (&'a str, &'a [u8])) -> Self {
        Self::InputFile(InputFile::from(value))
    }
}

impl From<(String, Bytes)> for InputFile {
    fn from((file_name, data): (String, Bytes)) -> Self {
        Self::Memory { file_name, data }
    }
}

impl<'a> From<(&'a str, Bytes)> for InputFile {
    fn from((file_name, data): (&'a str, Bytes)) -> Self {
        Self::Memory {
            file_name: file_name.to_owned(),
            data,
        }
    }
}

impl From<(String, Bytes)> for FileUpload {
    fn from(value: (String, Bytes)) -> Self {
        Self::InputFile(InputFile::from(value))
    }
}

impl<'a> From<(&'a str, Bytes)> for FileUpload {
    fn from(value: (&'a str, Bytes)) -> Self {
        Self::InputFile(InputFile::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(feature = "trait-sync", feature = "trait-async"))]
    use super::HasInputFile;

    #[cfg(any(feature = "trait-sync", feature = "trait-async"))]
    #[test]
    fn replace_attach_preserves_memory_bytes() {
        let data = vec![1_u8, 2, 3];
        let mut upload = FileUpload::from(InputFile::memory("demo.bin", data.clone()));

        let file = upload.replace_attach("payload").expect("should have bytes");

        match upload {
            FileUpload::String(ref value) => assert_eq!(value, "attach://payload"),
            FileUpload::InputFile(_) => panic!("file upload should be converted to attach"),
        }

        match file {
            InputFile::Memory {
                file_name,
                data: bytes,
            } => {
                assert_eq!(file_name, "demo.bin");
                assert_eq!(bytes.as_ref(), data.as_slice());
            }
            InputFile::Path(_) => panic!("expected memory variant"),
        }
    }

    #[test]
    fn memory_input_file_serializes_as_null() {
        #[derive(Serialize)]
        struct Wrapper {
            file: InputFile,
        }

        let payload = Wrapper {
            file: InputFile::memory("demo.bin", vec![0, 1, 2, 3]),
        };

        let json = serde_json::to_string(&payload).expect("serialize wrapper");
        assert_eq!(json, "{\"file\":null}");
    }
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
pub(crate) trait HasInputFile {
    fn replace_attach(&mut self, name: &str) -> Option<InputFile>;
    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, InputFile)>;
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for FileUpload {
    fn replace_attach(&mut self, name: &str) -> Option<InputFile> {
        match self {
            Self::InputFile(_) => {
                let attach = Self::String(format!("attach://{name}"));
                let Self::InputFile(file) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some(file)
            }
            Self::String(_) => None,
        }
    }

    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, InputFile)> {
        match self {
            Self::InputFile(_) => {
                let name = format!("file{}", index());
                let attach = Self::String(format!("attach://{name}"));
                let Self::InputFile(file) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some((name, file))
            }
            Self::String(_) => None,
        }
    }
}

#[cfg(any(feature = "trait-sync", feature = "trait-async"))]
impl HasInputFile for Option<FileUpload> {
    fn replace_attach(&mut self, name: &str) -> Option<InputFile> {
        match self {
            Some(FileUpload::InputFile(_)) => {
                let attach = Some(FileUpload::String(format!("attach://{name}")));
                let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some(file)
            }
            _ => None,
        }
    }

    fn replace_attach_dyn(&mut self, index: impl FnOnce() -> usize) -> Option<(String, InputFile)> {
        match self {
            Some(FileUpload::InputFile(_)) => {
                let name = format!("file{}", index());
                let attach = Some(FileUpload::String(format!("attach://{name}")));
                let Some(FileUpload::InputFile(file)) = std::mem::replace(self, attach) else {
                    unreachable!("the match already ensures it being an input file");
                };
                Some((name, file))
            }
            _ => None,
        }
    }
}
