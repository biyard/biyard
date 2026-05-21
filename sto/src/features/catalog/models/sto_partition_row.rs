//! 같은 `pk = STO#{uuid}` 안에 sk 가 다른 여러 entity (`STO`, `STO_META#*`, `FILING#*`) 가
//! 섞여 있어, 한 번의 Query 로 가져온 결과를 sk 종류에 따라 자동 분기해야 한다.
//!
//! `#[derive(DynamoEntity)]` 를 enum 에 붙이면 `pub async fn query(cli, pk)` 가 자동 생성되고,
//! custom Deserialize 로 `sk` 값을 먼저 보고 올바른 variant 로 dispatch 한다.

use crate::common::*;
use crate::features::catalog::models::{
    Sto, StoMetaArt, StoMetaLivestock, StoMetaMusic, StoMetaRealEstate,
};
use crate::features::filings::Filing;

#[derive(Debug, Clone, PartialEq, DynamoEntity)]
#[dynamo(table = "sto")]
pub enum StoPartitionRow {
    Sto(Sto),
    MetaMusic(StoMetaMusic),
    MetaArt(StoMetaArt),
    MetaRealEstate(StoMetaRealEstate),
    MetaLivestock(StoMetaLivestock),
    Filing(Filing),
}

impl serde::Serialize for StoPartitionRow {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sto(v) => v.serialize(s),
            Self::MetaMusic(v) => v.serialize(s),
            Self::MetaArt(v) => v.serialize(s),
            Self::MetaRealEstate(v) => v.serialize(s),
            Self::MetaLivestock(v) => v.serialize(s),
            Self::Filing(v) => v.serialize(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for StoPartitionRow {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let map = serde_json::Value::deserialize(d)?;
        let sk = map
            .get("sk")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if sk.starts_with("STO_META#MUSIC") || sk.starts_with("stoMeta#MUSIC") {
            serde_json::from_value::<StoMetaMusic>(map)
                .map(Self::MetaMusic)
                .map_err(D::Error::custom)
        } else if sk.starts_with("STO_META#ART") || sk.starts_with("stoMeta#ART") {
            serde_json::from_value::<StoMetaArt>(map)
                .map(Self::MetaArt)
                .map_err(D::Error::custom)
        } else if sk.starts_with("STO_META#REAL_ESTATE") || sk.starts_with("stoMeta#REAL_ESTATE") {
            serde_json::from_value::<StoMetaRealEstate>(map)
                .map(Self::MetaRealEstate)
                .map_err(D::Error::custom)
        } else if sk.starts_with("STO_META#LIVESTOCK") || sk.starts_with("stoMeta#LIVESTOCK") {
            serde_json::from_value::<StoMetaLivestock>(map)
                .map(Self::MetaLivestock)
                .map_err(D::Error::custom)
        } else if sk.starts_with("FILING#") || sk.starts_with("filing#") {
            serde_json::from_value::<Filing>(map)
                .map(Self::Filing)
                .map_err(D::Error::custom)
        } else {
            serde_json::from_value::<Sto>(map)
                .map(Self::Sto)
                .map_err(D::Error::custom)
        }
    }
}
