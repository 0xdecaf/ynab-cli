//! API types for the YNAB (You Need a Budget) API.
//!
//! All monetary amounts are in **milliunits** format (divide by 1000 for currency units).
//! For example, $25.00 is represented as 25000.

use serde::{Deserialize, Serialize};

// --- Enums ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Checking,
    Savings,
    Cash,
    CreditCard,
    LineOfCredit,
    OtherAsset,
    OtherLiability,
    Mortgage,
    AutoLoan,
    StudentLoan,
    PersonalLoan,
    MedicalDebt,
    OtherDebt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ClearedStatus {
    Cleared,
    Uncleared,
    Reconciled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FlagColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    #[serde(rename = "")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Transaction,
    Subtransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DebtTransactionType {
    Payment,
    Refund,
    Fee,
    Interest,
    Escrow,
    BalanceAdjustment,
    Credit,
    Charge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GoalType {
    TB,
    TBD,
    MF,
    NEED,
    DEBT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ScheduledFrequency {
    Never,
    Daily,
    Weekly,
    EveryOtherWeek,
    TwiceAMonth,
    Every4Weeks,
    Monthly,
    EveryOtherMonth,
    Every3Months,
    Every4Months,
    TwiceAYear,
    Yearly,
    EveryOtherYear,
}

// --- Format types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateFormat {
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyFormat {
    pub iso_code: String,
    pub example_format: String,
    pub decimal_digits: i32,
    pub decimal_separator: String,
    pub symbol_first: bool,
    pub group_separator: String,
    pub currency_symbol: String,
    pub display_symbol: bool,
}

// --- Core resource types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSummary {
    pub id: String,
    pub name: String,
    pub last_modified_on: Option<String>,
    pub first_month: Option<String>,
    pub last_month: Option<String>,
    pub date_format: Option<DateFormat>,
    pub currency_format: Option<CurrencyFormat>,
    pub accounts: Option<Vec<Account>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDetail {
    // Flattened from PlanSummary
    pub id: String,
    pub name: String,
    pub last_modified_on: Option<String>,
    pub first_month: Option<String>,
    pub last_month: Option<String>,
    pub date_format: Option<DateFormat>,
    pub currency_format: Option<CurrencyFormat>,
    pub accounts: Option<Vec<Account>>,
    pub payees: Option<Vec<Payee>>,
    pub payee_locations: Option<Vec<PayeeLocation>>,
    pub category_groups: Option<Vec<CategoryGroup>>,
    pub categories: Option<Vec<Category>>,
    pub months: Option<Vec<MonthDetail>>,
    pub transactions: Option<Vec<TransactionSummary>>,
    pub subtransactions: Option<Vec<SubTransaction>>,
    pub scheduled_transactions: Option<Vec<ScheduledTransactionSummary>>,
    pub scheduled_subtransactions: Option<Vec<ScheduledSubTransaction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSettings {
    pub date_format: Option<DateFormat>,
    pub currency_format: Option<CurrencyFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: AccountType,
    pub on_budget: bool,
    pub closed: bool,
    pub note: Option<String>,
    /// Balance in milliunits
    pub balance: i64,
    /// Cleared balance in milliunits
    pub cleared_balance: i64,
    /// Uncleared balance in milliunits
    pub uncleared_balance: i64,
    pub transfer_payee_id: Option<String>,
    pub direct_import_linked: Option<bool>,
    pub direct_import_in_error: Option<bool>,
    pub last_reconciled_at: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub category_group_id: String,
    pub category_group_name: Option<String>,
    pub name: String,
    pub hidden: bool,
    pub note: Option<String>,
    /// Assigned amount in milliunits
    pub budgeted: i64,
    /// Activity amount in milliunits
    pub activity: i64,
    /// Available balance in milliunits
    pub balance: i64,
    pub goal_type: Option<GoalType>,
    pub goal_target: Option<i64>,
    pub goal_target_date: Option<String>,
    pub goal_percentage_complete: Option<i32>,
    pub goal_months_to_budget: Option<i32>,
    pub goal_under_funded: Option<i64>,
    pub goal_overall_funded: Option<i64>,
    pub goal_overall_left: Option<i64>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryGroup {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryGroupWithCategories {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub deleted: bool,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payee {
    pub id: String,
    pub name: String,
    pub transfer_account_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayeeLocation {
    pub id: String,
    pub payee_id: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub id: String,
    pub date: String,
    /// Amount in milliunits
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ClearedStatus,
    pub approved: bool,
    pub flag_color: Option<FlagColor>,
    pub flag_name: Option<String>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub import_payee_name: Option<String>,
    pub import_payee_name_original: Option<String>,
    pub debt_transaction_type: Option<DebtTransactionType>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetail {
    // Flattened from TransactionSummary
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ClearedStatus,
    pub approved: bool,
    pub flag_color: Option<FlagColor>,
    pub flag_name: Option<String>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub import_payee_name: Option<String>,
    pub import_payee_name_original: Option<String>,
    pub debt_transaction_type: Option<DebtTransactionType>,
    pub deleted: bool,
    // TransactionDetail-specific fields
    pub account_name: String,
    pub payee_name: Option<String>,
    pub category_name: Option<String>,
    pub subtransactions: Vec<SubTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridTransaction {
    // Flattened from TransactionSummary
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ClearedStatus,
    pub approved: bool,
    pub flag_color: Option<FlagColor>,
    pub flag_name: Option<String>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: bool,
    // HybridTransaction-specific fields
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub parent_transaction_id: Option<String>,
    pub account_name: String,
    pub payee_name: Option<String>,
    pub category_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTransaction {
    pub id: String,
    pub transaction_id: String,
    /// Amount in milliunits
    pub amount: i64,
    pub memo: Option<String>,
    pub payee_id: Option<String>,
    pub payee_name: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthSummary {
    pub month: String,
    pub note: Option<String>,
    /// Income in milliunits
    pub income: i64,
    /// Budgeted amount in milliunits
    pub budgeted: i64,
    /// Activity amount in milliunits
    pub activity: i64,
    /// Ready to assign in milliunits
    pub to_be_budgeted: i64,
    pub age_of_money: Option<i32>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthDetail {
    pub month: String,
    pub note: Option<String>,
    pub income: i64,
    pub budgeted: i64,
    pub activity: i64,
    pub to_be_budgeted: i64,
    pub age_of_money: Option<i32>,
    pub deleted: bool,
    pub categories: Option<Vec<Category>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTransactionSummary {
    pub id: String,
    pub date_first: String,
    pub date_next: String,
    pub frequency: ScheduledFrequency,
    /// Amount in milliunits
    pub amount: i64,
    pub memo: Option<String>,
    pub flag_color: Option<FlagColor>,
    pub flag_name: Option<String>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTransactionDetail {
    pub id: String,
    pub date_first: String,
    pub date_next: String,
    pub frequency: ScheduledFrequency,
    pub amount: i64,
    pub memo: Option<String>,
    pub flag_color: Option<FlagColor>,
    pub flag_name: Option<String>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub deleted: bool,
    pub account_name: String,
    pub payee_name: Option<String>,
    pub category_name: Option<String>,
    pub subtransactions: Vec<ScheduledSubTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledSubTransaction {
    pub id: String,
    pub scheduled_transaction_id: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub payee_id: Option<String>,
    pub payee_name: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub transfer_account_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyMovement {
    pub id: String,
    pub month: String,
    pub moved_at: Option<String>,
    pub note: Option<String>,
    pub money_movement_group_id: Option<String>,
    pub performed_by_user_id: Option<String>,
    pub amount: i64,
    pub category_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyMovementGroup {
    pub id: String,
    pub group_created_at: Option<String>,
    pub month: String,
    pub note: Option<String>,
    pub performed_by_user_id: Option<String>,
}

// --- Save/Create types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveAccount {
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: AccountType,
    /// Balance in milliunits
    pub balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveTransaction {
    pub account_id: String,
    pub date: String,
    /// Amount in milliunits
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<ClearedStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_color: Option<FlagColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtransactions: Option<Vec<SaveSubTransaction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSubTransaction {
    /// Amount in milliunits
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavePayee {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveCategory {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_target: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_target_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMonthCategory {
    /// Budgeted amount in milliunits
    pub budgeted: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveCategoryGroup {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveScheduledTransaction {
    pub account_id: String,
    pub date: String,
    /// Amount in milliunits
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_color: Option<FlagColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduledFrequency>,
}

// --- Error types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub id: String,
    pub name: String,
    pub detail: String,
}

// --- API response wrappers ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlansData {
    pub plans: Vec<PlanSummary>,
    pub default_plan: Option<PlanSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDetailData {
    pub plan: PlanDetail,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSettingsData {
    pub settings: PlanSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsData {
    pub accounts: Vec<Account>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountData {
    pub account: Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriesData {
    pub category_groups: Vec<CategoryGroupWithCategories>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryData {
    pub category: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsData {
    pub transactions: Vec<TransactionDetail>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub transaction: TransactionDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridTransactionsData {
    pub transactions: Vec<HybridTransaction>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayeesData {
    pub payees: Vec<Payee>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayeeData {
    pub payee: Payee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayeeLocationsData {
    pub payee_locations: Vec<PayeeLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayeeLocationData {
    pub payee_location: PayeeLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthsData {
    pub months: Vec<MonthSummary>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthDetailData {
    pub month: MonthDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTransactionsData {
    pub scheduled_transactions: Vec<ScheduledTransactionDetail>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTransactionData {
    pub scheduled_transaction: ScheduledTransactionDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveTransactionsData {
    pub transaction_ids: Option<Vec<String>>,
    pub transaction: Option<TransactionDetail>,
    pub transactions: Option<Vec<TransactionDetail>>,
    pub duplicate_import_ids: Option<Vec<String>>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportData {
    pub transaction_ids: Vec<String>,
    pub duplicate_import_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyMovementsData {
    pub money_movements: Vec<MoneyMovement>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyMovementGroupsData {
    pub money_movement_groups: Vec<MoneyMovementGroup>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveCategoryData {
    pub category: Category,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveCategoryGroupData {
    pub category_group: CategoryGroup,
    pub server_knowledge: i64,
}

/// Helper to format milliunits as a human-readable currency string.
pub fn format_milliunits(milliunits: i64) -> String {
    let whole = milliunits / 1000;
    let frac = (milliunits % 1000).unsigned_abs();
    if milliunits < 0 && whole == 0 {
        format!("-{whole}.{frac:03}")
    } else {
        format!("{whole}.{frac:03}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_milliunits() {
        assert_eq!(format_milliunits(25000), "25.000");
        assert_eq!(format_milliunits(-100500), "-100.500");
        assert_eq!(format_milliunits(0), "0.000");
        assert_eq!(format_milliunits(-500), "-0.500");
    }
}
