use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use crate::libs::http::GithubHttpClient;
use crate::libs::issues::{IssueHelper, IssueReq};
use crate::libs::http::Result;


#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct Message {
    text: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct ArtifactLocation {
    uri: String,
    #[serde(default)]
    #[serde(rename = "uri_base_id")]
    uri_base_id: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct Region {
    #[serde(default)]
    #[serde(rename = "start_line")]
    start_line: u64,
    #[serde(default)]
    #[serde(rename = "start_column")]
    start_column: u64,
    #[serde(default)]
    #[serde(rename = "end_line")]
    end_line: u64,
    #[serde(default)]
    #[serde(rename = "end_column")]
    end_column: u64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct PhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: ArtifactLocation,
    region: Region,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct Location {
    #[serde(rename = "physicalLocation")]
    physical_location: PhysicalLocation,
    #[serde(default)]
    message: Message,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct Description {
    text: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct DefaultConfiguration {
    level: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct Help {
    text: String,
    markdown: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct DriverRuleProperties {
    #[serde(default)]
    precision: String,
    #[serde(rename = "security-severity")]
    security_severity: String,
    #[serde(default)]
    tags: Vec<String>,
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct DriverRule {
    id: String,
    name: String,

    #[serde(rename = "shortDescription")]
    short_description: Description,

    #[serde(rename = "fullDescription")]
    full_description: Description,

    #[serde(default)]
    #[serde(rename = "defaultConfiguration")]
    default_configuration: DefaultConfiguration,
    #[serde(rename = "helpUri")]
    help_uri: String,
    help: Help,
    properties: DriverRuleProperties,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct ResultRule {
    #[serde(rename = "ruleId")]
    rule_id: String,
    #[serde(default)]
    #[serde(rename = "ruleIndex")]
    rule_index: u64,
    #[serde(default)]
    level: String,
    #[serde(default)]
    message: Message,
    locations: Vec<Location>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct Driver {
    #[serde(default)]
    #[serde(rename = "fullName")]
    full_name: String,
    #[serde(rename = "informationUri")]
    information_uri: String,
    name: String,
    version: String,
    rules: Vec<DriverRule>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct Tool {
    driver: Driver,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct SarifProperties {
    #[serde(rename = "imageName")]
    image_name: String,
    #[serde(rename = "repoDigests")]
    repo_digests: Vec<String>,
    #[serde(rename = "repoTags")]
    repo_tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
struct SarifInner {
    tool: Tool,
    results: Vec<ResultRule>,
    #[serde(default)]
    #[serde(rename = "columnKind")]
    column_kind: String,
    #[serde(default)]
    properties: SarifProperties,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct Sarif {
    version: String,
    #[serde(rename = "$schema")]
    schema: String,
    runs: Vec<SarifInner>,
}

impl Sarif {
    pub fn parse_sarif_issue(&self) -> Vec<SarifIssue> {
        let mut sarif_issues: Vec<SarifIssue> = Vec::new();

        if !self.runs.is_empty() {
            for run in self.runs.iter() {
                let mut tools_map: HashMap<String, HashMap<String, String>> = HashMap::with_capacity(run.tool.driver.rules.len());
                for rule in run.tool.driver.rules.iter() {
                    let mut data = HashMap::new();
                    data.insert(String::from("driver"), run.tool.driver.name.clone());
                    data.insert(String::from("rule_id"), rule.id.clone());
                    data.insert(String::from("help"), rule.help.markdown.clone());
                    tools_map.insert(rule.id.clone(), data);
                }
                for result in run.results.iter() {
                    let mut sarif_issue = SarifIssue::default();

                    let cve_id = result.rule_id.clone();
                    let location: String = result.locations.iter()
                        .map(|x| x.physical_location.artifact_location.uri.clone())
                        .collect::<Vec<String>>().join(",");
                    sarif_issue.location = location;

                    sarif_issue.message = result.message.text.clone();

                    if let Some(rule) = tools_map.get(&cve_id) {
                        if let Some(rule_id) = rule.get("rule_id") {
                            sarif_issue.rule_id = rule_id.clone();
                        }
                        if let Some(driver) = rule.get("driver") {
                            sarif_issue.driver = driver.clone();
                        }

                        if let Some(help) = rule.get("help") {
                            sarif_issue.help = help.clone();
                        }
                    }
                    sarif_issues.push(sarif_issue);
                }
            }
        }

        sarif_issues
    }
}


pub async fn upload_sarif_report(client: GithubHttpClient, file: Vec<&str>, token: &str, owner: &str, repo: &str) -> Result<()> {
    let mut issue_helper = IssueHelper::new(client);
    for f in file.iter() {
        let file = File::open(f).unwrap();
        let reader = BufReader::new(file);
        let sarif: Sarif = serde_json::from_reader(reader).unwrap();
        let sarif_issues = sarif.parse_sarif_issue();

        for sarif_issue in sarif_issues.iter() {
            let mut issue: IssueReq = IssueReq::default();
            issue.title = sarif_issue.rule_id.clone();
            issue.body = sarif_issue.body();
            issue.assignees = vec![String::from("smartcat999")];
            issue.milestone = None;
            issue.labels = vec![String::from("bug")];

            issue_helper.create_issue_unique(token, owner, repo, &issue).await?;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct SarifIssue {
    pub rule_id: String,
    pub driver: String,
    pub location: String,
    pub message: String,
    pub help: String,
}

impl SarifIssue {
    pub fn body(&self) -> String {
        format!("{}\n | Tool | Location |\n| --- | --- |\n|{}|{}|\n\n{}\n\n{}\n", self.rule_id, self.driver, self.location, self.message, self.help)
    }
}


#[cfg(test)]
mod test {
    use serde_json;

    use std::fs::File;
    use std::io::BufReader;
    use tokio_test;
    use crate::libs::http::GithubHttpClient;
    use crate::libs::issues::{IssueHelper, IssueReq};

    use super::*;

    const TOKEN: &str = "*****";
    const OWNER: &str = "smartcat999";
    const REPO: &str = "issue_auto_report";

    #[test]
    fn test_unmarshal() {
        let file = File::open("./result.sarif").unwrap();
        let reader = BufReader::new(file);
        let sarif: Sarif = serde_json::from_reader(reader).unwrap();
        println!("sarif: {:?}", sarif);
    }

    #[test]
    fn test_parse_sarif_issue() {
        let file = File::open("./result.sarif").unwrap();
        let reader = BufReader::new(file);
        let sarif: Sarif = serde_json::from_reader(reader).unwrap();
        let sarif_issues = sarif.parse_sarif_issue();
        println!("sarif_issues: {:?}", sarif_issues);
    }

    #[test]
    fn test_sarif_body() {
        let file = File::open("./result.sarif").unwrap();
        let reader = BufReader::new(file);
        let sarif: Sarif = serde_json::from_reader(reader).unwrap();
        let sarif_issues = sarif.parse_sarif_issue();
        println!("{:#?}", sarif_issues[0].body())
    }

    #[test]
    fn test_create_sarif_issue() {
        let issue_helper = IssueHelper::new(GithubHttpClient::default());
        let file = File::open("./result.sarif").unwrap();
        let reader = BufReader::new(file);
        let sarif: Sarif = serde_json::from_reader(reader).unwrap();
        let sarif_issues = sarif.parse_sarif_issue();

        let mut issue: IssueReq = IssueReq::default();
        issue.title = sarif_issues[0].rule_id.clone();
        issue.body = sarif_issues[0].body();
        issue.assignees = vec![String::from("smartcat999")];
        issue.milestone = None;
        issue.labels = vec![String::from("bug")];

        let result = issue_helper.create_issue(TOKEN, OWNER, REPO, issue.body());
        tokio_test::block_on(result).expect("fetch error");
    }

    #[test]
    fn test_create_unique_sarif_issue() {
        let result = upload_sarif_report(GithubHttpClient::default(), vec!["./results.sarif"], TOKEN, OWNER, REPO);
        tokio_test::block_on(result).expect("fetch error");
    }
}