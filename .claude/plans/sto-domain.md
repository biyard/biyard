# STO 정보 플랫폼 — 데이터 모델 & 앱 구조 설계 (v0.3)

> 상태: **승인됨 — Phase 1 진행 가능**
> 작성: 2026-05-12 (v0.3 갱신)
> 범위: 현재 수집된 데이터를 localstack DynamoDB 에 적재하고, 신규 `sto/` Dioxus
> fullstack 앱으로 mock 사이트 기능을 실 서비스로 구현.
> **데이터 갱신·동기화 정책은 별도 작업** (이 문서 범위 외).

---

## 1. 확정된 결정 사항

| 항목 | 결정 |
|---|---|
| 앱 분리 | `console/` 옆에 신규 `sto/` Dioxus fullstack 앱 |
| 모델 위치 | `sto/src/features/*/models/` (공유 crate 없음 — console 패턴) |
| 테이블 | `biyard-{env}-sto` (console 메인 테이블과 분리) |
| 환경 | 1차 localstack 만 |
| 단일 테이블 설계 | O (sto 전용 테이블 안에서 단일 테이블 + GSI) |
| Musicow 472곡 | STO 와 동일 무게로 적재 |
| ID 규칙 | UUID v7 (`uuid::Uuid::now_v7()`) — console 표준 |
| `DynamoEntity` 매크로 | console 과 동일하게 사용 |
| `Partition` / `EntityType` enum | sto 앱 내부에 별도 정의 (console 과 독립) |
| 테이블 생성 + 시드 | **shell script** (`scripts/sto-init-entrypoint.sh`) — console 패턴 100% 일치 |
| 시드 데이터 생성 | Python 변환 스크립트 (`tools/build-sto-seed.py`) — 캐시 → DynamoDB JSON |
| 데이터 갱신 | 이번 범위 제외 |

---

## 2. 모노레포 구조

```
biyard/
├── console/                          # 기존 — B2B 콘솔
├── landing/                          # 기존 — 마케팅
├── sto/                              # 신규 — STO 정보 플랫폼
│   ├── Cargo.toml
│   ├── Dioxus.toml
│   ├── tailwind.css
│   ├── assets/
│   ├── public/
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── route.rs
│       ├── common/                   # run, Partition, EntityType, CommonConfig
│       │   └── types/
│       │       ├── partition.rs
│       │       └── entity_type.rs
│       └── features/
│           ├── catalog/              # 카탈로그·목록
│           │   ├── controllers/
│           │   ├── components/
│           │   ├── views/
│           │   ├── models/           # ★ Sto DynamoEntity
│           │   ├── dto/
│           │   ├── types/
│           │   ├── i18n.rs
│           │   └── mod.rs
│           ├── detail/               # STO 상세
│           ├── issuers/              # 발행사
│           │   └── models/           # ★ Issuer
│           ├── filings/              # 공시·PDF
│           │   └── models/           # ★ Filing
│           ├── biyard_index/         # 평가지표
│           ├── launchpad/            # 런치패드 소개
│           └── news/
├── packages/
│   ├── dart-collector/               # 기존
│   ├── api-doc-macros/               # 기존
│   ├── api-doc-types/                # 기존
│   └── console-interop/              # 기존
├── scripts/
│   ├── localstack-init-entrypoint.sh # 기존 (console)
│   ├── dynamodb-table.json           # 기존 (console)
│   ├── sto-init-entrypoint.sh        # 신규 — sto 전용 init
│   ├── sto-table.json                # 신규 — sto 테이블 스키마
│   └── sto-seed-data.json            # 신규 — 시드 데이터 (BatchWriteItem 입력)
├── tools/
│   └── build-sto-seed.py             # 신규 — 캐시 → seed JSON 변환기 (1회성)
├── docker-compose.yaml               # 기존 + sto-localstack-init 컨테이너 추가
└── Cargo.toml                        # workspace members: ["console", "landing", "sto"]
```

---

## 3. 데이터 모수

| 출처 | 건수 | 디테일 |
|---|---|---|
| DART 사이클 (5사 + 아티피오) | 32 | 본문 파싱 완료 |
| DART 원본 공시 | 197 | rcept_no·report_nm·flr_nm |
| Musicow 카탈로그 | 472 | 곡 메타 + 공통 발행 구조 |
| Musicow PDF 디테일 | 5 | 발행 좌수·단가·총액·신탁기간 |
| 기존 보도자료 STO | 22 | data.js 수동 큐레이션 |
| Issuer | 16 | 발행사 |

→ **DynamoDB 적재 모수: 약 510 Sto + 210 Filing + 16 Issuer ≈ 740 items**

---

## 4. 엔티티 정의 — console 패턴 적용

테이블명: `biyard-{env}-sto` (예: `biyard-local-sto`)

