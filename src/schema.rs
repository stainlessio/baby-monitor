use chrono::prelude::*;

#[derive(GraphQLEnum)]
enum BranchStatus {
    Open,
    ShiproomNotify(bug_num: String),
    ShiproomApprovalRequired(bug_num: String),
    ClosedForBreakage(bug_num: String),
    RecoveringFromBreakage(bug_num: String),
}

#[derive(GraphQLObject)]
#[graphql(description="a branch that contains checkin-openness status")]
struct Branch {
    name: String,
    status: BranchStatus,
    last_updated: DateTime<Utc>,
}