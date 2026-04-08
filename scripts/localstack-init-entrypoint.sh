#!/bin/bash

echo "start"
yum update
yum install -y awscli

# set default dynamo_endpoint if absent
DYNAMO_ENDPOINT=${DYNAMO_ENDPOINT:-http://localhost:4566}

echo 'Waiting for LocalStack to be ready...'
until aws dynamodb --endpoint-url=$DYNAMO_ENDPOINT  list-tables >/dev/null 2>&1; do
    sleep 2
done
echo 'Creating biyard-local-main table with GSIs...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb create-table --cli-input-json file://./scripts/dynamodb-table.json >/dev/null 2>&1
echo 'biyard-local-main table and GSIs created successfully'

# ----------------------------------------------------------------------------
# Seed data layout (current single-table model)
# ----------------------------------------------------------------------------
# Account     pk=ACCOUNT#<uuid>      sk=ACCOUNT
#   gsi1: pk=AC#<email>              sk=<password_hash>      (find_by_email_and_password)
#   gsi2: pk=AC#<email>              sk=ACCOUNT              (find_by_email)
#   gsi3: pk=ENTERPRISE#<id>         sk=ACCOUNT              (find_by_enterprise_id)
#
# Enterprise  pk=ENTERPRISE#<id>     sk=ENTERPRISE
#   gsi1: pk=OWNER#ACCOUNT#<owner_uuid>   sk=<created_at>    (find_by_owner_account_id)
#
# Project     pk=PROJECT#<id>        sk=PROJECT
#   gsi1: pk=ACCOUNT#<uuid>          sk=<created_at>         (find_by_account_id)
#   gsi2: pk=ENTERPRISE#<id>         sk=<created_at>         (find_by_organization_id)
#
# ProjectToken  pk=PROJECT#<id>      sk=TOKEN  (1:1 with project, no GSIs)
#
# Credential  pk=CREDENTIAL#<uuid>   sk=CREDENTIAL
#   gsi1: pk=CRED#ACCOUNT#<uuid>     sk=<created_at>         (find_by_account_id)
#   gsi2: pk=CRED#<api_key_hash>     sk=<created_at>         (find_by_api_key_hash)
#   gsi3: pk=CRED#ENTERPRISE#<id>    sk=<created_at>         (find_by_organization_id)
#
# Each seeded account is paired with its own personal Enterprise, created at
# signup time. Account.enterprise_id points at the Enterprise's pk.
# ----------------------------------------------------------------------------

ADMIN_ACCOUNT_ID="75734ca2-d695-4c95-88ea-4328825cd936"
ADMIN_ENTERPRISE_ID="01999999-0000-7000-8000-000000000001"
TEST_ACCOUNT_ID="e1cfb27d-b0e6-43de-ab76-784974352466"
TEST_ENTERPRISE_ID="01999999-0000-7000-8000-000000000002"

# qwer1234!@# legacy SHA3-256
PASSWORD_HASH="e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05"

NOW_MS="1761621606252"

echo 'Adding system admin account...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "ACCOUNT#'"${ADMIN_ACCOUNT_ID}"'"},
        "sk": {"S": "ACCOUNT"},
        "gsi1_pk": {"S": "AC#admin@biyard.co"},
        "gsi1_sk": {"S": "'"${PASSWORD_HASH}"'"},
        "gsi2_pk": {"S": "AC#admin@biyard.co"},
        "gsi2_sk": {"S": "ACCOUNT"},
        "gsi3_pk": {"S": "ENTERPRISE#'"${ADMIN_ENTERPRISE_ID}"'"},
        "gsi3_sk": {"S": "ACCOUNT"},
        "name": {"S": "SystemAdmin"},
        "email": {"S": "admin@biyard.co"},
        "password": {"S": "'"${PASSWORD_HASH}"'"},
        "password_scheme": {"S": "LEGACY_SHA_3"},
        "enterprise_id": {"S": "ENTERPRISE#'"${ADMIN_ENTERPRISE_ID}"'"},
        "organization_role": {"N": "3"},
        "user_type": {"N": "99"},
        "created_at": {"N": "'"${NOW_MS}"'"},
        "updated_at": {"N": "'"${NOW_MS}"'"}
    }'

