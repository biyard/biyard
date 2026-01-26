use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct SubmitContactRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub company_name: String,
    pub needs: Need,
    pub help: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct SubmitContactResponse {
    #[schemars(description = "Status of the operation")]
    pub id: ContactPartition,
}

pub async fn submit_contact_handler(
    State(AppState { cli, .. }): State<AppState>,
    Json(req): Json<SubmitContactRequest>,
) -> Result<Json<SubmitContactResponse>> {
    let SubmitContactRequest {
        first_name,
        last_name,
        email,
        company_name,
        needs,
        help,
    } = req;

    notify!(
        "New contact submission from {}({} {})",
        email,
        first_name,
        last_name
    );

    let c = Contact::new(last_name, first_name, email, company_name, needs, help);
    c.create(&cli).await?;

    Ok(Json(SubmitContactResponse { id: c.pk.into() }))
}
