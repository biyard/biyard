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
echo 'Creating ratel-local table with GSIs...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb create-table --cli-input-json file://./scripts/dynamodb-table.json >/dev/null 2>&1
echo 'biyard-local-main table and GSIs created successfully'

echo 'Adding system admin user...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "ACCOUNT#75734ca2-d695-4c95-88ea-4328825cd936"},
        "sk": {"S": "ACCOUNT"},
        "gsi1_pk": {"S": "AC#admin@biyard.co"},
        "gsi1_sk": {"S": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05"},
        "gsi2_pk": {"S": "AC#admin@biyard.co"},
        "gsi2_sk": {"S": "ACCOUNT"},
        "email": {"S": "admin@biyard.co"},
        "password": {"S": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05"},
        "name": {"S": "SystemAdmin"},
        "user_type": {"N": "99"},
        "created_at": {"N": "1761621606252"},
        "updated_at": {"N": "1761621606252"}
    }'

echo 'Adding test user...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "ACCOUNT#e1cfb27d-b0e6-43de-ab76-784974352466"},
        "sk": {"S": "ACCOUNT"},
        "gsi1_pk": {"S": "AC#test@biyard.co"},
        "gsi1_sk": {"S": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05"},
        "gsi2_pk": {"S": "AC#test@biyard.co"},
        "gsi2_sk": {"S": "ACCOUNT"},
        "email": {"S": "test@biyard.co"},
        "password": {"S": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05"},
        "name": {"S": "Playwright Test"},
        "user_type": {"N": "1"},
        "created_at": {"N": "1761621878390"},
        "updated_at": {"N": "1761621878390"}
    }'

echo 'Test users added successfully'

echo 'Adding ratel project for admin user...'
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "PROJECT#ratel"},
        "sk": {"S": "PROJECT"},
        "gsi1_pk": {"S": "ACCOUNT#75734ca2-d695-4c95-88ea-4328825cd936"},
        "gsi1_sk": {"S": "PROJECT"},
        "account_id": {"S": "ACCOUNT#75734ca2-d695-4c95-88ea-4328825cd936"},
        "name": {"S": "Ratel"},
        "description": {"S": "Ratel Project for Local Development"},
        "monthly_token_supply": {"N": "0"},
        "status": {"S": "ACTIVE"},
        "created_at": {"N": "1761621606252"},
        "updated_at": {"N": "1761621606252"}
    }'

echo 'Adding API credential for admin user...'
# API Key: by_local_test_api_key_12345678
# Hash: SHA3-256 of the API key
aws --endpoint-url=$DYNAMO_ENDPOINT dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "pk": {"S": "CREDENTIAL#admin-api-credential"},
        "sk": {"S": "CREDENTIAL"},
        "gsi1_pk": {"S": "CRED#ACCOUNT#75734ca2-d695-4c95-88ea-4328825cd936"},
        "gsi1_sk": {"S": "CREDENTIAL"},
        "gsi2_pk": {"S": "CRED#66a99dd5dca8c5ae22b7dd90771e9a516b7774d9d58100dd8ddb51cebe5ccd83"},
        "gsi2_sk": {"S": "CREDENTIAL"},
        "account_id": {"S": "ACCOUNT#75734ca2-d695-4c95-88ea-4328825cd936"},
        "name": {"S": "Local Development API Key"},
        "api_key_hash": {"S": "66a99dd5dca8c5ae22b7dd90771e9a516b7774d9d58100dd8ddb51cebe5ccd83"},
        "api_key_prefix": {"S": "by_local_test"},
        "status": {"N": "1"},
        "created_at": {"N": "1761621606252"},
        "updated_at": {"N": "1761621606252"}
    }'

echo 'Project and Credential added successfully'

echo ''
echo '=========================================='
echo 'Admin User'
echo '  Email: admin@biyard.co'
echo '  Password: qwer1234!@#$'
echo ''
echo 'Test User'
echo '  Email: test@biyard.co'
echo '  Password: qwer1234!@#$'
echo ''
echo 'Project (Admin)'
echo '  ID: ratel'
echo '  Name: Ratel'
echo ''
echo 'API Credential (Admin)'
echo '  API Key: by_local_test_api_key_12345678'
echo '  Note: Use as Bearer token in Authorization header'
echo '=========================================='
