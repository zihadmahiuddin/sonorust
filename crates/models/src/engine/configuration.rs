use serde::{Deserialize, Serialize};

use option::EngineOption;
use ui::Ui;

pub mod option;
pub mod ui;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineConfiguration {
    pub options: Vec<EngineOption>,
    pub ui: Ui,
}
