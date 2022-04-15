use core::panic;
use std::path::Path;

use clap::{arg, Command};
use once_cell::sync::OnceCell;
use regex::Regex;
use tokio::{
    self,
    fs::{self, File},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use tracing::info;

use crate::{
    fetch::Problems,
    formatter::{
        build_desc, insert_return_in_code, parse_discuss_link, parse_extra_use, parse_problem_link,
    },
    PROBLEMS,
};

pub type Result<T> = std::result::Result<T, failure::Error>;

pub struct Cli<'a> {
    problems: &'a OnceCell<Problems>,
}

impl<'a> Cli<'a> {
    pub async fn init() -> Result<Cli<'a>> {
        Ok(Cli {
            problems: &PROBLEMS,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Running");

        let problem_handle = tokio::spawn(async { Problems::new().await });

        let content = fs::read_to_string("./src/problem_sets/mod.rs")
            .await
            .unwrap();
        #[allow(unused_mut)]
        let mut initialized_id = get_initialized_id(content);

        let matches = Command::new("leetcode.rs")
            .version("0.1")
            .author("kh")
            .about("Fastly download problems from Leetcode to local and generate rust templates")
            .arg(
                arg!(-i --id <id>"generate problem by a frontend problem <id>")
                    .takes_value(true)
                    .required(false),
            )
            .arg(arg!(-r --random "generate a random problem").takes_value(false))
            .arg(
                arg!(-s --solve <id> "solve the problem's <id>")
                    .takes_value(true)
                    .required(false),
            )
            .get_matches();

        if let Some(problems) = problem_handle.await? {
            PROBLEMS.set(problems).unwrap();
        } else {
            panic!("Failed to init Problems");
        }

        if let Some(id) = matches.value_of("solve") {
            let id_number = id
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("not a number: {}", id));
            self.deal_solving(id_number).await;
        }

        if let Some(id) = matches.value_of("id") {
            let id_number = id
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("not a number: {}", id));
            if initialized_id.contains(&id_number) {
                println!(
                    "\nThe problem {} you chose has been initialized in problems/",
                    id
                );
            }
            self.deal_problem(id_number).await;
        }

        if matches.is_present("random") {
            let id = generate_random_id(&initialized_id);
            self.deal_problem(id).await;
            println!("Generate random problem: {}", &id);
        }

        Ok(())
    }

    async fn deal_problem(&self, id: u32) {
        let problem = self
            .problems
            .get()
            .unwrap()
            .get_problem_by_id(id)
            .await
            .unwrap_or_else(|| {
                panic!(
                    "Error: failed to get problem #{} (The problem may be paid-only or may not be \
                     exist).",
                    id
                )
            });

        let code = problem.code_definition.iter().find(|&d| d.value == *"rust");
        if code.is_none() {
            println!("Problem {} has no rust version.", problem.question_id);
            return;
        }
        let code = code.unwrap();

        let file_name = format!(
            "p{:04}_{}",
            problem.question_id,
            problem.title_slug.replace("-", "_")
        );
        let file_path = Path::new("./src/problem_sets").join(format!("{}.rs", file_name));
        if file_path.exists() {
            panic!("Problem {} already exists", problem.question_id);
        }
        let template = fs::read_to_string("./template.rs").await.unwrap();
        let source = template
            .replace("__PROBLEM_TITLE__", &problem.title)
            .replace("__PROBLEM_DESC__", &build_desc(&problem.content))
            .replace(
                "__PROBLEM_DEFAULT_CODE__",
                &insert_return_in_code(&problem.return_type, &code.default_code),
            )
            .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
            .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
            .replace("__PROBLEM_LINK__", &parse_problem_link(&problem))
            .replace("__DISCUSS_LINK__", &parse_discuss_link(&problem));

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .await
            .unwrap();

        file.write_all(source.as_bytes()).await.unwrap();
        drop(file);
        insert_mod_file("./src/problem_sets/mod.rs", &file_name).await;
    }

    async fn deal_solving(&self, id: u32) {
        let problem = PROBLEMS.get().unwrap().get_problem_by_id(id).await.unwrap();
        let file_name = format!(
            "p{:04}_{}",
            problem.question_id,
            problem.title_slug.replace("-", "_")
        );
        let file_path = Path::new("./src/problem_sets").join(format!("{}.rs", file_name));
        if !file_path.exists() {
            panic!("Problem {} does not init", id);
        }

        let solution_name = format!(
            "s{:04}_{}",
            problem.question_id,
            problem.title_slug.replace("-", "_")
        );
        let solution_path = Path::new("./src/solution_sets").join(format!("{}.rs", solution_name));
        if solution_path.exists() {
            panic!("Solution {} exists", id);
        }
        fs::rename(file_path, solution_path).await.unwrap();
        let mod_file = "./src/problem_sets/mod.rs";
        let target_line = format!("mod {};", file_name);
        let mut mod_lines = vec![];
        let mut lines = BufReader::new(File::open(mod_file).await.unwrap()).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if line != target_line {
                mod_lines.push(line);
            }
        }

        let mut mod_str = mod_lines.join("\n");
        mod_str.push_str("\n");

        fs::write(mod_file, mod_str).await.unwrap();
        // insert into solution_sets/mod.rs
        insert_mod_file("./src/solution_sets/mod.rs", &solution_name).await;
        insert_readme_file(id, solution_name, problem.title_slug, problem.difficulty).await;
    }
}

fn generate_random_id(initialized_id: &[u32]) -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    loop {
        let res: u32 = rng.gen_range(1..=2021);
        if !initialized_id.contains(&res) {
            return res;
        }
        println!(
            "Generate a random num ({}), but it is invalid (the problem may have been solved or \
             may have no rust version). Regenerate..",
            res
        );
    }
}

fn get_initialized_id(content: String) -> Vec<u32> {
    let id_pattern = Regex::new(r"p(\d{4})_").unwrap();
    id_pattern
        .captures_iter(&content)
        .map(|x| x.get(1).unwrap().as_str().parse().unwrap())
        .collect()
}

async fn insert_mod_file(path: &str, name: &str) {
    let mut mod_file = fs::OpenOptions::new()
        .append(true)
        .open(path)
        .await
        .unwrap();

    mod_file
        .write_all(format!("mod {};\n", name).as_bytes())
        .await
        .unwrap();
}

async fn insert_readme_file(
    id: u32,
    solution_name: String,
    title_slug: String,
    difficulty: String,
) {
    let mut readme_file = fs::OpenOptions::new()
        .append(true)
        .open("./README.md")
        .await
        .unwrap();

    readme_file
        .write_all(
            format!(
                "   <tr>
        <td>{:04}</td>
        <td><a href=\"./src/solution_sets/{}.rs\"> {}</a></td>
        <td>{}</td>
    </tr>\n",
                id, solution_name, title_slug, difficulty
            )
            .as_bytes(),
        )
        .await
        .unwrap();
}