echo 'Adding test account...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "ACCOUNT#'"${TEST_ACCOUNT_ID}"'"},
        "sk": {"S": "ACCOUNT"},
        "gsi1_pk": {"S": "AC#test@biyard.co"},
        "gsi1_sk": {"S": "'"${PASSWORD_HASH}"'"},
        "gsi2_pk": {"S": "AC#test@biyard.co"},
        "gsi2_sk": {"S": "ACCOUNT"},
        "gsi3_pk": {"S": "ENTERPRISE#'"${TEST_ENTERPRISE_ID}"'"},
        "gsi3_sk": {"S": "ACCOUNT"},
        "name": {"S": "Playwright Test"},
        "email": {"S": "test@biyard.co"},
        "password": {"S": "'"${PASSWORD_HASH}"'"},
        "password_scheme": {"S": "LEGACY_SHA_3"},
        "enterprise_id": {"S": "ENTERPRISE#'"${TEST_ENTERPRISE_ID}"'"},
        "organization_role": {"N": "3"},
        "user_type": {"N": "1"},
        "created_at": {"N": "'"${NOW_MS}"'"},
        "updated_at": {"N": "'"${NOW_MS}"'"}
    }'

echo 'Accounts seeded successfully'

echo 'Adding admin personal enterprise...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "ENTERPRISE#'"${ADMIN_ENTERPRISE_ID}"'"},
        "sk": {"S": "ENTERPRISE"},
        "gsi1_pk": {"S": "OWNER#ACCOUNT#'"${ADMIN_ACCOUNT_ID}"'"},
        "gsi1_sk": {"S": "'"${NOW_MS}"'"},
        "owner_account_id": {"S": "ACCOUNT#'"${ADMIN_ACCOUNT_ID}"'"},
        "name": {"S": "SystemAdmin Personal"},
        "created_at": {"N": "'"${NOW_MS}"'"},
        "updated_at": {"N": "'"${NOW_MS}"'"}
    }'

echo 'Adding test personal enterprise...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "ENTERPRISE#'"${TEST_ENTERPRISE_ID}"'"},
        "sk": {"S": "ENTERPRISE"},
        "gsi1_pk": {"S": "OWNER#ACCOUNT#'"${TEST_ACCOUNT_ID}"'"},
        "gsi1_sk": {"S": "'"${NOW_MS}"'"},
        "owner_account_id": {"S": "ACCOUNT#'"${TEST_ACCOUNT_ID}"'"},
        "name": {"S": "Playwright Test Personal"},
        "created_at": {"N": "'"${NOW_MS}"'"},
        "updated_at": {"N": "'"${NOW_MS}"'"}
    }'

echo 'Enterprises seeded successfully'

PROJECT_CREATED_AT="1766688564371"

echo 'Adding ratel project for admin enterprise...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "PROJECT#ratel"},
        "sk": {"S": "PROJECT"},
        "gsi1_pk": {"S": "ACCOUNT#'"${ADMIN_ACCOUNT_ID}"'"},
        "gsi1_sk": {"S": "'"${PROJECT_CREATED_AT}"'"},
        "gsi2_pk": {"S": "ENTERPRISE#'"${ADMIN_ENTERPRISE_ID}"'"},
        "gsi2_sk": {"S": "'"${PROJECT_CREATED_AT}"'"},
        "account_id": {"S": "ACCOUNT#'"${ADMIN_ACCOUNT_ID}"'"},
        "organization_id": {"S": "ENTERPRISE#'"${ADMIN_ENTERPRISE_ID}"'"},
        "name": {"S": "RATEL"},
        "description": {"NULL": true},
        "brand_logo_url": {"NULL": true},
        "monthly_token_supply": {"N": "10000"},
        "treasury_reserve_rate": {"N": "0.1"},
        "status": {"S": "ACTIVE"},
        "created_at": {"N": "'"${PROJECT_CREATED_AT}"'"},
        "updated_at": {"N": "'"${PROJECT_CREATED_AT}"'"}
    }'

