use serde::{Deserialize, Serialize};
use sonorust_ir::IRValue;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkinSpriteName(pub String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinData {
    pub width: IRValue,
    pub height: IRValue,
    pub interpolation: bool,
    pub sprites: Vec<SkinSprite>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinSprite {
    pub name: SkinSpriteName,
    pub x: IRValue,
    pub y: IRValue,
    pub w: IRValue,
    pub h: IRValue,
    pub transform: SkinSpriteTransform,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinSpriteTransform {
    pub x1: SkinSpriteTransformExpression,
    pub y1: SkinSpriteTransformExpression,
    pub x2: SkinSpriteTransformExpression,
    pub y2: SkinSpriteTransformExpression,
    pub x3: SkinSpriteTransformExpression,
    pub y3: SkinSpriteTransformExpression,
    pub x4: SkinSpriteTransformExpression,
    pub y4: SkinSpriteTransformExpression,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinSpriteTransformExpression {
    pub x1: Option<IRValue>,
    pub x2: Option<IRValue>,
    pub x3: Option<IRValue>,
    pub x4: Option<IRValue>,
    pub y1: Option<IRValue>,
    pub y2: Option<IRValue>,
    pub y3: Option<IRValue>,
    pub y4: Option<IRValue>,
}
