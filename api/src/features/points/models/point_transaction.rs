use crate::features::points::TransactionType;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema, OperationIo, Default)]
pub struct PointTransaction {
    #[schemars(description = "Composite key: PROJECT#<project_id>#META_USER#<meta_user_id>")]
    pub pk: CompositePartition,
    #[schemars(description = "Transaction ID: POINT_TRANSACTION#<transaction id>")]
    pub sk: EntityType,

    #[schemars(description = "Project ID")]
    pub project_id: Partition,

    #[schemars(description = "Meta user ID")]
    pub meta_user_id: String,

    #[schemars(description = "Month in YYYY-MM format")]
    pub month: String,

    #[schemars(description = "Transaction type")]
    pub transaction_type: TransactionType,

    #[schemars(description = "Amount of points")]
    pub amount: i64,

    #[schemars(description = "Target user ID for transfers")]
    pub target_user_id: Option<String>,

    #[schemars(description = "Description or memo")]
    pub description: Option<String>,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,
}

impl PointTransaction {
    pub fn new(
        project_pk: Partition,
        meta_user_id: String,
        month: String,
        transaction_type: TransactionType,
        amount: i64,
        target_user_id: Option<String>,
        description: Option<String>,
    ) -> Self {
        let created_at = time_utils::get_now();
        let uuid = uuid::Uuid::new_v4().to_string();
        let user_pk = Partition::MetaUser(meta_user_id.clone());

        Self {
            pk: CompositePartition(project_pk.clone(), user_pk),
            sk: EntityType::PointTransaction(uuid),
            project_id: project_pk,
            meta_user_id,
            month,
            transaction_type,
            amount,
            target_user_id,
            description,
            created_at,
        }
    }
}

