use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepAwakeConfig {
  /// Keep display on
  pub display: bool,
  /// Keep system from idle sleeping
  pub idle: bool,
  /// Keep system from sleeping (Functionality and conditions for this to work vary by OS)
  pub sleep: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepAwakeRequest {
  pub config: Option<KeepAwakeConfig>,
}