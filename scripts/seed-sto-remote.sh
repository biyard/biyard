#!/bin/bash
# STO DynamoDB 시드 — scripts/sto-seed-data.json 을 biyard-{stage}-sto 테이블에 적재.
# 사용법:
#   STAGE=local                                 bash scripts/seed-sto-remote.sh   # LocalStack
#   STAGE=dev  AWS_REGION=ap-northeast-2        bash scripts/seed-sto-remote.sh   # AWS dev
#   STAGE=prod AWS_REGION=ap-northeast-2        bash scripts/seed-sto-remote.sh   # AWS prod
# Makefile: `make seed STAGE=local` / `make seed STAGE=dev`

set -e

STAGE=${STAGE:-dev}
AWS_REGION=${AWS_REGION:-ap-northeast-2}
SOURCE_TABLE="biyard-local-sto"
TARGET_TABLE="biyard-${STAGE}-sto"
SEED_FILE="$(cd "$(dirname "$0")/.." && pwd)/scripts/sto-seed-data.json"

# LocalStack 분기 — STAGE=local 일 때 endpoint + 더미 자격증명 자동 설정
AWS_FLAGS=(--region "$AWS_REGION")
if [ "$STAGE" = "local" ]; then
    LOCAL_ENDPOINT=${DYNAMO_ENDPOINT:-http://localhost:4566}
    AWS_FLAGS+=(--endpoint-url "$LOCAL_ENDPOINT")
    export AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID:-test}
    export AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY:-test}
    echo "[seed-sto] LocalStack endpoint=$LOCAL_ENDPOINT"
fi

if [ ! -f "$SEED_FILE" ]; then
    echo "[seed-sto] ERROR: seed file not found at $SEED_FILE"
    echo "[seed-sto] hint: regenerate via 'python3 tools/build-sto-seed.py'"
    exit 1
fi

echo "[seed-sto] stage=$STAGE region=$AWS_REGION target=$TARGET_TABLE"

# 테이블 존재 확인
if ! aws "${AWS_FLAGS[@]}" dynamodb describe-table --table-name "$TARGET_TABLE" >/dev/null 2>&1; then
    echo "[seed-sto] ERROR: table $TARGET_TABLE does not exist in $AWS_REGION."
    if [ "$STAGE" = "local" ]; then
        echo "[seed-sto] hint: start LocalStack via 'docker compose up -d' and ensure localstack-init created the table."
    else
        echo "[seed-sto] hint: deploy CDK first ('make deploy ENV=$STAGE')"
    fi
    exit 1
fi

TOTAL=$(jq ".[\"$SOURCE_TABLE\"] | length" "$SEED_FILE")
echo "[seed-sto] Total items to seed: $TOTAL"

OK=0
FAIL=0
TMP_CHUNK=$(mktemp -t sto-chunk.XXXXXX.json)
trap 'rm -f "$TMP_CHUNK"' EXIT

for i in $(seq 0 25 $((TOTAL - 1))); do
    END=$((i + 25))
    # source -> target 키 변환 (local -> stage)
    jq --arg src "$SOURCE_TABLE" --arg dst "$TARGET_TABLE" \
       --argjson start $i --argjson end $END \
       '{($dst): .[$src][$start:$end]}' \
       "$SEED_FILE" > "$TMP_CHUNK"

    if aws "${AWS_FLAGS[@]}" dynamodb batch-write-item \
        --request-items "file://$TMP_CHUNK" >/dev/null 2>&1; then
        OK=$((OK + 25))
    else
        FAIL=$((FAIL + 25))
        echo "[seed-sto] chunk $i-$END failed"
    fi
done

ACTUAL=$(aws "${AWS_FLAGS[@]}" dynamodb scan \
    --table-name "$TARGET_TABLE" --select COUNT 2>/dev/null \
    | jq '.Count' 2>/dev/null || echo "?")
echo "[seed-sto] Seed complete. Items in $TARGET_TABLE: $ACTUAL / $TOTAL"
