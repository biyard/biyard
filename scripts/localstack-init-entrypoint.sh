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
        "PK": {"S": "USER#admin@biyard.io"},
        "SK": {"S": "PROFILE"},
        "EntityType": {"S": "User"},
        "UserId": {"S": "admin-user-id"},
        "Email": {"S": "admin@biyard.io"},
        "DisplayName": {"S": "System Admin"},
        "Role": {"S": "admin"},
        "CreatedAt": {"S": "2025-01-01T00:00:00Z"},
        "UpdatedAt": {"S": "2025-01-01T00:00:00Z"}
    }'

echo 'Adding test user...'
aws --endpoint-url=http://localstack:4566 dynamodb put-item \
    --table-name biyard-local-main \
    --item '{
        "PK": {"S": "USER#test@biyard.io"},
        "SK": {"S": "PROFILE"},
        "EntityType": {"S": "User"},
        "UserId": {"S": "test-user-id"},
        "Email": {"S": "test@biyard.io"},
        "DisplayName": {"S": "Test User"},
        "Role": {"S": "user"},
        "CreatedAt": {"S": "2025-01-01T00:00:00Z"},
        "UpdatedAt": {"S": "2025-01-01T00:00:00Z"}
    }'

echo 'Test users added successfully'

