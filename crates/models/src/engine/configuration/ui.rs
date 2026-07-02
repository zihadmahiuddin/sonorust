use serde::{Deserialize, Serialize};
use sonorust_ir::IRValue;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ui {
    pub primary_metric: String,
    pub secondary_metric: String,
    pub menu_visibility: Visibility,
    pub judgment_visibility: Visibility,
    pub combo_visibility: Visibility,
    pub primary_metric_visibility: Visibility,
    pub secondary_metric_visibility: Visibility,
    pub progress_visibility: Visibility,
    pub tutorial_navigation_visibility: Visibility,
    pub tutorial_instruction_visibility: Visibility,
    pub judgment_animation: Animation,
    pub combo_animation: Animation,
    pub judgment_error_style: String,
    pub judgment_error_placement: String,
    pub judgment_error_min: IRValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    pub scale: IRValue,
    pub alpha: IRValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    pub scale: AnimatedValue,
    pub alpha: AnimatedValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimatedValue {
    pub from: IRValue,
    pub to: IRValue,
    pub duration: IRValue,
    pub ease: String,
}
