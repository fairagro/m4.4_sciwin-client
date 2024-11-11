use super::types::CWLType;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yml::Value;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutputParameter {
    #[serde(default)]
    pub id: String,
    pub type_: CWLType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_binding: Option<CommandOutputBinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl CommandOutputParameter {
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    pub fn with_type(mut self, type_: CWLType) -> Self {
        self.type_ = type_;
        self
    }
    pub fn with_binding(mut self, binding: CommandOutputBinding) -> Self {
        self.output_binding = Some(binding);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutputBinding {
    pub glob: String,
}

pub fn deserialize_outputs<'de, D>(deserializer: D) -> Result<Vec<CommandOutputParameter>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    let parameters = match value {
        Value::Sequence(seq) => seq
            .into_iter()
            .map(|item| {
                let param: CommandOutputParameter = serde_yml::from_value(item).map_err(serde::de::Error::custom)?;
                Ok(param)
            })
            .collect::<Result<Vec<_>, _>>()?,
        Value::Mapping(map) => map
            .into_iter()
            .map(|(key, value)| {
                let id = key.as_str().ok_or_else(|| serde::de::Error::custom("Expected string key"))?;
                let mut param: CommandOutputParameter = serde_yml::from_value(value).map_err(serde::de::Error::custom)?;
                param.id = id.to_string();
                Ok(param)
            })
            .collect::<Result<Vec<_>, _>>()?,
        _ => return Err(serde::de::Error::custom("Expected sequence or mapping for outputs")),
    };

    Ok(parameters)
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowOutputParameter {
    pub id: String,
    pub type_: CWLType,
    pub output_source: String,
}

impl WorkflowOutputParameter {
    pub fn with_id(&mut self, id: &str) -> &Self {
        self.id = id.to_string();
        self
    }
}