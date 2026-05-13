//! 같은 `pk = STO#{uuid}` 안에 sk 가 다른 여러 entity (`STO`, `STO_META#*`, `FILING#*`) 가
//! 섞여 있어, 한 번의 Query 로 가져온 결과를 sk 종류에 따라 자동 분기해야 한다.
//!
//! `#[derive(DynamoEntity)]` 를 enum 에 붙이면 `pub async fn query(cli, pk)` 가 자동 생성되고
//! `#[serde(untagged)]` 로 `serde_dynamo::from_item` 이 각 row 의 attribute 집합에 따라
//! 적절한 variant 로 dispatch 한다.

use crate::common::*;
use crate::features::catalog::models::{
    Sto, StoMetaArt, StoMetaLivestock, StoMetaMusic, StoMetaRealEstate,
};
use crate::features::filings::Filing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity)]
#[dynamo(table = "sto")]
#[serde(untagged)]
pub enum StoPartitionRow {
    Sto(Sto),
    MetaMusic(StoMetaMusic),
    MetaArt(StoMetaArt),
    MetaRealEstate(StoMetaRealEstate),
    MetaLivestock(StoMetaLivestock),
    Filing(Filing),
}
