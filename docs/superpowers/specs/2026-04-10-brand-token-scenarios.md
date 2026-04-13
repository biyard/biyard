# Brand Token & Treasury: Scenario Guide

**Date:** 2026-04-10

## System Overview

```
                          +-----------+
                          | Multisig  |
                          | (DAO)   |
                          +-----+-----+
                                |
                    propose / approve / execute
                                |
          +---------------------+---------------------+
          |                                           |
    +-----+------+                            +-------+-------+
    | BrandToken |                            |   Treasury    |
    | (ERC-20)   |<------ buyback ----------->| (USDT 보관)   |
    +-----+------+                            +-------+-------+
          |                                           |
    triggerMonthlyMint                            deposit
    claim (서명 기반)                          getPrice / buyback
          |                                           |
    +-----+------+                            +-------+-------+
    |   유저들    |                            | 브랜드 매출   |
    | (포인트→토큰) |                           | (USDT 유입)  |
    +------------+                            +---------------+
```

## 핵심 공식

```
토큰 가격 = Treasury USDT 잔고 / 유통량
유통량   = totalSupply - BrandToken 컨트랙트 잔고(claim pool) - Treasury 보유 BrandToken
```

- Treasury에 USDT가 쌓일수록 → 가격 상승
- Treasury가 buyback으로 토큰 회수할수록 → 유통량 감소 → 가격 상승
- 월별 민팅으로 토큰 발행될수록 → 유통량 증가 → 가격 하락 압력
- claim pool에서 유저가 토큰을 수령할수록 → 유통량 증가 → 가격 하락 압력

> **주의:** 여기서 "가격"은 현재 유통분 기준 현물가이지, 장기 보장 가격이 아닙니다. Claim pool이 전부 유통되면 가격은 `Treasury USDT / totalSupply` 수준까지 내려갈 수 있습니다. 실제 가격은 매출 유입(Treasury 증가) 속도와 claim(유통량 증가) 속도의 균형에 의해 결정됩니다.

---

## Scenario 1: 브랜드 토큰 최초 배포

**주체:** 브랜드 운영팀 (Biyard PaaS 통해)

### 전제 조건

- 브랜드가 Biyard 콘솔에서 토큰 파라미터를 설정 완료

### 입력값 (예시)

| 파라미터           | 값                                      | 설명                 |
| ------------------ | --------------------------------------- | -------------------- |
| name               | "CafeToken"                             | 토큰 이름            |
| symbol             | "CAFE"                                  | 토큰 심볼            |
| monthlyEmission    | 1,000,000                               | 첫 달 발행량         |
| decayRateBps       | 500                                     | 월 5% 감소           |
| distributionSlots  | [(마케팅지갑, 1000), (파트너지갑, 500)] | 이벤트 물량 15%      |
| stableToken        | USDT 주소                               | Treasury가 받을 코인 |
| multisig signers   | [CEO, CTO, CFO]                         | 초기 서명자          |
| multisig threshold | 2                                       | 2-of-3               |

### 플로우

```
1. BUSDT 배포 (테스트넷만)
2. Multisig 배포 → signers: [CEO, CTO, CFO], threshold: 2
3. BrandToken 배포 → monthlyEmission: 1M, decay: 5%
4. Treasury 배포 → stableToken: BUSDT, brandToken: CAFE
5. BrandToken에 treasury 주소 설정
6. BrandToken 소유권을 Multisig으로 이전
```

### 결과 상태

| 항목             | 값                        |
| ---------------- | ------------------------- |
| CAFE totalSupply | 0 (아직 민팅 전)          |
| Treasury USDT    | 0                         |
| maxSupply        | ~19,078,604 (60개월 합산, 월 5% 감소 기하급수 합) |
| 토큰 가격        | 0 (유통량 0)              |

---

## Scenario 2: 첫 달 토큰 발행 (triggerMonthlyMint)

**주체:** 누구든 호출 가능 (보통 Biyard 서버가 월초에 자동 호출)

### 전제 조건

- 컨트랙트 배포 후 30일 경과 (또는 첫 month epoch 도달)

### 플로우

