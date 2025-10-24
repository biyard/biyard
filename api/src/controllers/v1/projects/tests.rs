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
        assert_eq!(response.monthly_points_supply, 1000000);
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

        let project_id = created_project.pk.to_string().replace("Project(", "").replace(")", "");

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

        let project_id = created_project.pk.to_string().replace("Project(", "").replace(")", "");

        // Account 2 tries to access Account 1's project - should be FORBIDDEN (403)
        let (status, _, _) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}", project_id),
            headers: headers2.clone(),
            response_type: serde_json::Value,
        };

        assert_eq!(status, 403, "Should return 403 Forbidden when accessing another user's project");
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

        let project_id = created_project.pk.to_string().replace("Project(", "").replace(")", "");

        // Try to access without authentication - should be UNAUTHORIZED (401)
        let (status, _, _) = get! {
            app: &ctx.app,
            path: format!("/v1/projects/{}", project_id),
            response_type: serde_json::Value,
        };

        assert_eq!(status, 401, "Should return 401 Unauthorized when not authenticated");
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

        assert_eq!(status, 404, "Should return 404 Not Found for non-existent project");
    }
}
