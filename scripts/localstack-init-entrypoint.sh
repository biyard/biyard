#!/bin/bash

echo "start"
yum update
yum install -y awscli

echo 'Waiting for LocalStack to be ready...'
until aws dynamodb --endpoint-url=http://localstack:4566  list-tables >/dev/null 2>&1; do
    sleep 2
done
echo 'Creating ratel-local table with GSIs...'
aws --endpoint-url=http://localstack:4566 dynamodb create-table --cli-input-json file:///scripts/dynamodb-schema.json
echo 'biyard-local-main table and GSIs created successfully'

echo 'Adding system admin user...'
aws --endpoint-url=http://localstack:4566 dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "password": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05",
        "user_type": 99,
        "updated_at": 1761621606252,
        "gsi1_pk": "AC#admin@biyard.co",
        "sk": "ACCOUNT",
        "name": "SystemAdmin",
        "created_at": 1761621606252,
        "pk": "ACCOUNT#75734ca2-d695-4c95-88ea-4328825cd936",
        "gsi2_pk": "AC#admin@biyard.co",
        "gsi2_sk": "ACCOUNT",
        "gsi1_sk": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05",
        "email": "admin@biyard.co"
    }'

echo 'Adding test user...'
aws --endpoint-url=http://localstack:4566 dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "password": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05",
        "user_type": 1,
        "updated_at": 1761621878390,
        "sk": "ACCOUNT",
        "name": "Playwright Test",
        "gsi1_pk": "AC#test@biyard.co",
        "created_at": 1761621878390,
        "pk": "ACCOUNT#e1cfb27d-b0e6-43de-ab76-784974352466",
        "gsi2_pk": "AC#test@biyard.co",
        "gsi2_sk": "ACCOUNT",
        "gsi1_sk": "e542fdd785ab67a110adf8c0e3b3f3ff9bcdbdec3091c0114d00010501b67c05",
        "email": "test@biyard.co"
    }'

echo 'Test users added successfully'

echo 'Admin User'
echo '  Email: admin@biyard.co'
echo '  Password: qwer1234!@#$'

echo 'Test User'
echo '  Email: test@biyard.co'
echo '  Password: qwer1234!@#$'