```
1. 누군가 BrandToken.triggerMonthlyMint() 호출
2. 컨트랙트 계산:
   - Month 1 ceiling = 1,000,000 CAFE
   - 마케팅지갑: 1,000,000 × 10% = 100,000 CAFE → 마케팅지갑으로 민팅
   - 파트너지갑: 1,000,000 × 5% = 50,000 CAFE → 파트너지갑으로 민팅
   - Claim pool: 1,000,000 × 85% = 850,000 CAFE → BrandToken 컨트랙트에 보관
3. 같은 달에 다시 호출하면 revert
```

### 결과 상태

| 항목                       | 값                       |
| -------------------------- | ------------------------ |
| CAFE totalSupply           | 1,000,000                |
| 마케팅지갑 잔고            | 100,000 CAFE             |
| 파트너지갑 잔고            | 50,000 CAFE              |
| Claim pool (컨트랙트 잔고) | 850,000 CAFE             |
| Treasury USDT              | 0 (아직 매출 없음)       |
| 토큰 가격                  | 0 (Treasury에 USDT 없음) |

---

## Scenario 3: 브랜드 매출 발생 → Treasury 입금

**주체:** 브랜드 (매출의 일부를 Treasury에 입금)

### 전제 조건

- 브랜드가 매월 매출의 reserve rate만큼 USDT를 Treasury에 입금하기로 설정 (예: 10%)
- 월 매출 1,000만원 (≈ 6,667 USDT at 1,500원/USDT)

### 플로우

```
1. 브랜드 → USDT approve(Treasury, 667 USDT)  // 매출의 10%
2. 브랜드 → Treasury.deposit(667 USDT)
3. Treasury에 667 USDT 적립
```

### 결과 상태

| 항목          | 값                                                       |
| ------------- | -------------------------------------------------------- |
| Treasury USDT | 667                                                      |
| 유통량        | 1,000,000 (totalSupply) - 850,000 (claim pool) = 150,000 |
| 토큰 가격     | 667 / 150,000 = 0.00445 USDT                             |

> 참고: 유통량 = totalSupply - BrandToken 컨트랙트 잔고(claim pool) - Treasury 보유 CAFE

---

## Scenario 4: 유저가 Point → Token 전환 (Claim)

**주체:** 유저 (브랜드 앱에서 포인트 적립 후 토큰으로 전환)

### 전제 조건

- 유저가 브랜드 앱에서 10,000 포인트 적립
- 1 포인트 = 1 토큰 전환 비율 (브랜드가 설정)
- Biyard 서버에 전환 요청

### 플로우

```
1. 유저 → Biyard 서버: "10,000 포인트를 CAFE로 전환해주세요"
2. Biyard 서버 검증:
   - 유저 포인트 잔고 확인
   - Claim pool 잔고 확인 (850,000 CAFE 남아있음)
   - 유저 포인트를 "pending" 상태로 전환 (즉시 차감하지 않음)
3. Biyard 서버 → EIP-712 서명 생성:
   - to: 유저 지갑
   - amount: 10,000 CAFE
   - nonce: 고유값
   - deadline: 1시간 후
4. 유저에게 서명 전달
5. 유저 → BrandToken.claim(10000, nonce, deadline, signature)
   - 유저가 직접 트랜잭션 전송 (가스비 유저 부담)
   - 서명 검증 통과
   - BrandToken 컨트랙트에서 유저로 10,000 CAFE 전송
6. Biyard 서버 → Claimed 이벤트 감지 → 포인트 확정 차감
```

> **실패 정산:** 유저가 온체인 트랜잭션을 보내지 않거나, 실패하거나, deadline이 만료된 경우:
> - 서버는 Claimed 이벤트가 없으므로 포인트를 "pending → 복원" 처리
> - 서명은 deadline 만료 후 사용 불가하므로 컨트랙트 측 리스크 없음
> - nonce는 미사용 상태로 남음 (재발급 가능)

### 결과 상태

| 항목           | 변경                         |
| -------------- | ---------------------------- |
| 유저 CAFE 잔고 | 0 → 10,000                   |
| Claim pool     | 850,000 → 840,000            |
| totalSupply    | 변동 없음 (1,000,000)        |
| 유통량         | 150,000 → 160,000            |
| 토큰 가격      | 667 / 160,000 = 0.00417 USDT |

> 유통량 증가 → 가격 미세 하락. 이건 정상적인 인플레이션.

---

## Scenario 5: 유저가 Token → USDT 교환 (Buyback)

