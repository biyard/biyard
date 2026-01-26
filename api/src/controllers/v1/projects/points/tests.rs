#[cfg(test)]
mod tests {
    use crate::features::points::*;
    use crate::features::projects::*;
    use crate::tests::test_context::TestContext;
    use crate::tests::*;
    use crate::*;

    #[tokio::test]
    async fn test_transact_points_award_success() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project first
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Points Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        // Award points to a meta user using new batch API format
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": "test-user-123",
            "amount": 100,
            "description": "Test award",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };

        assert_eq!(status, 200, "Should successfully award points");
    }

    #[tokio::test]
    async fn test_transact_points_multiple_awards() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Multi Award Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "test-user-multi";

        // Award points multiple times using batch API
        for i in 1..=3 {
            let body = serde_json::json!([{
                "tx_type": "Award",
                "to": meta_user_id,
                "amount": i * 50,
                "description": format!("Award #{}", i),
            }]);

            let (status, _, _) = call! {
                app: &ctx.app,
                path: format!("/v1/projects/{}/points", project_id),
                method: "POST",
                body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
                headers: headers.clone(),
                response_type: serde_json::Value
            };
            assert_eq!(status, 200, "Award #{} should succeed", i);
        }
    }

    #[tokio::test]
    async fn test_transact_points_with_specific_month() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Month Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "test-user-month";

        // Award points with specific month using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": meta_user_id,
            "amount": 200,
            "month": "2025-01",
            "description": "January award",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };

        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn test_transact_points_invalid_amount() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Invalid Amount Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "test-user-invalid";

        // Try to award 0 or negative points (should fail validation) using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": meta_user_id,
            "amount": 0,
            "description": "Invalid award",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };

        assert_eq!(status, 400, "Should return 400 for invalid amount");
    }

    #[tokio::test]
    async fn test_get_balance_success() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Balance Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "balance-test-user";

        // Award some points first using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": meta_user_id,
            "amount": 500,
            "description": "Initial balance",
        }]);

        let (status, _, _body) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };
        assert_eq!(status, 200);

        // Get the balance
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/{}", project_id, meta_user_id),
            headers: headers.clone(),
            response_type: ListResponse<PointBalanceResponse>,
        };

        assert_eq!(status, 200);
        assert!(
            !response.items.is_empty(),
            "Should have at least one balance record"
        );

        let balance = &response.items[0];
        assert_eq!(balance.balance, 500);
        assert_eq!(balance.total_earned, 500);
    }

    #[tokio::test]
    async fn test_get_balance_empty() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Empty Balance Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "no-balance-user";

        // Get balance without awarding any points
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/{}", project_id, meta_user_id),
            headers: headers.clone(),
            response_type: ListResponse<PointBalanceResponse>,
        };

        assert_eq!(status, 200);
        assert_eq!(response.items.len(), 0, "Should have no balance records");
    }

    #[tokio::test]
    async fn test_get_balance_with_month_filter() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Month Filter Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "month-filter-user";

        // Award points for different months using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": meta_user_id,
            "amount": 100,
            "month": "2025-01",
            "description": "January",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };
        assert_eq!(status, 200);

        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": meta_user_id,
            "amount": 200,
            "month": "2025-02",
            "description": "February",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };
        assert_eq!(status, 200);

        // Get balance for specific month
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/{}?month=2025-01", project_id, meta_user_id),
            headers: headers.clone(),
            response_type: ListResponse<PointBalanceResponse>,
        };

        assert_eq!(status, 200);
        assert!(!response.items.is_empty());

        // Verify it's January balance
        let balance = &response.items[0];
        assert_eq!(balance.month, "2025-01");
    }

    #[tokio::test]
    async fn test_list_transactions_success() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Transactions Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "tx-test-user";

        // Create some transactions using batch API
        for i in 1..=3 {
            let body = serde_json::json!([{
                "tx_type": "Award",
                "to": meta_user_id,
                "amount": i * 100,
                "description": format!("Transaction {}", i),
            }]);

            let (status, _, _) = call! {
                app: &ctx.app,
                path: format!("/v1/projects/{}/points", project_id),
                method: "POST",
                body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
                headers: headers.clone(),
                response_type: serde_json::Value
            };
            assert_eq!(status, 200);
        }

        // List all transactions for the project
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/transactions", project_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };

        assert_eq!(status, 200);
        assert!(
            response.items.len() >= 3,
            "Should have at least 3 transactions"
        );
    }

    #[tokio::test]
    async fn test_list_transactions_empty() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Empty Transactions Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        // List transactions without creating any
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/transactions", project_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };

        assert_eq!(status, 200);
        assert_eq!(response.items.len(), 0, "Should have no transactions");
    }

    #[tokio::test]
    async fn test_list_transactions_with_pagination() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Pagination Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "pagination-user";

        // Create multiple transactions using batch API
        for i in 1..=5 {
            let body = serde_json::json!([{
                "tx_type": "Award",
                "to": meta_user_id,
                "amount": i * 10,
                "description": format!("TX {}", i),
            }]);

            let (status, _, _) = call! {
                app: &ctx.app,
                path: format!("/v1/projects/{}/points", project_id),
                method: "POST",
                body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
                headers: headers.clone(),
                response_type: serde_json::Value
            };
            assert_eq!(status, 200);
        }

        // List with pagination (limit 2)
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/transactions?limit=2", project_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };

        assert_eq!(status, 200);
        assert_eq!(response.items.len(), 2, "Should return only 2 transactions");
        assert!(
            response.bookmark.is_some(),
            "Should have bookmark for next page"
        );
    }

    #[tokio::test]
    async fn test_unauthorized_access_to_points() {
        let ctx = TestContext::setup().await;
        let (_, headers1) = &ctx.account1;
        let (_, headers2) = &ctx.account2;

        // Account 1 creates a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers1.clone(),
            body: {
                "name": "Account 1 Points Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "test-user";

        // Account 2 tries to award points - should be FORBIDDEN (403) using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": meta_user_id,
            "amount": 100,
            "description": "Unauthorized award",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers2.clone(),
            response_type: serde_json::Value
        };

        assert_eq!(status, 403, "Should return 403 Forbidden");
    }

    #[tokio::test]
    async fn test_unauthenticated_access_to_points() {
        let ctx = TestContext::setup().await;
        let (_, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Auth Test Points Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "test-user";

        // Try to award points without authentication - should be UNAUTHORIZED (401) using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": meta_user_id,
            "amount": 100,
            "description": "Unauthenticated award",
        }]);

        let empty_headers = by_axum::axum::http::HeaderMap::new();
        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: empty_headers,
            response_type: serde_json::Value
        };

        assert_eq!(status, 401, "Should return 401 Unauthorized");
    }

    #[tokio::test]
    async fn test_points_isolation_between_users() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Isolation Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        // Award points to user1 using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": "user1",
            "amount": 100,
            "description": "User1 points",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };
        assert_eq!(status, 200);

        // Award points to user2 using batch API
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": "user2",
            "amount": 200,
            "description": "User2 points",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };
        assert_eq!(status, 200);

        // Check user1's balance
        let (status, _, response1) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/user1", project_id),
            headers: headers.clone(),
            response_type: ListResponse<PointBalanceResponse>,
        };
        assert_eq!(status, 200);
        assert_eq!(response1.items[0].balance, 100);

        // Check user2's balance
        let (status, _, response2) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/user2", project_id),
            headers: headers.clone(),
            response_type: ListResponse<PointBalanceResponse>,
        };
        assert_eq!(status, 200);
        assert_eq!(response2.items[0].balance, 200);
    }

    #[tokio::test]
    async fn test_list_user_transactions_success() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "User Transactions Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "user-tx-test";

        // Create some transactions for the user using batch API
        for i in 1..=3 {
            let body = serde_json::json!([{
                "tx_type": "Award",
                "to": meta_user_id,
                "amount": i * 100,
                "description": format!("User Transaction {}", i),
            }]);

            let (status, _, _) = call! {
                app: &ctx.app,
                path: format!("/v1/projects/{}/points", project_id),
                method: "POST",
                body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
                headers: headers.clone(),
                response_type: serde_json::Value
            };
            assert_eq!(status, 200);
        }

        // List transactions for the specific user
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/{}/transactions", project_id, meta_user_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };

        assert_eq!(status, 200);
        assert_eq!(
            response.items.len(),
            3,
            "Should have exactly 3 transactions for this user"
        );

        // Verify all transactions belong to the correct user
        for tx in response.items {
            assert_eq!(tx.meta_user_id, meta_user_id);
        }
    }

    #[tokio::test]
    async fn test_list_user_transactions_empty() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Empty User Transactions Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "user-no-tx";

        // List transactions without creating any
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/{}/transactions", project_id, meta_user_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };

        assert_eq!(status, 200);
        assert_eq!(response.items.len(), 0, "Should have no transactions");
    }

    #[tokio::test]
    async fn test_list_user_transactions_isolation() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "User Isolation Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        // Award points to user1
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": "user1",
            "amount": 100,
            "description": "User1 transaction",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };
        assert_eq!(status, 200);

        // Award points to user2
        let body = serde_json::json!([{
            "tx_type": "Award",
            "to": "user2",
            "amount": 200,
            "description": "User2 transaction",
        }]);

        let (status, _, _) = call! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points", project_id),
            method: "POST",
            body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
            headers: headers.clone(),
            response_type: serde_json::Value
        };
        assert_eq!(status, 200);

        // Get user1's transactions
        let (status, _, response1) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/user1/transactions", project_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };
        assert_eq!(status, 200);
        assert_eq!(response1.items.len(), 1);
        assert_eq!(response1.items[0].meta_user_id, "user1");
        assert_eq!(response1.items[0].amount, 100);

        // Get user2's transactions
        let (status, _, response2) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/user2/transactions", project_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };
        assert_eq!(status, 200);
        assert_eq!(response2.items.len(), 1);
        assert_eq!(response2.items[0].meta_user_id, "user2");
        assert_eq!(response2.items[0].amount, 200);
    }

    #[tokio::test]
    async fn test_list_user_transactions_with_pagination() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "User Pagination Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "user-pagination";

        // Create multiple transactions for the user
        for i in 1..=5 {
            let body = serde_json::json!([{
                "tx_type": "Award",
                "to": meta_user_id,
                "amount": i * 10,
                "description": format!("TX {}", i),
            }]);

            let (status, _, _) = call! {
                app: &ctx.app,
                path: format!("/v1/projects/{}/points", project_id),
                method: "POST",
                body: axum::body::Body::from(serde_json::to_vec(&body).unwrap()),
                headers: headers.clone(),
                response_type: serde_json::Value
            };
            assert_eq!(status, 200);
        }

        // List with pagination (limit 2)
        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/points/{}/transactions?limit=2", project_id, meta_user_id),
            headers: headers.clone(),
            response_type: ListResponse<PointTransactionResponse>,
        };

        assert_eq!(status, 200);
        assert_eq!(response.items.len(), 2, "Should return only 2 transactions");
        assert!(
            response.bookmark.is_some(),
            "Should have bookmark for next page"
        );

        // Verify all transactions belong to the correct user
        for tx in response.items {
            assert_eq!(tx.meta_user_id, meta_user_id);
        }
    }
}