### 4.1 공통 — `sto/src/common/types/partition.rs`

```rust
use crate::common::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "id")]
pub enum Partition {
    Sto(String),       // "STO#{uuid}"
    Issuer(String),    // "ISSUER#{slug}"
    // 필요시 추가
}

// DynamoEntity 매크로가 자동으로 "STO#" prefix 처리하도록 구성
```

### 4.2 공통 — `sto/src/common/types/entity_type.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Sto,        // SK = "STO"
    Issuer,     // SK = "ISSUER"
    Filing,     // SK 는 "FILING#{filing_id}" 형태 — 별도 SK 직접 구성
}
```

→ Filing 은 SK 가 동적이라서 console 의 일부 엔티티처럼 `pub sk: String` 으로 갈지,
   별도 enum variant 로 갈지는 구현 단계에서 결정.

### 4.3 `Sto` — `sto/src/features/catalog/models/sto.rs`

```rust
use crate::common::{Deserialize, DynamoEntity, EntityType, Partition, Result, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Sto {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    pub underlying: Option<String>,
    pub category: String,        // real_estate | art | music | livestock | luxury | content | infra

    #[dynamo(index = "gsi1", pk, name = "find_by_status")]
    pub status: String,          // 신고중 | 발행완료 | 철회 | 청산완료

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi3", sk)]
    pub issued_at: String,       // YYYY-MM-DD

    #[dynamo(index = "gsi2", pk, name = "find_by_region_category")]
    pub region_category: String, // KR#art (compound key)

    #[dynamo(index = "gsi3", pk, name = "find_by_issuer_id")]
    pub issuer_id: String,       // slug

    pub region: String,          // KR | GLOBAL
    pub country: String,
    pub security_type: String,
    pub classification: String,
    pub origin: String,          // DART | MUSICOW | MANUAL | PRESS
    pub external_id: Option<String>,
    pub external_url: Option<String>,

    pub offering: Option<Offering>,
    pub issuance_structure: Option<IssuanceStructure>,

    pub artist: Option<String>,
    pub rights_category: Option<String>,
    pub trust_no: Option<String>,
    pub year: Option<String>,

    pub sources: Vec<SourceRef>,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Offering {
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub unit_price: Option<i64>,
    pub total_units: Option<i64>,
    pub subscription_start: Option<String>,
    pub subscription_end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IssuanceStructure {
    pub issuer: Option<String>,
    pub trustee: Option<String>,
    pub trustee_role: Option<String>,
    pub underwriter: Option<String>,
    pub custody: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceRef {
    pub src: String,
    pub label: String,
}
```

생성된 액세스 패턴 (DynamoEntity 매크로가 만들어줌):

```
Sto::get(cli, pk, sk)
Sto::find_by_status(cli, status, ...)      → GSI1
Sto::find_by_region_category(cli, "KR#art", ...) → GSI2
Sto::find_by_issuer_id(cli, "stockeeper", ...)   → GSI3
```

### 4.4 `Filing` — `sto/src/features/filings/models/filing.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Filing {
    pub pk: Partition,           // STO#{sto_id}
    pub sk: String,              // FILING#{filing_id}

    pub filing_id: String,
    pub filing_source: String,   // DART | MUSICOW
    pub filing_type: String,
    pub title: String,
    pub filed_at: String,
    pub url: Option<String>,
    pub attachments: Vec<FilingAttachment>,
    pub rcept_no: Option<String>,
    pub parsed: Option<serde_json::Value>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilingAttachment {
    pub name: String,
    pub url: String,
    pub size_bytes: Option<i64>,
}
```

### 4.5 `Issuer` — `sto/src/features/issuers/models/issuer.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Issuer {
    pub pk: Partition,           // ISSUER#{slug}
    pub sk: EntityType,          // ISSUER

    pub name: String,
    pub region: String,
    pub country: String,
    pub category: String,
    pub description: String,
    pub status: String,
    pub sandbox: Option<String>,
    pub chain: Option<String>,
    pub website: Option<String>,
    pub sources: Vec<SourceRef>,

    pub created_at: i64,
    pub updated_at: i64,
}
```

→ Issuer 는 GSI 따로 안 만들고 *Sto.find_by_issuer_id (GSI3)* 로 역참조.

---

## 5. PK/SK + GSI 패턴 요약

```
Sto 단건:        PK=STO#{id}, SK=STO
Sto 자료 전부:   PK=STO#{id}, SK begins_with ""    (STO + FILING#* 다)
공시 단건:       PK=STO#{id}, SK=FILING#{filing_id}
발행사 단건:     PK=ISSUER#{slug}, SK=ISSUER
상태별 Sto 목록: GSI1 — gsi1_pk=STATUS#{status}
지역+카테고리:   GSI2 — gsi2_pk=CAT#KR#art
발행사 Sto 목록: GSI3 — gsi3_pk=ISSUER#{slug}
```

GSI 가 console 처럼 *gsi1~gsi5* 가속화 슬롯을 미리 정의해두는 패턴이지만,
sto 테이블은 *처음에는 GSI 3개* 만 활성화 (필요할 때 추가).

---

## 6. 출처별 매핑 (Python 변환 스크립트 동작)

`tools/build-sto-seed.py` 가 다음을 읽어서 `scripts/sto-seed-data.json` 생성:

```
입력:
  /tmp/dart-cache/raw/parsed/_cycles.json           # 32 사이클
  /tmp/dart-cache/raw/parsed/_filings_parsed.json   # DART 공시 파싱
  sto-mockup/dart-viewer-data.json                  # filings_raw 197건
  sto-mockup/musicow-catalog.json                   # 472곡 + 5건 디테일
  sto-mockup/musicow-pdf-details.json               # PDF 파싱
  sto-mockup/mock-db.json                           # 22 보도자료 STO + Issuer

출력:
  scripts/sto-seed-data.json
  형식: BatchWriteItem 입력 — { "biyard-local-sto": [ { "PutRequest": { "Item": {...} } }, ... ] }
  주의: BatchWriteItem 은 한 번에 25건 제한 → init script 가 chunk 처리
```

ID 부여:
- 멱등 키 `(origin, external_id)` 기준으로 *deterministic UUID v7* 생성 (또는 외부 키 그대로 슬러그 사용 가능)
- *재실행 시 같은 UUID 유지* 위해, 변환 스크립트 자체에서 캐싱 또는 hash 기반 결정

---

## 7. Issuer 16개 시드

| issuer_id (slug) | name | category | origin |
|---|---|---|---|
| stockeeper | 스탁키퍼 (뱅카우) | livestock | DART |
| datagen | 데이터젠 (핀돈) | livestock | DART |
| togetherart | 투게더아트 | art | DART |
| yeolmae | 열매컴퍼니 | art | DART |
| seoulauctionblue | 서울옥션블루 | art | DART |
| artipio | 아티피오 | art | DART |
| kasa | 카사 | real_estate | PRESS |
| lucentblock | 루센트블록 (소유) | real_estate | PRESS |
| funble | 펀블 | real_estate | PRESS (사업 종료) |
| musicow | 뮤직카우 | music | MUSICOW |
| masterworks·royal·wavist·konvi·securitize·anote 등 | 해외 7개 | various | PRESS |

---

## 8. localstack init 통합 — console 패턴 일치

### 8.1 `scripts/sto-table.json`

console 의 `dynamodb-table.json` 과 같은 형식. PK/SK + GSI 3개 정의.

```json
{
  "TableName": "biyard-local-sto",
  "AttributeDefinitions": [
    { "AttributeName": "pk", "AttributeType": "S" },
    { "AttributeName": "sk", "AttributeType": "S" },
    { "AttributeName": "gsi1_pk", "AttributeType": "S" },
    { "AttributeName": "gsi1_sk", "AttributeType": "S" },
    { "AttributeName": "gsi2_pk", "AttributeType": "S" },
    { "AttributeName": "gsi2_sk", "AttributeType": "S" },
    { "AttributeName": "gsi3_pk", "AttributeType": "S" },
    { "AttributeName": "gsi3_sk", "AttributeType": "S" }
  ],
  "KeySchema": [
    { "AttributeName": "pk", "KeyType": "HASH" },
    { "AttributeName": "sk", "KeyType": "RANGE" }
  ],
  "GlobalSecondaryIndexes": [
    { "IndexName": "gsi1", "KeySchema": [...], "Projection": { "ProjectionType": "ALL" } },
    { "IndexName": "gsi2", ... },
    { "IndexName": "gsi3", ... }
  ],
  "BillingMode": "PAY_PER_REQUEST"
}
```

### 8.2 `scripts/sto-init-entrypoint.sh`

```bash
#!/bin/bash
set -e
yum update -y && yum install -y awscli jq

DYNAMO_ENDPOINT=${DYNAMO_ENDPOINT:-http://localhost:4566}

until aws dynamodb --endpoint-url=$DYNAMO_ENDPOINT list-tables >/dev/null 2>&1; do
  sleep 2
done

# 1. 테이블 생성
echo "Creating biyard-local-sto table..."
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb create-table \
    --cli-input-json file://./scripts/sto-table.json >/dev/null 2>&1 || echo "Table exists"

# 2. 시드 데이터 25건씩 chunk 로 BatchWriteItem
echo "Seeding STOs / Filings / Issuers..."
TOTAL=$(jq '.["biyard-local-sto"] | length' ./scripts/sto-seed-data.json)
echo "Total items: $TOTAL"

for i in $(seq 0 25 $TOTAL); do
  CHUNK=$(jq --argjson start $i '{"biyard-local-sto": .["biyard-local-sto"][$start:$start+25]}' ./scripts/sto-seed-data.json)
  echo "$CHUNK" > /tmp/chunk.json
  aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb batch-write-item \
      --request-items file:///tmp/chunk.json >/dev/null 2>&1 || echo "  chunk $i failed"
done

echo "Seeding complete."
```

### 8.3 `docker-compose.yaml` 추가 컨테이너

```yaml
  sto-localstack-init:
    image: amazonlinux:2
    depends_on:
      localstack:
        condition: service_healthy
    environment:
      DYNAMO_ENDPOINT: "http://localstack:4566"
      AWS_ACCESS_KEY_ID: "test"
      AWS_SECRET_ACCESS_KEY: "test"
      AWS_REGION: "us-east-1"
    volumes:
      - ./scripts:/scripts
    working_dir: /
    command: ["bash", "/scripts/sto-init-entrypoint.sh"]
    networks:
      - biyard-network
```

→ console 의 `localstack-init` 과 *완전히 같은 패턴*. 한 docker-compose 안에서 두 init 컨테이너가 동시 기동.

---

## 9. 작업 분해

### Phase 0 — 설계 승인 ✅ (완료)

### Phase 1 — sto 앱 스켈레톤
- [ ] `sto/` 디렉토리 + Cargo.toml + Dioxus.toml + tailwind.css
- [ ] workspace `Cargo.toml` members 에 `"sto"` 추가
- [ ] `sto/src/main.rs` + `lib.rs` + `route.rs` (빈 라우터)
- [ ] `sto/src/common/run.rs` + DynamoDB client
- [ ] `cargo build -p sto` 통과

### Phase 2 — 공통 타입 + 엔티티
- [ ] `sto/src/common/types/partition.rs` (Sto, Issuer variant)
- [ ] `sto/src/common/types/entity_type.rs`
- [ ] `sto/src/features/catalog/models/sto.rs` — Sto DynamoEntity
- [ ] `sto/src/features/filings/models/filing.rs` — Filing
- [ ] `sto/src/features/issuers/models/issuer.rs` — Issuer
- [ ] `cargo build` 통과

### Phase 3 — 시드 데이터 생성 (Python)
- [ ] `tools/build-sto-seed.py`
  - 입력: 캐시 파일 5종
  - 출력: `scripts/sto-seed-data.json` (BatchWriteItem 포맷)
  - 멱등 ID 부여 (output 에 origin+external_id → uuid 매핑 기록)
- [ ] 검증: jq 로 item 수 확인 (~740)

### Phase 4 — 테이블 스키마 + init script
- [ ] `scripts/sto-table.json`
- [ ] `scripts/sto-init-entrypoint.sh`
- [ ] `docker-compose.yaml` 에 `sto-localstack-init` 컨테이너 추가

### Phase 5 — localstack 검증
- [ ] `docker compose up -d`
- [ ] 테이블 생성 + 시드 완료 로그 확인
- [ ] DynamoDB Admin UI 또는 awslocal 로 *item 수·GSI 쿼리* 검증

### Phase 6 — 백엔드 API
- [ ] `sto/src/features/catalog/controllers/list_stos.rs` — `GET /v1/stos?category=&region=&status=&bookmark=`
- [ ] `sto/src/features/catalog/controllers/get_sto.rs` — `GET /v1/stos/:sto_id`
- [ ] `sto/src/features/issuers/controllers/list_issuers.rs`
- [ ] `sto/src/features/issuers/controllers/get_issuer.rs`
- [ ] `sto/src/features/filings/controllers/list_filings.rs`

### Phase 7 — Dioxus 뷰 이식
- 카탈로그 → 상세 → 발행사 → 평가지표 → 런치패드 → 뉴스 → 가격
- 각 화면별 PR 분리 권장

### Phase 8 — i18n + 마무리

---

## 10. 범위 외 (별도 작업)

- DART 일일 폴링 → DynamoDB (dart-collector Lambda 갱신)
- Musicow 신규 곡 추적
- 상태 변경 이벤트
- 인증·권한
- B2B 라이선스
- Biyard Index 평가 결과 저장
- 검색 인덱스 (OpenSearch 등)
- 배포 인프라 (CDK)

---

## 11. 진행 시작 시점 메모

승인 시 시작 순서:
1. Phase 1 (앱 스켈레톤) — 가장 가벼움
2. Phase 2 (엔티티) — 모델만 정의
3. Phase 3 (Python seeder) — 시드 JSON 생성
4. Phase 4 (shell init) — console 패턴 적용
5. Phase 5 (검증) — 실제 데이터가 들어가는지

Phase 1~5 까지가 *데이터 적재 + 검증* 단계.
Phase 6 부터는 *백엔드 + 프론트* 본격 작업.