**주체:** 유저 (토큰을 USDT로 현금화)

### 전제 조건

- 유저가 10,000 CAFE 보유
- 현재 토큰 가격: 0.00417 USDT

### 플로우

```
1. 유저 → BrandToken.approve(Treasury, 10000)
2. 유저 → Treasury.buyback(10000)
3. Treasury 계산:
   - usdtOut = 10,000 × 0.00417 = 41.7 USDT
4. 유저에게서 10,000 CAFE → Treasury로 이동 (burn 아님, 보관)
5. Treasury에서 유저에게 41.7 USDT 전송
```

### 결과 상태

| 항목          | 변경                                                    |
| ------------- | ------------------------------------------------------- |
| 유저 CAFE     | 10,000 → 0                                              |
| 유저 USDT     | 0 → 41.7                                                |
| Treasury USDT | 667 → 625.3                                             |
| Treasury CAFE | 0 → 10,000                                              |
| 유통량        | 160,000 → 150,000 (Treasury가 보유하면 유통량에서 제외) |
| 토큰 가격     | 625.3 / 150,000 = 0.00417 USDT                          |

> 가격이 동일하게 유지됨! Buyback은 USDT와 유통량을 같은 비율로 줄이기 때문.
> **단, Treasury가 회수한 토큰을 재유통(withdrawToken)하면 유통량이 다시 늘어나 가격이 내려갑니다.**
> Treasury에 쌓인 10,000 CAFE는 Multisig에서 소각/재분배/유동성 제공 등을 결정할 수 있지만, 재유통 시 가격 영향을 고려해야 합니다.

---

## Scenario 6: 시간 경과 — 매출 성장과 가격 상승

**주체:** 시스템 (월별 자동 반복)

### 전제 조건

- 월 매출 10% 성장, reserve rate 10%
- 월별 토큰 발행 5% 감소

### 12개월 시뮬레이션

| 월  | 매출(만원) | Treasury 입금(USDT) | 누적 Treasury | 신규 발행 | 누적 유통량 | 토큰 가격 |
| :-: | :--------: | :-----------------: | :-----------: | :-------: | :---------: | :-------: |
|  1  |   1,000    |         667         |      667      |  150,000  |   150,000   |  0.0044   |
|  2  |   1,100    |         733         |     1,400     |  142,500  |   292,500   |  0.0048   |
|  3  |   1,210    |         807         |     2,207     |  135,375  |   427,875   |  0.0052   |
|  6  |   1,611    |        1,074        |     5,144     |  116,027  |   792,164   |  0.0065   |
| 12  |   2,594    |        1,729        |    14,256     |  88,025   |  1,416,420  |  0.0101   |

> 매출 성장률 > 토큰 발행 감소율이면 가격이 지속 상승.
> 이것이 콘솔의 Floor Price Simulator가 보여주는 시나리오와 동일.
>
> **주의: 이 시뮬레이션은 claim 속도(유저의 Point→Token 전환)를 0으로 가정한 최선 시나리오입니다.**
> 실제로는 claim pool에서 유저가 토큰을 수령할수록 유통량이 증가하여 가격 하락 압력이 생깁니다.
> "신규 발행" 칼럼의 유통량은 distribution slot(15%)에 해당하는 즉시 유통분만 반영한 것이며,
> claim pool(85%)이 전부 유통되면 가격은 더 낮아집니다.

---

## Scenario 7: Multisig — 발행량 변경 제안

**주체:** Owner (Biyard가 Multisig 통해 실행)

### 전제 조건

- 현재 monthlyEmission: 1,000,000
- 수요 대비 공급이 과다 → 발행량 줄이고 싶음
- Multisig: 1-of-1 (Biyard) — 현 단계에서는 관리자 키 역할, 향후 멀티시그/DAO로 전환 가능

### 플로우

```
1. Biyard → Multisig.propose(
     target: BrandToken,
     data: setMonthlyEmission(500000).encode(),
     value: 0
   )
   → proposalId: 1 반환, threshold 1이므로 즉시 실행 가능

2. Biyard → Multisig.approve(1)
3. Biyard → Multisig.execute(1)
   → Multisig이 BrandToken.setMonthlyEmission(500000) 호출
   → 다음 달부터 월 발행량 500,000으로 변경
```

