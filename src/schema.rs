use chrono::prelude::*;

#[derive(GraphQLEnum)]
pub enum BranchStatus {
    Open,
    ShiproomNotify,           // bug_num must be set
    ShiproomApprovalRequired, // bug_num must be set
    ClosedForBreakage,        // bug_num must be set
    RecoveringFromBreakage,   // bug_num must be set
}

#[derive(GraphQLObject)]
#[graphql(description="a branch that contains checkin-openness status")]
pub struct Branch {
    pub name: String,
    status: BranchStatus,
    last_updated: DateTime<Utc>,
    bug_num: Option<String>,
}

impl Branch {
    pub fn new(name: String) -> Branch {
        Branch {
            name: name,
            status: BranchStatus::Open,
            last_updated: Utc::now(),
            bug_num: None
        }
    }

    pub fn set_status(self, status: &BranchStatus) -> Result<Branch, ()> {
        // TODO: Enforce Bug Number
        Ok(Branch {
            status: *status,
            last_updated: Utc::now(),
            ..self
        })
    }
}