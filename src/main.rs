extern crate csv;
extern crate reqwest;
extern crate clap; 
extern crate serde; 
use serde::{Serialize, Deserialize};
use std::error::Error;
use csv::StringRecord;

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    title: String, 
    body: String, 
    labels: Vec<String>, 
    milestone: String,
}

struct Github {
    repository: String, 
    username: String,
    token: String
}



fn convert(record: &StringRecord) -> Result<Issue,Box<Error>> {
    // priority,category,status,description,comments
    let title = record.get(3).unwrap().into();
    let body = record.get(4).unwrap().into();
    let labels: Vec<String> = vec![record.get(1).unwrap().into()];
    let milestone = record.get(0).unwrap().into();

    Ok(Issue {
        title, body,labels, milestone
    })
}

fn parse() -> std::result::Result<Vec<Issue>,Box<Error>> {
    let mut rdr = csv::Reader::from_path("input.csv")?;
    let mut res = Vec::new();
    for result in rdr.records() {
        let issue = convert(&result?).unwrap();
        res.push(issue);
    }
    Ok(res)
}


fn import(github: &Github, issues: &[Issue])  {
    let url = format!("https://api.github.com/repos/{}/issues",github.repository);

    let client = reqwest::Client::new();

    for issue in issues {
        let res = client.post(&url)
            .json(&issue)
            .basic_auth(&github.username, Some(&github.token))
            .send().unwrap();
        // Just ignore failures...
    }
}


fn main() {
    
    let github = Github {
        repository: "<REPOSITORY>".into(), 
        username: "<USERNAME>".into(),
        token: "<TOKEN>".into(),
    };


    import(&github, &parse().unwrap())
}
