use std::{cell::RefCell, fmt::Display, sync::Arc};

use aoc::parser;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Clone)]
pub enum Command {
    #[from_str(regex = "cd (?P<path>.*)")]
    Cd { path: String },
    #[from_str(regex = "ls\n(?s)(?P<output>.*)")]
    Ls { output: Files },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Files(Vec<File>);

impl std::str::FromStr for Files {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Files(
            s.lines()
                .map(|line| File::from_str(line).unwrap())
                .collect(),
        ))
    }
}

#[derive(FromStr, PartialEq, Debug, Clone)]
pub enum File {
    #[from_str(regex = "^(?P<size>[0-9]+) (?P<name>.*)$")]
    File { size: usize, name: String },
    #[from_str(regex = "^dir (?P<name>.*)$")]
    Dir { name: String },
}

impl File {
    pub fn name(&self) -> &str {
        match self {
            File::File { name, .. } | File::Dir { name } => name,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            File::File { size, name } => write!(f, "{} (file, size={})", name, size),
            File::Dir { name } => write!(f, "{} (dir)", name),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub value: File,
    pub parent: Option<Arc<Graph>>,
    pub childrens: RefCell<Vec<Arc<Graph>>>,
}

impl Graph {
    pub fn init() -> Arc<Graph> {
        Arc::new(Graph {
            value: File::Dir {
                name: String::from("/"),
            },
            parent: None,
            childrens: RefCell::new(Vec::new()),
        })
    }

    pub fn create_file(self: &Arc<Self>, file: File) {
        self.childrens.borrow_mut().push(Arc::new(Graph {
            value: file,
            parent: Some(self.clone()),
            childrens: RefCell::new(Vec::new()),
        }))
    }

    pub fn change_directory(self: &mut Arc<Self>, path: &str) {
        if path == ".." {
            let parent = self.parent.as_ref().unwrap().clone();
            let _ = std::mem::replace(self, parent);
        } else if path == "/" {
            while self.parent.is_some() {
                self.change_directory("..");
            }
        } else {
            let child = self
                .childrens
                .borrow()
                .iter()
                .find(|dir| dir.value.name() == path)
                .unwrap()
                .clone();
            let _ = std::mem::replace(self, child);
        }
    }

    pub fn size(&self) -> usize {
        match self.value {
            File::File { .. } => return 0,
            File::Dir { .. } => (),
        }

        self.childrens
            .borrow()
            .iter()
            .map(|child| match child.value {
                File::File { size, .. } => size,
                File::Dir { .. } => child.size(),
            })
            .sum::<usize>()
    }

    pub fn traverse(self: Arc<Self>) -> Vec<Arc<Self>> {
        let mut v = self
            .childrens
            .borrow()
            .iter()
            .map(|a| a.clone().traverse())
            .fold(Vec::new(), |mut v, mut a| {
                v.append(&mut a);
                v
            });

        v.push(self.clone());
        v
    }

    pub fn parse() -> Arc<Graph> {
        let commands: Vec<Command> = parser::input::<String>()
            .split("$ ")
            .map(|command| command.trim())
            .filter(|command| !command.is_empty())
            .map(|command| command.parse::<Command>().unwrap())
            .collect();

        let mut fs = Graph::init();

        for command in commands.into_iter().skip(1) {
            match command {
                Command::Cd { path } => fs.change_directory(&path),
                Command::Ls { output } => {
                    for file in output.0 {
                        fs.create_file(file);
                    }
                }
            }
        }
        fs.change_directory("/");

        fs
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        writeln!(f, "{} - {}", "  ".repeat(depth), self.value)?;

        for child in self.childrens.borrow().iter() {
            child.display(f, depth + 1)?;
        }

        Ok(())
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f, 0)
    }
}
