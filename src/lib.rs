mod config;
pub mod models;
pub mod parser;
pub mod processor;

// Re-export commonly used types
pub use models::*;

// Constants
pub const ALL_SECTIONS: [&str; 17] = [
    "Company Details",
    "Business Details",
    "Particulars of Stated Capital",
    "Certificate (Issued by Other Institutions)",
    "Office Bearers",
    "Shareholders",
    "Members (Applicable for Company Limited by Guarantee or Shares and Guarantee)",
    "Annual Return filed for last 3 years",
    "Financial Summary/Statements filed for last 3 years",
    "Last Financial Summary Filed",
    "Profit and Loss Statement",
    "Balance Sheet",
    "Charges",
    "Removal/Winding Up Details",
    "Objections",
    "Last Annual Registration Fee Paid",
    "Extract of file with additional comments",
];
