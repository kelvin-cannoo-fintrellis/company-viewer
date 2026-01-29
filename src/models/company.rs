use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompanyData {
    pub organisation_info: OrganisationInfo,
    pub view_all: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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
    pub balance_sheet: super::financial::BalanceSheet,
    pub profit_and_loss: super::financial::ProfitAndLoss,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BusinessDetailsList {
    pub business_details: Vec<BusinessDetails>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatedCapital {
    pub share_type: String,
    pub num_shares: String,
    pub currency: String,
    pub stated_capital1: String,
    pub par_value: String,
    pub amount_unpaid: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatedCapitalList {
    pub stated_capitals: Vec<StatedCapital>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub certif: String,
    pub certif_type: String,
    pub effective_date: String,
    pub expiry_date: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CertificateList {
    pub certificates: Vec<Certificate>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OfficeBearer {
    pub position: String,
    pub name: String,
    pub address: String,
    pub country: String,
    pub appointed_date: String,
    pub entity_type: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OfficeBearerList {
    pub office_bearers: Vec<OfficeBearer>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ShareHolder {
    pub name: String,
    pub num_shares: String,
    pub share_type: String,
    pub currency: String,
    pub entity_type: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ShareHolderList {
    pub share_holders: Vec<ShareHolder>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Financial {
    pub financial_year_ended_date: String,
    pub currency: String,
    pub date_approved: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReturn {
    pub annual_return_date: String,
    pub annual_meeting_date: String,
    pub filed_date: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReturnList {
    pub annual_returns: Vec<AnnualReturn>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationFee {
    pub amount: String,
}
