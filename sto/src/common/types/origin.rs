use crate::common::*;

/// 데이터 원천. 어떤 종류의 출처에서 수집된 정보인지 일반화.
/// 특정 발행사 (예: Musicow) 의 자체 공시 페이지는 `Company` 로 분류 — 발행사가 늘어나도
/// enum variant 를 추가하지 않는다.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum Origin {
    #[default]
    Unknown,
    /// 전자공시시스템 (DART)
    Dart,
    /// 발행사 자체 공시 페이지 / 공식 안내문
    Company,
    /// 금융 규제기관 (FSC 등) 지정/인가 정보
    Regulator,
}
