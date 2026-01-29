use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAssets {
    pub inventories: i64,
    pub trade_and_other_recv: i64,
    pub cash_and_cash_equiv: i64,
    pub others_current_assets: i64,
    pub total_current_assets: i64,
    pub total_assets_current_assets: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EquityAndLiabilities {
    pub share_capital: i64,
    pub other_reserves: i64,
    pub retained_earnings: i64,
    pub other_equi: i64,
    pub total_equi: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NonCurrentLiabilities {
    pub long_term_borrow: i64,
    pub deferred_tax: i64,
    pub long_term_prov: i64,
    pub others_non_current_liab: i64,
    pub total_non_current_liab: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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
