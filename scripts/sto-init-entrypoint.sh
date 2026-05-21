#!/bin/bash
# STO localstack init — biyard-local-sto 테이블 생성 + 시드 데이터 적재
# console 의 localstack-init-entrypoint.sh 와 동일한 패턴.

set -e

echo "[sto-init] start"
yum update -y >/dev/null 2>&1 || true
yum install -y awscli jq >/dev/null 2>&1

DYNAMO_ENDPOINT=${DYNAMO_ENDPOINT:-http://localhost:4566}
TABLE="biyard-local-sto"

echo "[sto-init] Waiting for LocalStack..."
until aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb list-tables >/dev/null 2>&1; do
    sleep 2
done

# 1. 테이블 생성 (이미 있으면 무시)
echo "[sto-init] Creating $TABLE table..."
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb create-table \
    --cli-input-json file:///scripts/sto-table.json >/dev/null 2>&1 \
    && echo "[sto-init] Table created" \
    || echo "[sto-init] Table already exists or create skipped"

# 2. 시드 데이터 BatchWriteItem (25건씩 chunk)
SEED_FILE=/scripts/sto-seed-data.json
if [ ! -f "$SEED_FILE" ]; then
    echo "[sto-init] ERROR: seed file not found at $SEED_FILE"
    exit 1
fi

TOTAL=$(jq ".[\"$TABLE\"] | length" "$SEED_FILE")
echo "[sto-init] Total items to seed: $TOTAL"

OK=0
FAIL=0
for i in $(seq 0 25 $((TOTAL - 1))); do
    END=$((i + 25))
    CHUNK=$(jq --argjson start $i --argjson end $END \
        "{\"$TABLE\": .[\"$TABLE\"][\$start:\$end]}" "$SEED_FILE")
    echo "$CHUNK" > /tmp/sto-chunk.json
    if aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb batch-write-item \
        --request-items file:///tmp/sto-chunk.json >/dev/null 2>&1; then
        OK=$((OK + 25))
    else
        FAIL=$((FAIL + 25))
        echo "[sto-init] chunk $i-$END failed"
    fi
done

# 마지막 chunk 처리는 25 단위 잘림으로 끝
ACTUAL=$(aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb scan \
    --table-name $TABLE --select COUNT 2>/dev/null \
    | jq '.Count' 2>/dev/null || echo "?")
echo "[sto-init] Seed complete. Items in table: $ACTUAL / $TOTAL"
