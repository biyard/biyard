use crate::common::*;

#[derive(
    Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum EntityType {
    #[default]
    None,

    // Timestamp prefix (SK 범위 쿼리용)
    TS(String),

    // STO 메타데이터
    Sto,

    // 발행사 메타
    Issuer,

    // 공시 (PDF 첨부 포함)
    Filing(String),
}
