/// In the [OCI Content Descriptors](https://github.com/opencontainers/image-spec/blob/master/descriptor.md)
/// specification, a descriptor consists of a set of properties encapsulated in key-value fields.

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Descriptor {
  #[serde(alias = "mediaType")]
  pub media_type: String,
  #[serde(alias = "digest")]
  pub digest: String,
  #[serde(alias = "size")]
  pub size: i64,
  #[serde(alias = "urls")]
  pub urls: Option<Vec<String>>,
  #[serde(alias = "annotations")]
  pub annotations: Option<Vec<HashMap<String, String>>>,
}