pub struct PointTransactionQueryOption {
    pub sk: Option<String>,
    pub bookmark: Option<String>,
    pub limit: i32,
    pub scan_index_forward: bool,
    pub all: bool,
}
impl Default for PointTransactionQueryOption {
    fn default() -> Self {
        Self {
            sk: None,
            bookmark: None,
            limit: 10,
            scan_index_forward: false,
            all: false,
        }
    }
}
impl PointTransaction {
    pub fn opt() -> PointTransactionQueryOption {
        PointTransactionQueryOption::default()
    }
    pub fn opt_with_bookmark(bookmark: Option<String>) -> PointTransactionQueryOption {
        let mut opt = PointTransactionQueryOption::default();
        if let Some(bookmark) = bookmark {
            opt.bookmark = Some(bookmark);
        }
        opt
    }
    pub fn opt_one() -> PointTransactionQueryOption {
        PointTransactionQueryOption {
            limit: 1,
            ..Default::default()
        }
    }
    pub fn opt_all() -> PointTransactionQueryOption {
        PointTransactionQueryOption {
            limit: 1_000_000,
            all: true,
            ..Default::default()
        }
    }
    pub fn opt_one_with_sk(self, sk: impl std::fmt::Display) -> PointTransactionQueryOption {
        PointTransactionQueryOption {
            sk: Some(format!("{}", sk)),
            limit: 1,
            ..Default::default()
        }
    }
}
impl PointTransactionQueryOption {
    pub fn builder() -> Self {
        Self::default()
    }
    pub fn sk(mut self, sk: String) -> Self {
        self.sk = Some(format!("{}", sk));
        self
    }
    pub fn bookmark(mut self, bookmark: String) -> Self {
        self.bookmark = Some(bookmark);
        self
    }
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = limit;
        self
    }
    pub fn scan_index_forward(mut self, scan_index_forward: bool) -> Self {
        self.scan_index_forward = scan_index_forward;
        self
    }
}
impl PointTransaction {
    pub fn encode_lek_all(
        lek: &std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    ) -> std::result::Result<String, crate::Error> {
        let mut bookmark = vec![];
        for (k, v) in lek.iter() {
            match v {
                aws_sdk_dynamodb::types::AttributeValue::S(s) => {
                    bookmark.push(format!("{};;;{}", k, s));
                }
                _ => {
                    return Err(crate::Error::InternalServerError(
                        "Unsupported AttributeValue type in LEK".into(),
                    ));
                }
            }
        }
        let bookmark = bookmark.join(";;;").to_owned();
        use base64::Engine as _;
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bookmark);
        Ok(encoded)
    }
    pub fn decode_bookmark_all(
        bookmark: &str,
    ) -> std::result::Result<
        std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
        crate::Error,
    > {
        use base64::Engine as _;
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(bookmark)?;
        let s = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        let parts: Vec<&str> = s.split(";;;").collect();
        if parts.len() % 2 != 0 {
            return Err(crate::Error::InvalidBookmark);
        }
        let mut v = std::collections::HashMap::new();
        for i in (0..parts.len()).step_by(2) {
            let key = parts[i];
            let value = parts[i + 1];
            v.insert(
                key.to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(value.to_string()),
            );
        }
        Ok(v)
    }
    pub fn indexed_fields(
        &self,
        mut item: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    ) -> std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue> {
        let value = self.get_pk_for_find_by_project();
        if !value.is_empty() {
            item.insert(
                "gsi1_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(value),
            );
        }
        let value = self.get_sk_for_find_by_project();
        if !value.is_empty() {
            item.insert(
                "gsi1_sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(value),
            );
        }
        let value = self.get_pk_for_find_by_user();
        if !value.is_empty() {
            item.insert(
                "gsi2_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(value),
            );
        }
        let value = self.get_sk_for_find_by_user();
        if !value.is_empty() {
            item.insert(
                "gsi2_sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(value),
            );
        }
        item
    }
    pub async fn find_by_project(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display + Clone,
        opt: PointTransactionQueryOption,
    ) -> std::result::Result<(Vec<Self>, Option<String>), crate::Error> {
        let key_condition = if opt.sk.is_some() {
            "#pk = :pk AND begins_with(#sk, :sk)"
        } else {
            "#pk = :pk"
        };
        let mut req = cli
            .query()
            .table_name(Self::table_name())
            .index_name("gsi1-index")
            .expression_attribute_names("#pk", "gsi1_pk")
            .expression_attribute_values(
                ":pk",
                aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi1_pk(pk.clone())),
            );
        if let Some(sk) = opt.sk.clone() {
            req = req
                .expression_attribute_names("#sk", "gsi1_sk")
                .expression_attribute_values(
                    ":sk",
                    aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi1_sk(sk.clone())),
                );
        }
        if let Some(bookmark) = opt.bookmark {
            let lek = Self::decode_bookmark_all(&bookmark)?;
            req = req.set_exclusive_start_key(Some(lek));
        }
        let resp = req
            .limit(opt.limit)
            .scan_index_forward(opt.scan_index_forward)
            .key_condition_expression(key_condition)
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        let mut items = resp
            .items
            .unwrap_or_default()
            .into_iter()
            .map(|item| serde_dynamo::from_item(item))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let bookmark = if opt.all {
            let mut bookmark = resp.last_evaluated_key;
            while let Some(bm) = bookmark {
                let mut req = cli
                    .query()
                    .table_name(Self::table_name())
                    .index_name("gsi1-index")
                    .set_exclusive_start_key(Some(bm))
                    .expression_attribute_names("#pk", "gsi1_pk")
                    .expression_attribute_values(
                        ":pk",
                        aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi1_pk(
                            pk.clone(),
                        )),
                    );
                if let Some(sk) = opt.sk.clone() {
                    req = req
                        .expression_attribute_names("#sk", "gsi1_sk")
                        .expression_attribute_values(
                            ":sk",
                            aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi1_sk(
                                sk.clone(),
                            )),
                        );
                }
                let resp = req
                    .scan_index_forward(opt.scan_index_forward)
                    .key_condition_expression(key_condition)
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
                let more_items = resp
                    .items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| serde_dynamo::from_item(item))
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                items.extend(more_items);
                bookmark = resp.last_evaluated_key;
            }
            None
        } else {
            if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
                Some(Self::encode_lek_all(last_evaluated_key)?)
            } else {
                None
            }
        };
        Ok((items, bookmark))
    }
    pub async fn find_by_user(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display + Clone,
        opt: PointTransactionQueryOption,
    ) -> std::result::Result<(Vec<Self>, Option<String>), crate::Error> {
        let key_condition = if opt.sk.is_some() {
            "#pk = :pk AND begins_with(#sk, :sk)"
        } else {
            "#pk = :pk"
        };
        let mut req = cli
            .query()
            .table_name(Self::table_name())
            .index_name("gsi2-index")
            .expression_attribute_names("#pk", "gsi2_pk")
            .expression_attribute_values(
                ":pk",
                aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi2_pk(pk.clone())),
            );
        if let Some(sk) = opt.sk.clone() {
            req = req
                .expression_attribute_names("#sk", "gsi2_sk")
                .expression_attribute_values(
                    ":sk",
                    aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi2_sk(sk.clone())),
                );
        }
        if let Some(bookmark) = opt.bookmark {
            let lek = Self::decode_bookmark_all(&bookmark)?;
            req = req.set_exclusive_start_key(Some(lek));
        }
        let resp = req
            .limit(opt.limit)
            .scan_index_forward(opt.scan_index_forward)
            .key_condition_expression(key_condition)
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        let mut items = resp
            .items
            .unwrap_or_default()
            .into_iter()
            .map(|item| serde_dynamo::from_item(item))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let bookmark = if opt.all {
            let mut bookmark = resp.last_evaluated_key;
            while let Some(bm) = bookmark {
                let mut req = cli
                    .query()
                    .table_name(Self::table_name())
                    .index_name("gsi2-index")
                    .set_exclusive_start_key(Some(bm))
                    .expression_attribute_names("#pk", "gsi2_pk")
                    .expression_attribute_values(
                        ":pk",
                        aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi2_pk(
                            pk.clone(),
                        )),
                    );
                if let Some(sk) = opt.sk.clone() {
                    req = req
                        .expression_attribute_names("#sk", "gsi2_sk")
                        .expression_attribute_values(
                            ":sk",
                            aws_sdk_dynamodb::types::AttributeValue::S(Self::compose_gsi2_sk(
                                sk.clone(),
                            )),
                        );
                }
                let resp = req
                    .scan_index_forward(opt.scan_index_forward)
                    .key_condition_expression(key_condition)
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
                let more_items = resp
                    .items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| serde_dynamo::from_item(item))
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                items.extend(more_items);
                bookmark = resp.last_evaluated_key;
            }
            None
        } else {
            if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
                Some(Self::encode_lek_all(last_evaluated_key)?)
            } else {
                None
            }
        };
        Ok((items, bookmark))
    }
}
pub struct PointTransactionUpdater {
    k: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    inner: PointTransaction,
    m: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValueUpdate>,
    set_update_expressions: Vec<String>,
    remove_update_expressions: Vec<String>,
    expression_attribute_names:
        std::collections::HashMap<::std::string::String, ::std::string::String>,
    expression_attribute_values:
        std::collections::HashMap<::std::string::String, aws_sdk_dynamodb::types::AttributeValue>,
}
impl PointTransaction {
    pub fn updater(
        pk: impl std::fmt::Display,
        sk: impl std::fmt::Display,
    ) -> PointTransactionUpdater {
        let k = std::collections::HashMap::from([
            (
                "pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
            ),
            (
                "sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
            ),
        ]);
        PointTransactionUpdater {
            inner: Default::default(),
            m: std::collections::HashMap::new(),
            k,
            set_update_expressions: vec![],
            remove_update_expressions: vec![],
            expression_attribute_names: std::collections::HashMap::new(),
            expression_attribute_values: std::collections::HashMap::new(),
        }
    }
    pub fn create_transact_write_item(&self) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let item =
            serde_dynamo::to_item(self).expect("failed to serialize struct to dynamodb item");
        let item = self.indexed_fields(item);
        let req = aws_sdk_dynamodb::types::Put::builder()
            .table_name(Self::table_name())
            .condition_expression("attribute_not_exists(pk) AND attribute_not_exists(sk)")
            .set_item(Some(item))
            .build()
            .unwrap();
        aws_sdk_dynamodb::types::TransactWriteItem::builder()
            .put(req)
            .build()
    }
    pub fn upsert_transact_write_item(&self) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let item =
            serde_dynamo::to_item(self).expect("failed to serialize struct to dynamodb item");
        let item = self.indexed_fields(item);
        let req = aws_sdk_dynamodb::types::Put::builder()
            .table_name(Self::table_name())
            .set_item(Some(item))
            .build()
            .unwrap();
        aws_sdk_dynamodb::types::TransactWriteItem::builder()
            .put(req)
            .build()
    }
    pub fn delete_transact_write_item(
        pk: impl std::fmt::Display,
        sk: impl std::fmt::Display,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let k = std::collections::HashMap::from([
            (
                "pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
            ),
            (
                "sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
            ),
        ]);
        let req = aws_sdk_dynamodb::types::Delete::builder()
            .table_name(Self::table_name())
            .condition_expression("attribute_exists(pk) AND attribute_exists(sk)")
            .set_key(Some(k))
            .build()
            .unwrap();
        aws_sdk_dynamodb::types::TransactWriteItem::builder()
            .delete(req)
            .build()
    }
}
impl PointTransactionUpdater {
    pub fn with_project_id(mut self, project_id: Partition) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&project_id).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(project_id).to_string(), v);
        self.inner.project_id = project_id;
        self.set_update_expressions
            .push("#project_id = :project_id".to_string());
        self.expression_attribute_names.insert(
            "#project_id".to_string(),
            stringify!(project_id).to_string(),
        );
        self.expression_attribute_values
            .insert(":project_id".to_string(), av);
        let value = self.inner.get_pk_for_gsi1();
        if !value.is_empty() {
            self.m.insert(
                "gsi1_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(aws_sdk_dynamodb::types::AttributeValue::S(
                        self.inner.get_pk_for_gsi1(),
                    ))
                    .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                    .build(),
            );
            if !self
                .set_update_expressions
                .contains(&"#gsi1_pk = :gsi1_pk".to_string())
            {
                self.set_update_expressions
                    .push("#gsi1_pk = :gsi1_pk".to_string());
            }
            self.expression_attribute_names
                .insert("#gsi1_pk".to_string(), "gsi1_pk".to_string());
            self.expression_attribute_values.insert(
                ":gsi1_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(self.inner.get_pk_for_gsi1()),
            );
        }
        let value = self.inner.get_pk_for_gsi2();
        if !value.is_empty() {
            self.m.insert(
                "gsi2_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(aws_sdk_dynamodb::types::AttributeValue::S(
                        self.inner.get_pk_for_gsi2(),
                    ))
                    .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                    .build(),
            );
            if !self
                .set_update_expressions
                .contains(&"#gsi2_pk = :gsi2_pk".to_string())
            {
                self.set_update_expressions
                    .push("#gsi2_pk = :gsi2_pk".to_string());
            }
            self.expression_attribute_names
                .insert("#gsi2_pk".to_string(), "gsi2_pk".to_string());
            self.expression_attribute_values.insert(
                ":gsi2_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(self.inner.get_pk_for_gsi2()),
            );
        }
        self
    }
    pub fn remove_project_id(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(project_id).to_string(), v);
        self.remove_update_expressions
            .push("#project_id".to_string());
        self.expression_attribute_names.insert(
            "#project_id".to_string(),
            stringify!(project_id).to_string(),
        );
        self.m.insert(
            "gsi1_pk".to_string(),
            aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                .build(),
        );
        self.remove_update_expressions.push("#gsi1_pk".to_string());
        self.expression_attribute_names
            .insert("#gsi1_pk".to_string(), "gsi1_pk".to_string());
        self.m.insert(
            "gsi2_pk".to_string(),
            aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                .build(),
        );
        self.remove_update_expressions.push("#gsi2_pk".to_string());
        self.expression_attribute_names
            .insert("#gsi2_pk".to_string(), "gsi2_pk".to_string());
        self
    }
    pub fn with_meta_user_id(mut self, meta_user_id: String) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&meta_user_id).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(meta_user_id).to_string(), v);
        self.inner.meta_user_id = meta_user_id;
        self.set_update_expressions
            .push("#meta_user_id = :meta_user_id".to_string());
        self.expression_attribute_names.insert(
            "#meta_user_id".to_string(),
            stringify!(meta_user_id).to_string(),
        );
        self.expression_attribute_values
            .insert(":meta_user_id".to_string(), av);
        let value = self.inner.get_pk_for_gsi2();
        if !value.is_empty() {
            self.m.insert(
                "gsi2_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(aws_sdk_dynamodb::types::AttributeValue::S(
                        self.inner.get_pk_for_gsi2(),
                    ))
                    .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                    .build(),
            );
            if !self
                .set_update_expressions
                .contains(&"#gsi2_pk = :gsi2_pk".to_string())
            {
                self.set_update_expressions
                    .push("#gsi2_pk = :gsi2_pk".to_string());
            }
            self.expression_attribute_names
                .insert("#gsi2_pk".to_string(), "gsi2_pk".to_string());
            self.expression_attribute_values.insert(
                ":gsi2_pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(self.inner.get_pk_for_gsi2()),
            );
        }
        self
    }
    pub fn remove_meta_user_id(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(meta_user_id).to_string(), v);
        self.remove_update_expressions
            .push("#meta_user_id".to_string());
        self.expression_attribute_names.insert(
            "#meta_user_id".to_string(),
            stringify!(meta_user_id).to_string(),
        );
        self.m.insert(
            "gsi2_pk".to_string(),
            aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                .build(),
        );
        self.remove_update_expressions.push("#gsi2_pk".to_string());
        self.expression_attribute_names
            .insert("#gsi2_pk".to_string(), "gsi2_pk".to_string());
        self
    }
    pub fn with_month(mut self, month: String) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&month).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(month).to_string(), v);
        self.inner.month = month;
        self.set_update_expressions
            .push("#month = :month".to_string());
        self.expression_attribute_names
            .insert("#month".to_string(), stringify!(month).to_string());
        self.expression_attribute_values
            .insert(":month".to_string(), av);
        let value = self.inner.get_sk_for_gsi2();
        if !value.is_empty() {
            self.m.insert(
                "gsi2_sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(aws_sdk_dynamodb::types::AttributeValue::S(
                        self.inner.get_sk_for_gsi2(),
                    ))
                    .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                    .build(),
            );
            if !self
                .set_update_expressions
                .contains(&"#gsi2_sk = :gsi2_sk".to_string())
            {
                self.set_update_expressions
                    .push("#gsi2_sk = :gsi2_sk".to_string());
            }
            self.expression_attribute_names
                .insert("#gsi2_sk".to_string(), "gsi2_sk".to_string());
            self.expression_attribute_values.insert(
                ":gsi2_sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(self.inner.get_sk_for_gsi2()),
            );
        }
        self
    }
    pub fn remove_month(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(month).to_string(), v);
        self.remove_update_expressions.push("#month".to_string());
        self.expression_attribute_names
            .insert("#month".to_string(), stringify!(month).to_string());
        self.m.insert(
            "gsi2_sk".to_string(),
            aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                .build(),
        );
        self.remove_update_expressions.push("#gsi2_sk".to_string());
        self.expression_attribute_names
            .insert("#gsi2_sk".to_string(), "gsi2_sk".to_string());
        self
    }
    pub fn with_transaction_type(mut self, transaction_type: TransactionType) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&transaction_type).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(transaction_type).to_string(), v);
        self.inner.transaction_type = transaction_type;
        self.set_update_expressions
            .push("#transaction_type = :transaction_type".to_string());
        self.expression_attribute_names.insert(
            "#transaction_type".to_string(),
            stringify!(transaction_type).to_string(),
        );
        self.expression_attribute_values
            .insert(":transaction_type".to_string(), av);
        self
    }
    pub fn remove_transaction_type(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(transaction_type).to_string(), v);
        self.remove_update_expressions
            .push("#transaction_type".to_string());
        self.expression_attribute_names.insert(
            "#transaction_type".to_string(),
            stringify!(transaction_type).to_string(),
        );
        self
    }
    pub fn with_amount(mut self, amount: i64) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&amount).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(amount).to_string(), v);
        self.inner.amount = amount;
        self.set_update_expressions
            .push("#amount = :amount".to_string());
        self.expression_attribute_names
            .insert("#amount".to_string(), stringify!(amount).to_string());
        self.expression_attribute_values
            .insert(":amount".to_string(), av);
        self
    }
    pub fn remove_amount(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(amount).to_string(), v);
        self.remove_update_expressions.push("#amount".to_string());
        self.expression_attribute_names
            .insert("#amount".to_string(), stringify!(amount).to_string());
        self
    }
    pub fn increase_amount(mut self, by: i64) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(by).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Add)
            .build();
        self.m.insert(stringify!(amount).to_string(), v);
        self.set_update_expressions
            .push("#amount = if_not_exists(#amount, :z) + :amount".to_string());
        self.expression_attribute_names
            .insert("#amount".to_string(), stringify!(amount).to_string());
        self.expression_attribute_values
            .insert(":amount".to_string(), av.clone());
        self.expression_attribute_values.insert(
            ":z".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N("0".to_string()),
        );
        self
    }
    pub fn decrease_amount(mut self, by: i64) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(-by).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Add)
            .build();
        self.m.insert(stringify!(amount).to_string(), v);
        self.set_update_expressions
            .push("#amount = if_not_exists(#amount, :z) + :amount".to_string());
        self.expression_attribute_names
            .insert("#amount".to_string(), stringify!(amount).to_string());
        self.expression_attribute_values
            .insert(":amount".to_string(), av.clone());
        self.expression_attribute_values.insert(
            ":z".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N("0".to_string()),
        );
        self
    }
    pub fn with_target_user_id(mut self, target_user_id: String) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&target_user_id).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(target_user_id).to_string(), v);
        self.inner.target_user_id = Some(target_user_id);
        self.set_update_expressions
            .push("#target_user_id = :target_user_id".to_string());
        self.expression_attribute_names.insert(
            "#target_user_id".to_string(),
            stringify!(target_user_id).to_string(),
        );
        self.expression_attribute_values
            .insert(":target_user_id".to_string(), av);
        self
    }
    pub fn remove_target_user_id(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(target_user_id).to_string(), v);
        self.remove_update_expressions
            .push("#target_user_id".to_string());
        self.expression_attribute_names.insert(
            "#target_user_id".to_string(),
            stringify!(target_user_id).to_string(),
        );
        self
    }
    pub fn with_description(mut self, description: String) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&description).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(description).to_string(), v);
        self.inner.description = Some(description);
        self.set_update_expressions
            .push("#description = :description".to_string());
        self.expression_attribute_names.insert(
            "#description".to_string(),
            stringify!(description).to_string(),
        );
        self.expression_attribute_values
            .insert(":description".to_string(), av);
        self
    }
    pub fn remove_description(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(description).to_string(), v);
        self.remove_update_expressions
            .push("#description".to_string());
        self.expression_attribute_names.insert(
            "#description".to_string(),
            stringify!(description).to_string(),
        );
        self
    }
    pub fn with_created_at(mut self, created_at: i64) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(&created_at).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Put)
            .build();
        self.m.insert(stringify!(created_at).to_string(), v);
        self.inner.created_at = created_at;
        self.set_update_expressions
            .push("#created_at = :created_at".to_string());
        self.expression_attribute_names.insert(
            "#created_at".to_string(),
            stringify!(created_at).to_string(),
        );
        self.expression_attribute_values
            .insert(":created_at".to_string(), av);
        let value = self.inner.get_sk_for_gsi1();
        if !value.is_empty() {
            self.m.insert(
                "gsi1_sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                    .value(aws_sdk_dynamodb::types::AttributeValue::S(
                        self.inner.get_sk_for_gsi1(),
                    ))
                    .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                    .build(),
            );
            if !self
                .set_update_expressions
                .contains(&"#gsi1_sk = :gsi1_sk".to_string())
            {
                self.set_update_expressions
                    .push("#gsi1_sk = :gsi1_sk".to_string());
            }
            self.expression_attribute_names
                .insert("#gsi1_sk".to_string(), "gsi1_sk".to_string());
            self.expression_attribute_values.insert(
                ":gsi1_sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(self.inner.get_sk_for_gsi1()),
            );
        }
        self
    }
    pub fn remove_created_at(mut self) -> Self {
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
            .build();
        self.m.insert(stringify!(created_at).to_string(), v);
        self.remove_update_expressions
            .push("#created_at".to_string());
        self.expression_attribute_names.insert(
            "#created_at".to_string(),
            stringify!(created_at).to_string(),
        );
        self.m.insert(
            "gsi1_sk".to_string(),
            aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                .action(aws_sdk_dynamodb::types::AttributeAction::Delete)
                .build(),
        );
        self.remove_update_expressions.push("#gsi1_sk".to_string());
        self.expression_attribute_names
            .insert("#gsi1_sk".to_string(), "gsi1_sk".to_string());
        self
    }
    pub fn increase_created_at(mut self, by: i64) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(by).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Add)
            .build();
        self.m.insert(stringify!(created_at).to_string(), v);
        self.set_update_expressions
            .push("#created_at = if_not_exists(#created_at, :z) + :created_at".to_string());
        self.expression_attribute_names.insert(
            "#created_at".to_string(),
            stringify!(created_at).to_string(),
        );
        self.expression_attribute_values
            .insert(":created_at".to_string(), av.clone());
        self.expression_attribute_values.insert(
            ":z".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N("0".to_string()),
        );
        self
    }
    pub fn decrease_created_at(mut self, by: i64) -> Self {
        let av: aws_sdk_dynamodb::types::AttributeValue =
            serde_dynamo::to_attribute_value(-by).expect("failed to serialize field");
        let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
            .value(av.clone())
            .action(aws_sdk_dynamodb::types::AttributeAction::Add)
            .build();
        self.m.insert(stringify!(created_at).to_string(), v);
        self.set_update_expressions
            .push("#created_at = if_not_exists(#created_at, :z) + :created_at".to_string());
        self.expression_attribute_names.insert(
            "#created_at".to_string(),
            stringify!(created_at).to_string(),
        );
        self.expression_attribute_values
            .insert(":created_at".to_string(), av.clone());
        self.expression_attribute_values.insert(
            ":z".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N("0".to_string()),
        );
        self
    }
    pub fn transact_write_item(self) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let mut req = aws_sdk_dynamodb::types::Update::builder()
            .table_name(PointTransaction::table_name())
            .condition_expression("attribute_exists(pk) AND attribute_exists(sk)")
            .set_key(Some(self.k));
        let mut update_expr = "".to_string();
        if !self.remove_update_expressions.is_empty() {
            update_expr = format!("REMOVE {}", self.remove_update_expressions.join(", "));
        }
        if !self.set_update_expressions.is_empty() {
            update_expr = format!(
                "SET {} {}",
                self.set_update_expressions.join(", "),
                update_expr
            );
        };
        if !update_expr.is_empty() {
            req = req.update_expression(update_expr);
        }
        if !self.expression_attribute_names.is_empty() {
            req = req.set_expression_attribute_names(Some(self.expression_attribute_names));
        }
        if !self.expression_attribute_values.is_empty() {
            req = req.set_expression_attribute_values(Some(self.expression_attribute_values));
        }
        aws_sdk_dynamodb::types::TransactWriteItem::builder()
            .update(req.build().expect("invalid transact write item request"))
            .build()
    }
    pub fn transact_upsert_item(self) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let mut req = aws_sdk_dynamodb::types::Update::builder()
            .table_name(PointTransaction::table_name())
            .set_key(Some(self.k));
        let mut update_expr = "".to_string();
        if !self.remove_update_expressions.is_empty() {
            update_expr = format!("REMOVE {}", self.remove_update_expressions.join(", "));
        }
        if !self.set_update_expressions.is_empty() {
            update_expr = format!(
                "SET {} {}",
                self.set_update_expressions.join(", "),
                update_expr
            );
        };
        if !update_expr.is_empty() {
            req = req.update_expression(update_expr);
        }
        if !self.expression_attribute_names.is_empty() {
            req = req.set_expression_attribute_names(Some(self.expression_attribute_names));
        }
        if !self.expression_attribute_values.is_empty() {
            req = req.set_expression_attribute_values(Some(self.expression_attribute_values));
        }
        aws_sdk_dynamodb::types::TransactWriteItem::builder()
            .update(req.build().expect("invalid transact write item request"))
            .build()
    }
    pub async fn execute(
        self,
        cli: &aws_sdk_dynamodb::Client,
    ) -> std::result::Result<PointTransaction, crate::Error> {
        let res = cli
            .update_item()
            .table_name(PointTransaction::table_name())
            .set_key(Some(self.k))
            .set_attribute_updates(Some(self.m))
            .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        Ok(serde_dynamo::from_item(res.attributes.unwrap_or_default())?)
    }
}
impl PointTransaction {
    pub fn compose_gsi1_pk(key: impl std::fmt::Display) -> String {
        key.to_string()
    }
    pub fn compose_gsi2_pk(key: impl std::fmt::Display) -> String {
        key.to_string()
    }
    pub fn compose_gsi2_sk(key: impl std::fmt::Display) -> String {
        let key = key.to_string();
        if key.starts_with("MONTH#") {
            return key;
        }
        format!("{}#{}", "MONTH", key)
    }
    pub fn compose_gsi1_sk(key: impl std::fmt::Display) -> String {
        let key = key.to_string();
        if key.starts_with("TX#") {
            return key;
        }
        format!("{}#{}", "TX", key)
    }
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn with_pk(mut self, pk: CompositePartition) -> Self {
        self.pk = pk;
        self
    }
    pub fn with_sk(mut self, sk: EntityType) -> Self {
        self.sk = sk;
        self
    }
    pub fn with_project_id(mut self, project_id: Partition) -> Self {
        self.project_id = project_id;
        self
    }
    pub fn with_meta_user_id(mut self, meta_user_id: String) -> Self {
        self.meta_user_id = meta_user_id;
        self
    }
    pub fn with_month(mut self, month: String) -> Self {
        self.month = month;
        self
    }
    pub fn with_transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = transaction_type;
        self
    }
    pub fn with_amount(mut self, amount: i64) -> Self {
        self.amount = amount;
        self
    }
    pub fn with_target_user_id(mut self, target_user_id: String) -> Self {
        self.target_user_id = Some(target_user_id);
        self
    }
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_created_at(mut self, created_at: i64) -> Self {
        self.created_at = created_at;
        self
    }
    pub fn generate_pk_for_find_by_project(project_id: Partition) -> String {
        vec![project_id.to_string()].join("#")
    }
    pub fn get_pk_for_find_by_project(&self) -> String {
        vec![self.project_id.to_string()].join("#")
    }
    pub fn get_pk_for_gsi1(&self) -> String {
        vec![self.project_id.to_string()].join("#")
    }
    pub fn generate_sk_for_find_by_project(created_at: i64) -> String {
        vec!["TX".to_string(), created_at.to_string()].join("#")
    }
    pub fn get_sk_for_find_by_project(&self) -> String {
        vec!["TX".to_string(), self.created_at.to_string()].join("#")
    }
    pub fn get_sk_for_gsi1(&self) -> String {
        vec!["TX".to_string(), self.created_at.to_string()].join("#")
    }
    pub fn generate_pk_for_find_by_user(project_id: Partition, meta_user_id: String) -> String {
        vec![
            "META_USER".to_string(),
            project_id.to_string(),
            meta_user_id.to_string(),
        ]
        .join("#")
    }
    pub fn get_pk_for_find_by_user(&self) -> String {
        vec![
            "META_USER".to_string(),
            self.project_id.to_string(),
            self.meta_user_id.to_string(),
        ]
        .join("#")
    }
    pub fn get_pk_for_gsi2(&self) -> String {
        vec![
            "META_USER".to_string(),
            self.project_id.to_string(),
            self.meta_user_id.to_string(),
        ]
        .join("#")
    }
    pub fn generate_sk_for_find_by_user(month: String) -> String {
        vec!["MONTH".to_string(), month.to_string()].join("#")
    }
    pub fn get_sk_for_find_by_user(&self) -> String {
        vec!["MONTH".to_string(), self.month.to_string()].join("#")
    }
    pub fn get_sk_for_gsi2(&self) -> String {
        vec!["MONTH".to_string(), self.month.to_string()].join("#")
    }
    pub fn table_name() -> &'static str {
        "biyard-local-main"
    }
    pub fn pk_field() -> &'static str {
        "pk"
    }
    pub fn sk_field() -> Option<&'static str> {
        Some("sk")
    }
    pub async fn query(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display,
        opt: PointTransactionQueryOption,
    ) -> std::result::Result<(Vec<PointTransaction>, Option<String>), crate::Error> {
        let key_condition = if opt.sk.is_some() {
            "#pk = :pk AND begins_with(#sk, :sk)"
        } else {
            "#pk = :pk"
        };
        let mut req = cli
            .query()
            .table_name("biyard-local-main")
            .key_condition_expression(key_condition)
            .expression_attribute_names("#pk", "pk")
            .expression_attribute_values(
                ":pk",
                aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
            );
        if let Some(sk) = opt.sk {
            req = req
                .expression_attribute_names("#sk", "sk")
                .expression_attribute_values(
                    ":sk",
                    aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
                );
        }
        if let Some(bookmark) = opt.bookmark {
            let lek = Self::decode_bookmark_all(&bookmark)?;
            req = req.set_exclusive_start_key(Some(lek));
        }
        let resp = req
            .limit(opt.limit)
            .scan_index_forward(opt.scan_index_forward)
            .key_condition_expression(key_condition)
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        let items = resp
            .items
            .unwrap_or_default()
            .into_iter()
            .map(|item| serde_dynamo::from_item(item))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let bookmark = if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
            Some(Self::encode_lek_all(last_evaluated_key)?)
        } else {
            None
        };
        Ok((items, bookmark))
    }
    pub async fn query_begins_with_sk(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display,
        sk: impl std::fmt::Display,
    ) -> std::result::Result<(Vec<PointTransaction>, Option<String>), crate::Error> {
        let resp = cli
            .query()
            .table_name("biyard-local-main")
            .limit(100)
            .scan_index_forward(false)
            .key_condition_expression("#pk = :pk AND begins_with(#sk, :sk)")
            .expression_attribute_names("#pk", "pk")
            .expression_attribute_names("#sk", "sk")
            .expression_attribute_values(
                ":pk",
                aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
            )
            .expression_attribute_values(
                ":sk",
                aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
            )
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        let items = resp
            .items
            .unwrap_or_default()
            .into_iter()
            .map(|item| serde_dynamo::from_item(item))
            .collect::<std::result::Result<Vec<PointTransaction>, _>>()?;
        let bookmark = if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
            Some(Self::encode_lek_all(last_evaluated_key)?)
        } else {
            None
        };
        Ok((items, bookmark))
    }
    pub async fn create(
        &self,
        cli: &aws_sdk_dynamodb::Client,
    ) -> std::result::Result<(), crate::Error> {
        let item = serde_dynamo::to_item(self)?;
        let item = self.indexed_fields(item);
        cli.put_item()
            .table_name(Self::table_name())
            .condition_expression("attribute_not_exists(pk) AND attribute_not_exists(sk)")
            .set_item(Some(item))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        Ok(())
    }
    pub async fn upsert(
        &self,
        cli: &aws_sdk_dynamodb::Client,
    ) -> std::result::Result<(), crate::Error> {
        let item = serde_dynamo::to_item(self)?;
        let item = self.indexed_fields(item);
        cli.put_item()
            .table_name(Self::table_name())
            .set_item(Some(item))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        Ok(())
    }
    pub async fn get(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display,
        sk: Option<impl std::fmt::Display>,
    ) -> std::result::Result<Option<Self>, crate::Error> {
        let key_condition = if sk.is_some() {
            "#pk = :pk AND begins_with(#sk, :sk)"
        } else {
            "#pk = :pk"
        };
        let mut req = cli
            .query()
            .table_name(Self::table_name())
            .key_condition_expression(key_condition)
            .expression_attribute_names("#pk", Self::pk_field())
            .expression_attribute_values(
                ":pk",
                aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
            );
        if let Some(sk) = sk {
            req = req
                .expression_attribute_names("#sk", "sk")
                .expression_attribute_values(
                    ":sk",
                    aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
                );
        }
        let resp = req
            .limit(1)
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        if let Some(mut items) = resp.items {
            if let Some(item) = items.pop() {
                let ev: Self = serde_dynamo::from_item(item)?;
                Ok(Some(ev))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    pub async fn delete(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display,
        sk: Option<impl std::fmt::Display>,
    ) -> std::result::Result<Self, crate::Error> {
        let mut req = cli
            .delete_item()
            .table_name(Self::table_name())
            .condition_expression("attribute_exists(pk) AND attribute_exists(sk)")
            .key(
                Self::pk_field(),
                aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
            );
        if let Some(sk) = sk {
            req = req.key(
                Self::sk_field().expect("sk field is required"),
                aws_sdk_dynamodb::types::AttributeValue::S(format!("{}", sk)),
            );
        }
        let old = req
            .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        if let Some(item) = old.attributes {
            let ev: Self = serde_dynamo::from_item(item)?;
            Ok(ev)
        } else {
            Err("Item not found".to_string().into())
        }
    }
    pub async fn batch_get(
        cli: &aws_sdk_dynamodb::Client,
        keys: Vec<(impl std::fmt::Display, impl std::fmt::Display)>,
    ) -> std::result::Result<Vec<Self>, crate::Error> {
        if keys.is_empty() {
            return Ok(vec![]);
        }
        let keys = keys
            .iter()
            .map(|key| {
                std::collections::HashMap::from([
                    (
                        "pk".to_string(),
                        aws_sdk_dynamodb::types::AttributeValue::S(key.0.to_string()),
                    ),
                    (
                        "sk".to_string(),
                        aws_sdk_dynamodb::types::AttributeValue::S(key.1.to_string()),
                    ),
                ])
            })
            .collect::<Vec<_>>();
        let keys_and_attributes = aws_sdk_dynamodb::types::KeysAndAttributes::builder()
            .set_keys(Some(keys))
            .consistent_read(false)
            .build()
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        let table_name = Self::table_name();
        let response = cli
            .batch_get_item()
            .request_items(table_name, keys_and_attributes)
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        let items = if let Some(responses) = response.responses() {
            if let Some(items) = responses.get(table_name) {
                serde_dynamo::from_items(items.to_vec())?
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        Ok(items)
    }
}
