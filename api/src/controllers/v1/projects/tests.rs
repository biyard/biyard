#[cfg(test)]
mod tests {
    use crate::features::projects::*;
    use crate::tests::test_context::TestContext;
    use crate::tests::*;
    use crate::*;

    #[tokio::test]
    async fn test_create_project() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _header, response) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Test Project",
                "description": "A test project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };

        assert_eq!(status, 200, "Failed to create project");
        assert_eq!(response.name, "Test Project");
        assert_eq!(response.monthly_token_supply, 10000);
    }

    #[tokio::test]
    async fn test_get_project() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project first
        let (_, _, created_project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Test Project",
                "description": "A test project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };

        let project_id = created_project.id.to_string();

        // Get the project
        let (status, _header, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}", project_id),
            headers: headers.clone(),
            response_type: ProjectResponse,
        };

        assert_eq!(status, 200, "Failed to get project");
        assert_eq!(response.name, "Test Project");
    }

    #[tokio::test]
    async fn test_list_projects() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Test Project 1",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };

        // List projects
        let (status, _header, _response) = get! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            response_type: serde_json::Value,
        };

        assert_eq!(status, 200, "Failed to list projects");
    }

    #[tokio::test]
    async fn test_unauthorized_access_to_project() {
        let ctx = TestContext::setup().await;
        let (_, headers1) = &ctx.account1;
        let (_, headers2) = &ctx.account2;

        // Account 1 creates a project
        let (status, _, created_project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers1.clone(),
            body: {
                "name": "Account 1 Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200, "Failed to create project");

        let project_id = created_project.id.to_string();

        // Account 2 tries to access Account 1's project - should be FORBIDDEN (403)
        let (status, _, _) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}", project_id),
            headers: headers2.clone(),
            response_type: serde_json::Value,
        };

        assert_eq!(
            status, 403,
            "Should return 403 Forbidden when accessing another user's project"
        );
    }

    #[tokio::test]
    async fn test_unauthenticated_access_to_project() {
        let ctx = TestContext::setup().await;
        let (_, headers) = &ctx.account1;

        // Create a project
        let (status, _, created_project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200, "Failed to create project");

        let project_id = created_project.id.to_string();

        // Try to access without authentication - should be UNAUTHORIZED (401)
        let (status, _, _) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}", project_id),
            response_type: serde_json::Value,
        };

        assert_eq!(
            status, 401,
            "Should return 401 Unauthorized when not authenticated"
        );
    }

    #[tokio::test]
    async fn test_access_nonexistent_project() {
        let ctx = TestContext::setup().await;
        let (_, headers) = &ctx.account1;

        let fake_project_id = "00000000-0000-0000-0000-000000000000";

        // Try to access a non-existent project - should be NOT_FOUND (404)
        let (status, _, _) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}", fake_project_id),
            headers: headers.clone(),
            response_type: serde_json::Value,
        };

        assert_eq!(
            status, 404,
            "Should return 404 Not Found for non-existent project"
        );
    }

    // ---- Purchase endpoint tests ----

    #[tokio::test]
    async fn test_create_purchase_success() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        // Create a project
        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Purchase Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, response) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/purchases", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": "buyer-001",
                "amount": 1000,
                "item_name": "Premium Badge",
                "reward_rate": 5.0,
            },
            response_type: PurchaseResponse,
        };

        assert_eq!(status, 200, "Purchase should succeed");
        assert_eq!(response.purchase_amount, 1000);
        assert_eq!(response.reward_points, 50); // 1000 * 5.0 / 100
        assert_eq!(response.treasury_contribution, 1000);
    }

    #[tokio::test]
    async fn test_create_purchase_zero_reward_rate() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Zero Reward Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, response) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/purchases", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": "buyer-zero",
                "amount": 500,
                "item_name": "Basic Item",
                "reward_rate": 0.0,
            },
            response_type: PurchaseResponse,
        };

        assert_eq!(status, 200);
        assert_eq!(response.reward_points, 0);
        assert_eq!(response.treasury_contribution, 500);
    }

    #[tokio::test]
    async fn test_create_purchase_invalid_amount() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Invalid Purchase Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, _) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/purchases", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": "buyer-bad",
                "amount": 0,
                "item_name": "Bad Item",
                "reward_rate": 5.0,
            },
            response_type: serde_json::Value,
        };

        assert_eq!(status, 400, "Should return 400 for zero amount");
    }

    // ---- Treasury endpoint tests ----

    #[tokio::test]
    async fn test_get_treasury_empty() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Treasury Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/treasury", project_id),
            headers: headers.clone(),
            response_type: TreasuryResponse,
        };

        assert_eq!(status, 200, "Should return treasury summary");
        assert_eq!(response.total_treasury, 0);
        assert_eq!(response.total_supply, 10000);
        assert_eq!(response.circulating_supply, 10000);
        assert_eq!(response.floor_price, 0.0);
    }

    #[tokio::test]
    async fn test_get_treasury_after_purchases() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Treasury After Purchases",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        // Make a purchase that awards 100 points
        let (status, _, _) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/purchases", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": "treasury-buyer",
                "amount": 2000,
                "item_name": "Gold Badge",
                "reward_rate": 5.0,
            },
            response_type: PurchaseResponse,
        };
        assert_eq!(status, 200);

        let (status, _, response) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/treasury", project_id),
            headers: headers.clone(),
            response_type: TreasuryResponse,
        };

        assert_eq!(status, 200);
        // 2000 * 5% = 100 points awarded → supplied_points = 100 → total_treasury = 100
        assert_eq!(response.total_treasury, 100);
        assert_eq!(response.total_supply, 10000);
    }

    // ---- Activity endpoint tests ----

    #[tokio::test]
    async fn test_create_activity_success() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Activity Test Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, response) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/activities", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": "active-user-001",
                "activity_type": "login",
                "value": 500,
                "description": "Daily login bonus",
            },
            response_type: ActivityResponse,
        };

        assert_eq!(status, 200, "Activity should succeed");
        assert_eq!(response.points_earned, 5); // 500 / 100 default steps_per_point
        assert_eq!(response.total_points, 5);
    }

    #[tokio::test]
    async fn test_create_activity_custom_steps_per_point() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Custom Steps Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, response) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/activities", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": "custom-steps-user",
                "activity_type": "purchase",
                "value": 1000,
                "description": "In-app purchase",
                "steps_per_point": 10,
            },
            response_type: ActivityResponse,
        };

        assert_eq!(status, 200);
        assert_eq!(response.points_earned, 100); // 1000 / 10
    }

    #[tokio::test]
    async fn test_create_activity_accumulates_points() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Accumulate Points Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();
        let meta_user_id = "accumulator-user";

        // First activity: 500 / 100 = 5 points
        let (status, _, r1) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/activities", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": meta_user_id,
                "activity_type": "login",
                "value": 500,
                "description": "First login",
            },
            response_type: ActivityResponse,
        };
        assert_eq!(status, 200);
        assert_eq!(r1.points_earned, 5);
        assert_eq!(r1.total_points, 5);

        // Second activity: 300 / 100 = 3 points
        let (status, _, r2) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/activities", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": meta_user_id,
                "activity_type": "referral",
                "value": 300,
                "description": "Referred a friend",
            },
            response_type: ActivityResponse,
        };
        assert_eq!(status, 200);
        assert_eq!(r2.points_earned, 3);
        assert_eq!(r2.total_points, 8); // 5 + 3
    }

    #[tokio::test]
    async fn test_create_activity_invalid_value() {
        let ctx = TestContext::setup().await;
        let (_account, headers) = &ctx.account1;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers.clone(),
            body: {
                "name": "Invalid Activity Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, _) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/activities", project_id),
            headers: headers.clone(),
            body: {
                "meta_user_id": "bad-user",
                "activity_type": "login",
                "value": 0,
                "description": "Invalid activity",
            },
            response_type: serde_json::Value,
        };

        assert_eq!(status, 400, "Should return 400 for zero value");
    }

    #[tokio::test]
    async fn test_purchase_unauthorized() {
        let ctx = TestContext::setup().await;
        let (_, headers1) = &ctx.account1;
        let (_, headers2) = &ctx.account2;

        let (status, _, project) = post! {
            app: &ctx.app,
            path: "/v1/projects",
            headers: headers1.clone(),
            body: {
                "name": "Unauthorized Purchase Project",
                "monthly_points_supply": 1000000,
                "monthly_token_supply": 10000,
                "exchange_ratio": 1.0,
            },
            response_type: ProjectResponse,
        };
        assert_eq!(status, 200);

        let project_id = project.id.to_string();

        let (status, _, _) = post! {
            app: &ctx.app,
            path: format!("/v1/projects/{}/purchases", project_id),
            headers: headers2.clone(),
            body: {
                "meta_user_id": "hacker",
                "amount": 100,
                "item_name": "Stolen item",
                "reward_rate": 5.0,
            },
            response_type: serde_json::Value,
        };

        assert_eq!(status, 403, "Should return 403 for cross-project access");
    }
}
