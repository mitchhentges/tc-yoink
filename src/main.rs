extern crate reqwest;

use std::{env, fs};
use std::fs::{File};
use std::io::{Write, copy};
use serde_json::Value;
use std::path::Path;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: tc-yoink <task id>");
        std::process::exit(1);
    }

    let task_id = args.get(1).unwrap();
    let task_definition_url = format!("https://queue.taskcluster.net/v1/task/{}", task_id);
    let mut response = reqwest::get(&task_definition_url)?;
    let text = response.text()?;

    fs::create_dir_all("work_dir")?;
    let mut dest = File::create("work_dir/task.json")?;
    let json: Value = serde_json::from_str(&text)?;
    dest.write(&text.as_bytes())?;

    for upstream_task in json["payload"]["upstreamArtifacts"].as_array().unwrap() {
        let task_id = upstream_task["taskId"].as_str().unwrap().clone();
        for artifact in upstream_task["paths"].as_array().unwrap() {
            let tc_artifact_path = artifact.as_str().unwrap().clone();
            println!("Downloading: {}", tc_artifact_path);
            let path_str = format!("work_dir/cot/{}/{}", task_id, tc_artifact_path);
            let path = Path::new(&path_str);
            fs::create_dir_all(&path.parent().unwrap())?;
            let artifact_url = format!("https://queue.taskcluster.net/v1/task/{}/runs/0/artifacts/{}", task_id, tc_artifact_path);
            let mut response = reqwest::get(&artifact_url)?;
            let mut artifact_dest = File::create(path)?;
            copy(&mut response, &mut artifact_dest)?;
        }
    }
    Ok(())
}