echo 'Adding RATEL token for the project...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "PROJECT#ratel"},
        "sk": {"S": "TOKEN"},
        "name": {"S": "RATEL"},
        "symbol": {"S": "RAT"},
        "decimals": {"N": "0"},
        "description": {"S": "RATEL DEFAULT"},
        "total_supply": {"N": "0"},
        "circulating_supply": {"N": "0"},
        "contract_address": {"NULL": true},
        "treasury_contract_address": {"NULL": true},
        "stable_token_address": {"NULL": true},
        "chain_id": {"NULL": true},
        "deployment_tx_hash": {"NULL": true},
        "treasury_deployment_tx_hash": {"NULL": true},
        "treasury_reserve_bps": {"N": "0"},
        "created_at": {"N": "'"${PROJECT_CREATED_AT}"'"},
        "updated_at": {"N": "'"${PROJECT_CREATED_AT}"'"}
    }'

echo 'Project and token seeded successfully'

# API Key: by_local_test_api_key_12345678
# Hash: SHA3-256 of the API key
API_KEY_HASH="66a99dd5dca8c5ae22b7dd90771e9a516b7774d9d58100dd8ddb51cebe5ccd83"

echo 'Adding API credential for admin account...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "CREDENTIAL#admin-api-credential"},
        "sk": {"S": "CREDENTIAL"},
        "gsi1_pk": {"S": "CRED#ACCOUNT#'"${ADMIN_ACCOUNT_ID}"'"},
        "gsi1_sk": {"S": "'"${NOW_MS}"'"},
        "gsi2_pk": {"S": "CRED#'"${API_KEY_HASH}"'"},
        "gsi2_sk": {"S": "'"${NOW_MS}"'"},
        "gsi3_pk": {"S": "CRED#ENTERPRISE#'"${ADMIN_ENTERPRISE_ID}"'"},
        "gsi3_sk": {"S": "'"${NOW_MS}"'"},
        "account_id": {"S": "ACCOUNT#'"${ADMIN_ACCOUNT_ID}"'"},
        "organization_id": {"S": "ENTERPRISE#'"${ADMIN_ENTERPRISE_ID}"'"},
        "name": {"S": "Local Development API Key"},
        "api_key_hash": {"S": "'"${API_KEY_HASH}"'"},
        "api_key_prefix": {"S": "by_local_test"},
        "status": {"N": "1"},
        "created_at": {"N": "'"${NOW_MS}"'"},
        "updated_at": {"N": "'"${NOW_MS}"'"},
        "last_used_at": {"NULL": true}
    }'

echo 'Credential seeded successfully'

echo ''
echo '=========================================='
echo 'Admin User'
echo '  Email: admin@biyard.co'
echo '  Password: qwer1234!@#$'
echo "  Enterprise: ENTERPRISE#${ADMIN_ENTERPRISE_ID}"
echo ''
echo 'Test User'
echo '  Email: test@biyard.co'
echo '  Password: qwer1234!@#$'
echo "  Enterprise: ENTERPRISE#${TEST_ENTERPRISE_ID}"
echo ''
echo 'Project (Admin Enterprise)'
echo '  Project ID: ratel'
echo '  Project Name: RATEL'
echo '  Token: RAT'
echo ''
echo 'API Credential (Admin)'
echo '  API Key: by_local_test_api_key_12345678'
echo '  Note: Use as Bearer token in Authorization header'
echo '=========================================='
