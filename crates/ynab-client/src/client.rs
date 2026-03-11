use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use ynab_types::*;

use crate::error::YnabError;
use crate::rate_limit::RateLimiter;

const BASE_URL: &str = "https://api.ynab.com/v1";

/// HTTP client for the YNAB API.
#[derive(Clone)]
pub struct YnabClient {
    http: reqwest::Client,
    token: String,
    rate_limiter: std::sync::Arc<RateLimiter>,
}

/// Represents a planned HTTP request (for --dry-run).
#[derive(Debug, Clone, serde::Serialize)]
pub struct DryRunRequest {
    pub method: String,
    pub url: String,
    pub headers: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

impl YnabClient {
    pub fn new(token: String) -> Result<Self, YnabError> {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|e| YnabError::Other(format!("Invalid token: {e}")))?;
        headers.insert(AUTHORIZATION, auth_value);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(format!("ynab-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            http,
            token,
            rate_limiter: std::sync::Arc::new(RateLimiter::new()),
        })
    }

    /// Build a DryRunRequest for display purposes.
    pub fn dry_run_request(
        &self,
        method: &str,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> DryRunRequest {
        let mut headers = std::collections::HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}...", &self.token[..self.token.len().min(8)]),
        );
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        DryRunRequest {
            method: method.to_string(),
            url: format!("{BASE_URL}{path}"),
            headers,
            body: body.cloned(),
        }
    }

    fn check_rate_limit(&self) -> Result<(), YnabError> {
        match self.rate_limiter.check() {
            Ok(remaining) if remaining <= 10 => {
                eprintln!("Warning: {remaining} API requests remaining in current window");
                Ok(())
            }
            Ok(_) => Ok(()),
            Err(wait) => Err(YnabError::RateLimited {
                retry_after_secs: wait.as_secs(),
            }),
        }
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, YnabError> {
        self.check_rate_limit()?;
        self.rate_limiter.record();
        let url = format!("{BASE_URL}{path}");
        let response = self.http.get(&url).send().await?;
        self.handle_response(response).await
    }

    async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, YnabError> {
        self.check_rate_limit()?;
        self.rate_limiter.record();
        let url = format!("{BASE_URL}{path}");
        let response = self.http.post(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn put<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, YnabError> {
        self.check_rate_limit()?;
        self.rate_limiter.record();
        let url = format!("{BASE_URL}{path}");
        let response = self.http.put(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn patch<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, YnabError> {
        self.check_rate_limit()?;
        self.rate_limiter.record();
        let url = format!("{BASE_URL}{path}");
        let response = self.http.patch(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn delete_request<T: DeserializeOwned>(&self, path: &str) -> Result<T, YnabError> {
        self.check_rate_limit()?;
        self.rate_limiter.record();
        let url = format!("{BASE_URL}{path}");
        let response = self.http.delete(&url).send().await?;
        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, YnabError> {
        let status = response.status().as_u16();

        if status == 429 {
            return Err(YnabError::RateLimited {
                retry_after_secs: 60,
            });
        }

        if !response.status().is_success() {
            let body = response.text().await?;
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&body) {
                return Err(YnabError::from_api_error(status, error_response.error));
            }
            return Err(YnabError::Other(format!("HTTP {status}: {body}")));
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(YnabError::from)
    }

    // --- User ---

    pub async fn get_user(&self) -> Result<User, YnabError> {
        let resp: ApiResponse<UserData> = self.get("/user").await?;
        Ok(resp.data.user)
    }

    // --- Plans ---

    pub async fn get_plans(&self) -> Result<PlansData, YnabError> {
        let resp: ApiResponse<PlansData> = self.get("/plans").await?;
        Ok(resp.data)
    }

    pub async fn get_plan(&self, plan_id: &str) -> Result<PlanDetailData, YnabError> {
        let resp: ApiResponse<PlanDetailData> = self.get(&format!("/plans/{plan_id}")).await?;
        Ok(resp.data)
    }

    pub async fn get_plan_settings(&self, plan_id: &str) -> Result<PlanSettings, YnabError> {
        let resp: ApiResponse<PlanSettingsData> =
            self.get(&format!("/plans/{plan_id}/settings")).await?;
        Ok(resp.data.settings)
    }

    // --- Accounts ---

    pub async fn get_accounts(
        &self,
        plan_id: &str,
        last_knowledge: Option<i64>,
    ) -> Result<AccountsData, YnabError> {
        let mut path = format!("/plans/{plan_id}/accounts");
        if let Some(k) = last_knowledge {
            path.push_str(&format!("?last_knowledge_of_server={k}"));
        }
        let resp: ApiResponse<AccountsData> = self.get(&path).await?;
        Ok(resp.data)
    }

    pub async fn get_account(&self, plan_id: &str, account_id: &str) -> Result<Account, YnabError> {
        let resp: ApiResponse<AccountData> = self
            .get(&format!("/plans/{plan_id}/accounts/{account_id}"))
            .await?;
        Ok(resp.data.account)
    }

    pub async fn create_account(
        &self,
        plan_id: &str,
        account: &SaveAccount,
    ) -> Result<Account, YnabError> {
        let body = serde_json::json!({ "account": account });
        let resp: ApiResponse<AccountData> = self
            .post(&format!("/plans/{plan_id}/accounts"), &body)
            .await?;
        Ok(resp.data.account)
    }

    // --- Transactions ---

    pub async fn get_transactions(
        &self,
        plan_id: &str,
        since_date: Option<&str>,
        transaction_type: Option<&str>,
        last_knowledge: Option<i64>,
    ) -> Result<TransactionsData, YnabError> {
        let mut params = Vec::new();
        if let Some(d) = since_date {
            params.push(format!("since_date={d}"));
        }
        if let Some(t) = transaction_type {
            params.push(format!("type={t}"));
        }
        if let Some(k) = last_knowledge {
            params.push(format!("last_knowledge_of_server={k}"));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        let resp: ApiResponse<TransactionsData> = self
            .get(&format!("/plans/{plan_id}/transactions{query}"))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transaction(
        &self,
        plan_id: &str,
        transaction_id: &str,
    ) -> Result<TransactionDetail, YnabError> {
        let resp: ApiResponse<TransactionData> = self
            .get(&format!("/plans/{plan_id}/transactions/{transaction_id}"))
            .await?;
        Ok(resp.data.transaction)
    }

    pub async fn create_transaction(
        &self,
        plan_id: &str,
        transaction: &SaveTransaction,
    ) -> Result<SaveTransactionsData, YnabError> {
        let body = serde_json::json!({ "transaction": transaction });
        let resp: ApiResponse<SaveTransactionsData> = self
            .post(&format!("/plans/{plan_id}/transactions"), &body)
            .await?;
        Ok(resp.data)
    }

    pub async fn update_transaction(
        &self,
        plan_id: &str,
        transaction_id: &str,
        transaction: &serde_json::Value,
    ) -> Result<TransactionDetail, YnabError> {
        let body = serde_json::json!({ "transaction": transaction });
        let resp: ApiResponse<TransactionData> = self
            .put(
                &format!("/plans/{plan_id}/transactions/{transaction_id}"),
                &body,
            )
            .await?;
        Ok(resp.data.transaction)
    }

    pub async fn delete_transaction(
        &self,
        plan_id: &str,
        transaction_id: &str,
    ) -> Result<TransactionDetail, YnabError> {
        let resp: ApiResponse<TransactionData> = self
            .delete_request(&format!("/plans/{plan_id}/transactions/{transaction_id}"))
            .await?;
        Ok(resp.data.transaction)
    }

    pub async fn get_transactions_by_account(
        &self,
        plan_id: &str,
        account_id: &str,
        since_date: Option<&str>,
        last_knowledge: Option<i64>,
    ) -> Result<TransactionsData, YnabError> {
        let mut params = Vec::new();
        if let Some(d) = since_date {
            params.push(format!("since_date={d}"));
        }
        if let Some(k) = last_knowledge {
            params.push(format!("last_knowledge_of_server={k}"));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        let resp: ApiResponse<TransactionsData> = self
            .get(&format!(
                "/plans/{plan_id}/accounts/{account_id}/transactions{query}"
            ))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transactions_by_category(
        &self,
        plan_id: &str,
        category_id: &str,
        since_date: Option<&str>,
        last_knowledge: Option<i64>,
    ) -> Result<HybridTransactionsData, YnabError> {
        let mut params = Vec::new();
        if let Some(d) = since_date {
            params.push(format!("since_date={d}"));
        }
        if let Some(k) = last_knowledge {
            params.push(format!("last_knowledge_of_server={k}"));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        let resp: ApiResponse<HybridTransactionsData> = self
            .get(&format!(
                "/plans/{plan_id}/categories/{category_id}/transactions{query}"
            ))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transactions_by_payee(
        &self,
        plan_id: &str,
        payee_id: &str,
        since_date: Option<&str>,
        last_knowledge: Option<i64>,
    ) -> Result<HybridTransactionsData, YnabError> {
        let mut params = Vec::new();
        if let Some(d) = since_date {
            params.push(format!("since_date={d}"));
        }
        if let Some(k) = last_knowledge {
            params.push(format!("last_knowledge_of_server={k}"));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        let resp: ApiResponse<HybridTransactionsData> = self
            .get(&format!(
                "/plans/{plan_id}/payees/{payee_id}/transactions{query}"
            ))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_transactions_by_month(
        &self,
        plan_id: &str,
        month: &str,
        last_knowledge: Option<i64>,
    ) -> Result<TransactionsData, YnabError> {
        let query = match last_knowledge {
            Some(k) => format!("?last_knowledge_of_server={k}"),
            None => String::new(),
        };
        let resp: ApiResponse<TransactionsData> = self
            .get(&format!(
                "/plans/{plan_id}/months/{month}/transactions{query}"
            ))
            .await?;
        Ok(resp.data)
    }

    pub async fn import_transactions(&self, plan_id: &str) -> Result<ImportData, YnabError> {
        let resp: ApiResponse<ImportData> = self
            .post(
                &format!("/plans/{plan_id}/transactions/import"),
                &serde_json::json!({}),
            )
            .await?;
        Ok(resp.data)
    }

    pub async fn update_transactions_bulk(
        &self,
        plan_id: &str,
        transactions: &[serde_json::Value],
    ) -> Result<SaveTransactionsData, YnabError> {
        let body = serde_json::json!({ "transactions": transactions });
        let resp: ApiResponse<SaveTransactionsData> = self
            .patch(&format!("/plans/{plan_id}/transactions"), &body)
            .await?;
        Ok(resp.data)
    }

    // --- Categories ---

    pub async fn get_categories(
        &self,
        plan_id: &str,
        last_knowledge: Option<i64>,
    ) -> Result<CategoriesData, YnabError> {
        let query = match last_knowledge {
            Some(k) => format!("?last_knowledge_of_server={k}"),
            None => String::new(),
        };
        let resp: ApiResponse<CategoriesData> = self
            .get(&format!("/plans/{plan_id}/categories{query}"))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_category(
        &self,
        plan_id: &str,
        category_id: &str,
    ) -> Result<Category, YnabError> {
        let resp: ApiResponse<CategoryData> = self
            .get(&format!("/plans/{plan_id}/categories/{category_id}"))
            .await?;
        Ok(resp.data.category)
    }

    // --- Payees ---

    pub async fn get_payees(
        &self,
        plan_id: &str,
        last_knowledge: Option<i64>,
    ) -> Result<PayeesData, YnabError> {
        let query = match last_knowledge {
            Some(k) => format!("?last_knowledge_of_server={k}"),
            None => String::new(),
        };
        let resp: ApiResponse<PayeesData> =
            self.get(&format!("/plans/{plan_id}/payees{query}")).await?;
        Ok(resp.data)
    }

    pub async fn get_payee(&self, plan_id: &str, payee_id: &str) -> Result<Payee, YnabError> {
        let resp: ApiResponse<PayeeData> = self
            .get(&format!("/plans/{plan_id}/payees/{payee_id}"))
            .await?;
        Ok(resp.data.payee)
    }

    // --- Months ---

    pub async fn get_months(
        &self,
        plan_id: &str,
        last_knowledge: Option<i64>,
    ) -> Result<MonthsData, YnabError> {
        let query = match last_knowledge {
            Some(k) => format!("?last_knowledge_of_server={k}"),
            None => String::new(),
        };
        let resp: ApiResponse<MonthsData> =
            self.get(&format!("/plans/{plan_id}/months{query}")).await?;
        Ok(resp.data)
    }

    pub async fn get_month(&self, plan_id: &str, month: &str) -> Result<MonthDetail, YnabError> {
        let resp: ApiResponse<MonthDetailData> = self
            .get(&format!("/plans/{plan_id}/months/{month}"))
            .await?;
        Ok(resp.data.month)
    }

    // --- Scheduled Transactions ---

    pub async fn get_scheduled_transactions(
        &self,
        plan_id: &str,
        last_knowledge: Option<i64>,
    ) -> Result<ScheduledTransactionsData, YnabError> {
        let query = match last_knowledge {
            Some(k) => format!("?last_knowledge_of_server={k}"),
            None => String::new(),
        };
        let resp: ApiResponse<ScheduledTransactionsData> = self
            .get(&format!("/plans/{plan_id}/scheduled_transactions{query}"))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_scheduled_transaction(
        &self,
        plan_id: &str,
        scheduled_transaction_id: &str,
    ) -> Result<ScheduledTransactionDetail, YnabError> {
        let resp: ApiResponse<ScheduledTransactionData> = self
            .get(&format!(
                "/plans/{plan_id}/scheduled_transactions/{scheduled_transaction_id}"
            ))
            .await?;
        Ok(resp.data.scheduled_transaction)
    }

    // --- Payee Locations ---

    pub async fn get_payee_locations(
        &self,
        plan_id: &str,
    ) -> Result<Vec<PayeeLocation>, YnabError> {
        let resp: ApiResponse<PayeeLocationsData> = self
            .get(&format!("/plans/{plan_id}/payee_locations"))
            .await?;
        Ok(resp.data.payee_locations)
    }

    pub async fn get_payee_location(
        &self,
        plan_id: &str,
        payee_location_id: &str,
    ) -> Result<PayeeLocation, YnabError> {
        let resp: ApiResponse<PayeeLocationData> = self
            .get(&format!(
                "/plans/{plan_id}/payee_locations/{payee_location_id}"
            ))
            .await?;
        Ok(resp.data.payee_location)
    }

    pub async fn get_payee_locations_by_payee(
        &self,
        plan_id: &str,
        payee_id: &str,
    ) -> Result<Vec<PayeeLocation>, YnabError> {
        let resp: ApiResponse<PayeeLocationsData> = self
            .get(&format!(
                "/plans/{plan_id}/payees/{payee_id}/payee_locations"
            ))
            .await?;
        Ok(resp.data.payee_locations)
    }

    // --- Money Movements ---

    pub async fn get_money_movements(
        &self,
        plan_id: &str,
        last_knowledge: Option<i64>,
    ) -> Result<MoneyMovementsData, YnabError> {
        let query = match last_knowledge {
            Some(k) => format!("?last_knowledge_of_server={k}"),
            None => String::new(),
        };
        let resp: ApiResponse<MoneyMovementsData> = self
            .get(&format!("/plans/{plan_id}/money_movements{query}"))
            .await?;
        Ok(resp.data)
    }

    pub async fn get_money_movement_groups(
        &self,
        plan_id: &str,
        last_knowledge: Option<i64>,
    ) -> Result<MoneyMovementGroupsData, YnabError> {
        let query = match last_knowledge {
            Some(k) => format!("?last_knowledge_of_server={k}"),
            None => String::new(),
        };
        let resp: ApiResponse<MoneyMovementGroupsData> = self
            .get(&format!("/plans/{plan_id}/money_movement_groups{query}"))
            .await?;
        Ok(resp.data)
    }
}
