use crate::common::*;

#[derive(
    Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum EntityType {
    #[default]
    None,

    // Timestamp prefix (SK 범위 쿼리용)
    TS(String),

    // STO 공통 메타데이터
    Sto,

    // STO 카테고리별 부가 메타 (예: STO_META#MUSIC)
    // pk 는 STO#{uuid} 와 동일하게 묶여서 Query 1번으로 함께 조회.
    StoMeta(String),

    // 발행사 메타
    Issuer,

    // 공시 (PDF 첨부 포함)
    Filing(String),

    // 집계 row (pk = AGGREGATE, sk = AGGREGATE#{TYPE})
    Aggregate(String),
}
