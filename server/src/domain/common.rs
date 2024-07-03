use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct ID(pub String);
