use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct FCP {
    pub id: u32,
    pub fk_issue: u32,
    pub fk_initiator: u32,
    pub fk_initiating_comment: u32,
    pub disposition: Option<String>,
    pub fk_bot_tracking_comment: u32,
    pub fcp_start: Option<String>,
    pub fcp_closed: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Reviewer {
    pub id: u32,
    pub login: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    pub reviewer: Reviewer,
    pub approved: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FCPIssue {
    pub id: u32,
    pub number: u32,
    pub fk_milestone: Option<String>,
    pub fk_user: u32,
    pub fk_assignee: Option<u32>,
    pub open: bool,
    pub is_pull_request: bool,
    pub title: String,
    pub body: String,
    pub locked: bool,
    pub closed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub labels: Vec<String>,
    pub repository: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusComment {
    pub id: u64,
    pub fk_issue: u32,
    pub fk_user: u32,
    pub body: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub repository: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullFCP {
    pub fcp: FCP,
    pub reviews: Vec<Review>,
    pub issue: FCPIssue,
    pub status_comment: StatusComment,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FCPDecorator {
    pub number: u64,
    pub title: String,
    pub html_url: String,
    pub repo_name: String,
    pub labels: String,
    pub assignees: String,
    pub updated_at: String,

    pub bot_tracking_comment: String,
    pub bot_tracking_comment_html_url: String,
    pub bot_tracking_comment_content: String,
    pub initiating_comment: String,
    pub initiating_comment_html_url: String,
    pub initiating_comment_content: String,
}

impl FCPDecorator {
    pub fn from_issue_fcp(
        full_fcp: &FullFCP,
        issue_decorator: &crate::actions::IssueDecorator,
    ) -> Self {
        Self {
            number: issue_decorator.number.clone(),
            title: issue_decorator.title.clone(),
            html_url: issue_decorator.html_url.clone(),
            repo_name: issue_decorator.repo_name.clone(),
            labels: issue_decorator.labels.clone(),
            assignees: issue_decorator.assignees.clone(),
            updated_at: issue_decorator.updated_at.clone(),

            bot_tracking_comment: full_fcp.fcp.fk_bot_tracking_comment.to_string(),
            bot_tracking_comment_html_url: format!(
                "{}#issuecomment-{}",
                issue_decorator.html_url, full_fcp.fcp.fk_bot_tracking_comment
            ),
            bot_tracking_comment_content: String::new(), // TODO: get from GitHub
            initiating_comment: full_fcp.fcp.fk_initiating_comment.to_string(),
            initiating_comment_html_url: format!(
                "{}#issuecomment-{}",
                issue_decorator.html_url, full_fcp.fcp.fk_initiating_comment
            ),
            initiating_comment_content: full_fcp.status_comment.body.clone(),
        }
    }
}

pub async fn get_all_fcps() -> anyhow::Result<HashMap<String, FullFCP>> {
    let url = Url::parse(&"https://rfcbot.rs/api/all")?;
    let res = reqwest::get(url).await?.json::<Vec<FullFCP>>().await?;
    println!("res: {:#?}", res);

    let mut map: HashMap<String, FullFCP> = HashMap::new();
    for full_fcp in res.into_iter() {
        map.insert(
            // format!(
            //     "https://github.com/{}/pulls/{}",
            //     full_fcp.issue.repository.clone(),
            //     full_fcp.issue.number.clone()
            // ),
            format!(
                "{}:{}:{}",
                full_fcp.issue.repository.clone(),
                full_fcp.issue.number.clone(),
                full_fcp.issue.title.clone(),
            ),
            full_fcp,
        );
    }

    Ok(map)
}
