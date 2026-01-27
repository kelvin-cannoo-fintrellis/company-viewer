use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OllamaRequest<'a> {
    pub model: &'a str,
    pub prompt: &'a str,
    pub stream: bool,
    pub format: &'a str,
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyData {
    pub organisation_info: OrganisationInfo,
    pub view_all: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganisationInfo {
    pub company_details: CompanyDetails,
    pub business_details_list: Vec<BusinessDetails>,
    pub stated_capitals_list: Vec<StatedCapital>,
    pub certificates_list: Vec<Certificate>,
    pub office_bearers_list: Vec<OfficeBearer>,
    pub share_holders_list: Vec<ShareHolder>,
    pub financials_list: Vec<Financial>,
    pub liquidators_list: Vec<serde_json::Value>,
    pub annual_return_list: Vec<AnnualReturn>,
    pub receivers_list: Vec<serde_json::Value>,
    pub administrators_list: Option<serde_json::Value>,
    pub charges_list: Vec<serde_json::Value>,
    pub members_list: Option<serde_json::Value>,
    pub winding_up_details_list: Vec<serde_json::Value>,
    pub objections_list: Vec<serde_json::Value>,
    pub last_annual_registration_fee_paid: RegistrationFee,
    pub additional_notes_list: Option<serde_json::Value>,
    pub balance_sheet: BalanceSheet,
    pub profit_and_loss: ProfitAndLoss,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyDetails {
    pub org_no: String,
    pub org_file_no: String,
    pub org_name: String,
    pub org_incorp_date: String,
    pub org_nature_cd: String,
    pub org_nature_cd_code: String,
    pub org_type_cd: String,
    pub org_last_sta_cd: String,
    pub company_address: String,
    pub category_desc: String,
    pub org_category_code: String,
    pub org_sub_category_code: Option<String>,
    pub sub_category_desc: Option<String>,
    pub defunct_date: Option<String>,
    pub effective_start_date: String,
    pub former_org_name: String,
    pub total_comprehensive_income: String,
    pub winding_up_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessDetails {
    pub bus_file_no: String,
    pub business_reg_no: String,
    pub bsn_business_name: String,
    pub business_type: Option<String>,
    pub main_address: String,
    pub bus_nature: String,
    pub status: Option<String>,
    pub app_name: Option<String>,
    pub bus_reg_dt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatedCapital {
    pub share_type: String,
    pub num_shares: String,
    pub currency: String,
    pub stated_capital1: String,
    pub par_value: String,
    pub amount_unpaid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub certif: String,
    pub certif_type: String,
    pub effective_date: String,
    pub expiry_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficeBearer {
    pub position: String,
    pub name: String,
    pub address: String,
    pub appointed_date: String,
    pub entity_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareHolder {
    pub name: String,
    pub num_shares: String,
    pub share_type: String,
    pub currency: String,
    pub entity_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Financial {
    pub financial_year_ended_date: String,
    pub currency: String,
    pub date_approved: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReturn {
    pub annual_return_date: String,
    pub annual_meeting_date: String,
    pub filed_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationFee {
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheet {
    pub non_current_assets: NonCurrentAssets,
    pub current_assets: CurrentAssets,
    pub equity_and_liabilities: EquityAndLiabilities,
    pub non_current_liabilities: NonCurrentLiabilities,
    pub current_liabilities: CurrentLiabilities,
    pub financial_year: String,
    pub currency: String,
    pub unit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonCurrentAssets {
    pub prop_plant_equip: i64,
    pub invest_prop: i64,
    pub intangible_assets: i64,
    pub invest_in_sub: i64,
    pub other_inv: i64,
    pub biological_assets: i64,
    pub others_non_current: i64,
    pub total_non_current: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAssets {
    pub inventories: i64,
    pub trade_and_other_recv: i64,
    pub cash_and_cash_equiv: i64,
    pub others_current_assets: i64,
    pub total_current_assets: i64,
    pub total_assets_current_assets: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityAndLiabilities {
    pub share_capital: i64,
    pub other_reserves: i64,
    pub retained_earnings: i64,
    pub other_equi: i64,
    pub total_equi: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonCurrentLiabilities {
    pub long_term_borrow: i64,
    pub deferred_tax: i64,
    pub long_term_prov: i64,
    pub others_non_current_liab: i64,
    pub total_non_current_liab: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentLiabilities {
    pub trade_and_other_pay: i64,
    pub short_term_borrowings: i64,
    pub current_tax_payable: i64,
    pub short_term_prov: i64,
    pub others_current_liab: i64,
    pub total_current_liab: i64,
    pub total_liab: i64,
    pub total_equity_and_liab: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfitAndLoss {
    pub turnover: i64,
    pub cost_of_sales: i64,
    pub gross_profit: i64,
    pub other_income: i64,
    pub distribution_costs: i64,
    pub administration_costs: i64,
    pub other_expenses: i64,
    pub finance_costs: i64,
    pub profit_before_tax: i64,
    pub tax_expense: i64,
    pub profit_for_the_period: i64,
    pub financial_year: String,
    pub currency: String,
    pub approved_date: String,
    pub unit: i32,
}
