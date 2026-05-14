use crate::common::*;

/// 자산 카테고리. DB 에는 영문 UPPER_SNAKE 로 저장.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr, Default,
    DynamoEnum, Translate,
)]
pub enum Category {
    #[default]
    #[translate(en = "Other", ko = "기타")]
    Unknown,
    #[translate(en = "🎵 Music IP", ko = "🎵 음악 IP")]
    Music,
    #[translate(en = "🎨 Art", ko = "🎨 미술품")]
    Art,
    #[translate(en = "🏢 Real estate", ko = "🏢 부동산")]
    RealEstate,
    #[translate(en = "🐂 Livestock", ko = "🐂 한우·축산")]
    Livestock,
}

impl Category {
    /// 카테고리 아이콘 (이모지) — 언어 중립.
    pub fn icon(self) -> &'static str {
        use Category::*;
        match self {
            RealEstate => "🏢",
            Art => "🎨",
            Music => "🎵",
            Livestock => "🐂",
            Unknown => "·",
        }
    }
}
