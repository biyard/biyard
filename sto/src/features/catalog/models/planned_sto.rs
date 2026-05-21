use crate::common::*;

/// 발행사·증권사가 사이트에 직접 등록한 "예상 공모" row.
///
/// - DART 신고서 제출 이전 단계의 마케팅성 정보.
/// - `pk = PLANNED`, `sk = PLANNED#{id}` 로 정상 STO 와 완전히 분리해서 보관.
/// - 신고서 제출되어 정식 `Sto` 로 승급되면 archive 하고 더 이상 노출하지 않는다 (운영 정책).
/// - 청약 기간이 확정되지 않은 단계라 `expected_window` 는 자유 텍스트.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct PlannedSto {
    pub pk: Partition,

    #[dynamo(index = "gsi4", pk, name = "find_all")]
    pub sk: EntityType,

    /// 표시 ID — `pk` 와 동일한 값 (planned-{slug} 같은 결정적 식별자).
    #[serde(default)]
    pub planned_id: String,

    pub name: String,
    pub category: Category,
    pub country: Country,

    /// 발행사 slug (Issuer row 와 연결).
    pub issuer_id: String,
    /// 발행사 표시명 (Issuer 메타에서 denormalize).
    #[serde(default)]
    pub issuer_name: Option<String>,

    /// 증권사 (인수/계좌관리/중개 등). DART 정식 신고 이전이라 변경될 수 있음.
    #[serde(default)]
    pub broker: Option<String>,

    /// 증권사의 역할 표시 (인수 / 계좌관리 / 중개).
    #[serde(default)]
    pub broker_role: Option<String>,

    /// 예상 모집액 (원). 확정 아님.
    #[serde(default)]
    pub expected_amount: Option<i64>,

    /// 예상 청약 시기 — "2026년 5월 중순", "5월 셋째 주" 같은 자유 텍스트.
    /// 정확한 날짜가 정해지지 않은 단계라 String 으로 유지.
    #[serde(default)]
    pub expected_window: Option<String>,

    /// 정렬용 — 등록 시각 (epoch ms). GSI sort key 로도 활용.
    #[dynamo(index = "gsi4", sk, prefix = "TS")]
    pub registered_at: i64,

    /// 정보 등록 주체 — 발행사 slug 또는 운영자 식별자.
    #[serde(default)]
    pub registered_by: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}
