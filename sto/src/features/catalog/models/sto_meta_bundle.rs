use super::{StoMetaArt, StoMetaLivestock, StoMetaMusic, StoMetaRealEstate};

/// 한 STO 에 묶인 카테고리별 메타 row 들. 동일 `pk = STO#{uuid}` 를 Query 한 뒤
/// `sk` prefix (`STO_META#MUSIC` 등) 로 분기해 채움. 카테고리당 1개만 채워짐.
#[derive(Debug, Clone, Default)]
pub struct StoMetaBundle {
    pub music: Option<StoMetaMusic>,
    pub art: Option<StoMetaArt>,
    pub real_estate: Option<StoMetaRealEstate>,
    pub livestock: Option<StoMetaLivestock>,
}
