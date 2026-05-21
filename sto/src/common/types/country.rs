use crate::common::*;

/// 국가. 표시명은 Translate, 국기 이모지는 `flag()` 헬퍼.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
    Translate,
)]
pub enum Country {
    #[default]
    #[translate(en = "🌍 Global", ko = "🌍 해외")]
    Unknown,
    #[translate(en = "🇰🇷 Korea", ko = "🇰🇷 한국")]
    Kr,
    #[translate(en = "🇺🇸 United States", ko = "🇺🇸 미국")]
    Us,
    #[translate(en = "🇸🇬 Singapore", ko = "🇸🇬 싱가포르")]
    Sg,
    #[translate(en = "🇪🇺 Europe", ko = "🇪🇺 유럽")]
    Eu,
    #[translate(en = "🌍 Global", ko = "🌍 해외")]
    Other,
}

impl Country {
    /// 국기 이모지 — 언어 중립.
    pub fn flag(self) -> &'static str {
        use Country::*;
        match self {
            Kr => "🇰🇷",
            Us => "🇺🇸",
            Sg => "🇸🇬",
            Eu => "🇪🇺",
            Other | Unknown => "🌍",
        }
    }

    /// KR / GLOBAL 필터 분류.
    pub fn is_global(self) -> bool {
        !matches!(self, Country::Kr | Country::Unknown)
    }
}
