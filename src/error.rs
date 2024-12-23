use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum KeepAwakeError {
  AlreadyStarted,
}

impl std::fmt::Display for KeepAwakeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::AlreadyStarted => write!(f, "KeepAwake is already started"),
    }
  }
}


#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[cfg(mobile)]
  #[error(transparent)]
  PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),

  #[error(transparent)]
  Keepawake(#[from] keepawake::Error),
  #[error("KeepAwake plugin error: {0}")]
  KeepawakePlugin(KeepAwakeError),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