### 결과

- 다음 달 ceiling: `500,000 × (1 - 5%)^month` (month = 배포 시점 기준, 변경 시점 아님)
- 예: 배포 후 6개월차에 변경했다면, 7개월차 ceiling = `500,000 × 0.95^7 = 348,436`
- maxSupply는 변경 불가 (기존 계획보다 느리게 발행될 뿐)

---

## Scenario 8: Multisig — Treasury에서 USDT 출금

**주체:** Owner (Biyard가 Multisig 통해 실행)

### 플로우

```
1. Biyard → Multisig.propose(
     target: Treasury,
     data: withdrawStable(운영지갑, 1000 USDT).encode(),
     value: 0
   )

2. Biyard → Multisig.approve(proposalId)
3. Biyard → Multisig.execute(proposalId)
   → Treasury에서 운영지갑으로 1,000 USDT 이체
```

### 결과

- Treasury USDT 감소 → 토큰 가격 하락
- 현재 1-of-1 구조에서 Multisig은 사실상 관리자 키 역할
- 향후 멀티시그(2-of-3 등)로 전환하면 출금에 다수 서명이 필요해져 견제 가능

---

## Scenario 9: 테스트 환경 — BUSDT로 전체 플로우 테스트

**주체:** 개발자 / QA

### 플로우

```
1. BUSDT.mint(테스터지갑, 1,000,000 BUSDT)  — 무제한 faucet
2. BUSDT.approve(Treasury, 100,000)
3. Treasury.deposit(100,000)  — BUSDT로 Treasury 채움
4. BrandToken.triggerMonthlyMint()  — 첫 달 토큰 발행
5. Biyard 서버에서 claim 서명 생성
6. BrandToken.claim(10000, nonce, deadline, sig)  — 토큰 수령
7. Treasury.getPrice()  — 가격 확인
8. BrandToken.approve(Treasury, 5000)
9. Treasury.buyback(5000)  — 토큰 → BUSDT 교환
10. Multisig.propose → approve → execute  — 운영 테스트
```

### 검증 포인트

- [ ] BUSDT 무제한 민팅 가능
- [ ] deposit 후 Treasury 잔고 증가
- [ ] triggerMonthlyMint 후 distribution slot 지갑에 토큰 입금
- [ ] claim pool에서 유저 토큰 수령
- [ ] getPrice() 가격 공식 정확
- [ ] buyback 후 USDT 수령, 토큰은 Treasury에 보관
- [ ] buyback 전후 가격 동일
- [ ] Multisig 제안→승인→실행 전체 플로우
- [ ] 같은 달에 triggerMonthlyMint 중복 호출 시 revert
- [ ] 만료된 deadline으로 claim 시 revert
- [ ] 동일 nonce로 claim 시 revert
- [ ] maxSupply 초과 민팅 시 revert

---

## Edge Cases

### 가격이 0인 경우

- Treasury USDT가 0이면 가격 = 0
- Buyback 시 usdtOut = 0 → 유저에게 의미 없음 → revert 처리

### Claim pool 소진

- 당월 claim pool이 전부 소진되면 추가 claim 불가
- 다음 달 triggerMonthlyMint 이후 다시 claim 가능

### maxSupply 도달

- triggerMonthlyMint에서 남은 여유분만 민팅 (ceiling보다 적을 수 있음)
- maxSupply 이후 추가 민팅 영구 불가

### 아무도 triggerMonthlyMint를 호출하지 않은 달

- 해당 월의 토큰은 영구 미발행 (maxSupply에서 차감되지 않음)
- 다음 달에 triggerMonthlyMint 하면 다음 달 ceiling으로 발행
- 이전 달 미발행분은 소실 (의도적 디자인: 발행 인센티브)

### Treasury에 쌓인 BrandToken 처리

- Buyback으로 회수된 토큰은 Treasury에 보관 → 유통량에서 제외 → 가격 유지 효과
- Multisig이 결정 가능: 소각 / 재분배 / 유동성 제공 등
- `withdrawToken(BrandToken, ...)` 으로 꺼낼 수 있음
- **주의:** 회수 토큰을 재유통하면 유통량이 다시 늘어나 가격이 하락합니다. Buyback의 가격 유지 효과는 "회수 토큰을 재유통하지 않는 한"에만 성립합니다.
