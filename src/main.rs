use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path, path::PathBuf, fs, fs::OpenOptions};
use rand::Rng;
use chrono::{Local, Duration};
use dialoguer::{theme::ColorfulTheme, Select, console::Term};
use directories::UserDirs;
use clap::Command;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Task {
    start_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    total_account: i32,
    tasks: BTreeMap<String, i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    job: Job,
    task_list: BTreeMap<String, BTreeMap<String, Task>>,
}

fn main() {
    let _version = Command::new("version")
            .version(env!("CARGO_PKG_VERSION"))
            .get_matches();

    let config_path = get_config_path();
    let mut config: Config = get_config(&config_path);

    check_task(&config_path, &mut config);

    draw_gui(&config_path, &mut config);
}

fn get_accounts(config: &Config) -> Vec<String> {
    let mut accounts = vec![];
    for (account, tasks) in &config.task_list {
        for task in tasks.values() {
            if task.start_time == Local::now().format("%Y-%m-%d").to_string()
            {
                accounts.push(account.to_string());
                break;
            }
        }
    }
    accounts
}

fn get_tasks(config: &Config, account: &str) -> Vec<String> {
    let mut tasks = vec![];
    if let Some(account_tasks) = config.task_list.get(account) {
        for (task, task_info) in account_tasks {
            let now = Local::now().format("%Y-%m-%d").to_string();
            if task_info.start_time == now
            {
                tasks.push(task.to_string());
            }
        }
    }
    tasks
}

fn draw_gui(config_path: &Path, config: &mut Config) {
    let term = Term::stdout();
    loop {
        let accounts = get_accounts(&config);
        if accounts.is_empty() {
            println!("当前无任务，按CTRL+C退出程序");
            std::thread::sleep(std::time::Duration::from_secs(60 * 60));
        }

        let account = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("选择账号")
            .default(0)
            .items(&accounts)
            .interact_on_opt(&term)
            .unwrap()
        {
            Some(index) => accounts[index].clone(),
            None => panic!("用户没有选择账号。"),
        };

        loop {
            let tasks = get_tasks(&config, &account);
            if tasks.is_empty() {
                break;
            }

            let task = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("选择任务")
                .default(0)
                .items(&tasks)
                .interact_on_opt(&term)
                .unwrap()
            {
                Some(index) => tasks[index].clone(),
                None => panic!("User did not select a task."),
            };

            // 将选中的任务安排到下一个日期
            let interval = config.job.tasks[&task];
            let now = Local::now();
            
            let duration = Duration::try_days(interval as i64).expect("Invalid number of days");
            let next_day = (now + duration).format("%Y-%m-%d").to_string();
            
            config
                .task_list
                .get_mut(&account)
                .unwrap()
                .get_mut(&task)
                .unwrap()
                .start_time = next_day.to_string();

            let new_data = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
            fs::write(config_path, new_data).expect("Unable to write file");
        }
    }
}

fn get_config_path() -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    let config_path = PathBuf::from(user_dirs.home_dir());
    let config_path = config_path.join("ctaskconfig.json");
    config_path
}

fn get_config(config_path: &Path) -> Config {
    // checkout
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config_path)
        .expect("Unable to open file");

    let metadata = fs::metadata(&config_path).expect("Unable to get metadata");
    if metadata.len() == 0 {
        let default_config = 
r#"{
    "job": {
        "total_account": 0,
        "tasks": {}
    },
    "task_list": {}
}"#;

        fs::write(&config_path, default_config).expect("Unable to write file");
    }

    let data = fs::read_to_string(config_path).expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).unwrap();
    config
}

fn check_task(config_path: &Path, config: &mut Config) {
    let mut rng = rand::thread_rng();

    for (_account, tasks) in config.task_list.iter_mut() {
        tasks.retain(|task, _| config.job.tasks.contains_key(task));
    }
    for (task, count) in config.job.tasks.iter() {
        for i in 0..config.job.total_account {
            let account = format!("{:03}", i + 1);
    
            let days = rng.gen_range(0..*count);
            
            if *count != 1 {
                let now = Local::now();
                let duration = Duration::try_days(days as i64).expect("Invalid number of days");
                let set_day = (now + duration).format("%Y-%m-%d").to_string();
                
                let task_entry = Task {
                    start_time: set_day,
                };
                
                if !config.task_list.get(&account).map_or(false, |tasks| tasks.contains_key(task)) {
                    config
                        .task_list
                        .entry(account.clone())
                        .or_insert_with(BTreeMap::new)
                        .insert(task.to_string(), task_entry);
                }
            } else {
                let now = Local::now();
                let duration = Duration::try_days(days as i64).expect("Invalid number of days");
                let set_day = (now + duration).format("%Y-%m-%d").to_string();
                
                let task_entry = Task {
                    start_time: set_day,
                };
                
                if !config.task_list.get(&account).map_or(false, |tasks| tasks.contains_key(task)) {
                    config
                        .task_list
                        .entry(account.clone())
                        .or_insert_with(BTreeMap::new)
                        .insert(task.to_string(), task_entry);
                }
            }
    
            if *count == 0 {
                config.task_list.get_mut(&account).unwrap().remove(task);
            }
        }
    }
    let new_data = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    fs::write(&config_path, new_data).expect("Unable to write file");
}