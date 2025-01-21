use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Debug)]
enum TaskStatus {
    Pending,
    Success(Duration),
    Failure(String),
}

#[derive(Debug)]
enum SubTaskType {
    FileCopy,
    #[cfg(feature = "build_generate_graphs")]
    GraphGeneration,
}

struct SubTask {
    description: &'static str,
    status: TaskStatus,
    start_time: Option<Instant>,
    task_type: SubTaskType,
}

struct Task {
    description: &'static str,
    status: TaskStatus,
    sub_tasks: Vec<SubTask>,
}

struct BuildProcess {
    tasks: Vec<Task>,
}

impl BuildProcess {
    fn new() -> Self {
        BuildProcess {
            tasks: vec![
                Task {
                    description: "Copying files",
                    status: TaskStatus::Pending,
                    sub_tasks: vec![
                        SubTask {
                            description: "Copy CREDITS.md to build/web/credits/",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::FileCopy,
                        },
                        SubTask {
                            description: "Copy build/windows/icon.ico to build/web/icon.ico",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::FileCopy,
                        },
                    ],
                },
                #[cfg(feature = "build_generate_graphs")]
                Task {
                    description: "Updating Graphs",
                    status: TaskStatus::Pending,
                    sub_tasks: vec![
                        SubTask {
                            description: "Generate implicit dependency graph with deduplicated transitive dependencies",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                        SubTask {
                            description: "Generate explicit dependency graph without deduplicated transitive dependencies",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                        SubTask {
                            description: "Generate full explicit dependency graph without deduplication",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                        SubTask {
                            description: "Generate full implicit dependency graph with deduplicated transitive dependencies",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                        SubTask {
                            description: "Generate graph for internal crates only",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                        SubTask {
                            description: "Focus on wgpu-core crate for Bevy game development",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                        SubTask {
                            description: "Focus on bevy crate",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                        SubTask {
                            description: "Focus on bevy_render crate",
                            status: TaskStatus::Pending,
                            start_time: None,
                            task_type: SubTaskType::GraphGeneration,
                        },
                    ],
                },
            ],
        }
    }

    fn print_progress(&self) {
        print!("\x1B[2J\x1B[H");
        println!("Building project...\n");

        for task in &self.tasks {
            match &task.status {
                TaskStatus::Pending => println!("[ ] {} (Pending)", task.description),
                TaskStatus::Success(duration) => {
                    println!("[✓] {} (Completed in {:.2?})", task.description, duration)
                }
                TaskStatus::Failure(msg) => {
                    println!("[✗] {} (Failed: {})", task.description, msg.trim_end())
                }
            }

            for sub_task in &task.sub_tasks {
                match &sub_task.status {
                    TaskStatus::Pending => println!("    [ ] {}", sub_task.description),
                    TaskStatus::Success(duration) => {
                        println!(
                            "    [✓] {} (Completed in {:.2?})",
                            sub_task.description, duration
                        );
                    }
                    TaskStatus::Failure(msg) => {
                        println!(
                            "    [✗] {} (Failed: {})",
                            sub_task.description,
                            msg.trim_end()
                        );
                    }
                }
            }
        }

        io::stdout().flush().unwrap();
    }

    fn copy_file(source: &Path, destination: &Path) -> Result<(), String> {
        if !source.exists() {
            return Err(format!("Source file {} does not exist", source.display()));
        }

        if let Some(parent) = destination.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
        }

        fs::copy(source, destination).map_err(|e| format!("Failed to copy file: {}", e))?;

        Ok(())
    }

    #[cfg(feature = "build_generate_graphs")]
    fn run_graph_command(command: &str) -> Result<(), String> {
        let parent_path = Path::new("depgraph/crates-focus/generate-it!");
        if let Some(parent) = parent_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
        }

        let output = std::process::Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    fn process_sub_task(&mut self, task_index: usize, sub_task_index: usize) {
        let sub_task = &mut self.tasks[task_index].sub_tasks[sub_task_index];
        sub_task.start_time = Some(Instant::now());

        let result = match sub_task.task_type {
            SubTaskType::FileCopy => match sub_task_index {
                0 => BuildProcess::copy_file(Path::new("CREDITS.md"), Path::new("build/web/credits/CREDITS.md")),
                1 => BuildProcess::copy_file(Path::new("build/windows/icon.ico"), Path::new("build/web/icon.ico")),
                _ => Err("Unknown sub-task".to_string()),
            },
            #[cfg(feature = "build_generate_graphs")]
            SubTaskType::GraphGeneration => match sub_task_index {
                0 => BuildProcess::run_graph_command("cargo depgraph --all-deps --depth 1 --dedup-transitive-deps | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/implicit_graph.png"),
                1 => BuildProcess::run_graph_command("cargo depgraph --all-deps --depth 1 | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/explicit_graph.png"),
                2 => BuildProcess::run_graph_command("cargo depgraph --all-deps | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/full_explicit_graph.png"),
                3 => BuildProcess::run_graph_command("cargo depgraph --all-deps --dedup-transitive-deps | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/full_implicit_graph.png"),
                4 => BuildProcess::run_graph_command("cargo depgraph --workspace-only | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/internal_crates_graph.png"),
                5 => BuildProcess::run_graph_command("cargo depgraph --all-deps --dedup-transitive-deps --focus wgpu | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/crates-focus/wgpu_graph.png"),
                6 => BuildProcess::run_graph_command("cargo depgraph --all-deps --dedup-transitive-deps --focus bevy | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/crates-focus/bevy_graph.png"),
                7 => BuildProcess::run_graph_command("cargo depgraph --all-deps --dedup-transitive-deps --focus bevy_render | D:\\graphviz\\bin\\dot.exe -Tpng -o depgraph/crates-focus/bevy_render_graph.png"),
                _ => Err("Unknown graph generation sub-task".to_string()),
            },
        };

        match result {
            Ok(()) => {
                let elapsed = sub_task.start_time.unwrap().elapsed();
                sub_task.status = TaskStatus::Success(elapsed);
            }
            Err(msg) => sub_task.status = TaskStatus::Failure(msg),
        }

        let task = &mut self.tasks[task_index];
        if task
            .sub_tasks
            .iter()
            .any(|sub_task| matches!(sub_task.status, TaskStatus::Failure(_)))
        {
            task.status = TaskStatus::Failure("One or more sub-tasks failed".to_string());
        } else {
            let elapsed = task
                .sub_tasks
                .iter()
                .filter_map(|s| s.start_time.map(|start| start.elapsed()))
                .max()
                .unwrap_or_default();
            task.status = TaskStatus::Success(elapsed);
        }

        self.print_progress();
    }

    fn run(&mut self) {
        for i in 0..self.tasks.len() {
            self.process_task(i);
        }
        println!("\nBuild completed!");
    }

    fn process_task(&mut self, task_index: usize) {
        let task = &mut self.tasks[task_index];
        task.status = TaskStatus::Pending;

        for sub_task_index in 0..task.sub_tasks.len() {
            self.process_sub_task(task_index, sub_task_index);
        }
    }
}

fn main() {
    let mut build_process = BuildProcess::new();
    build_process.run();
}
