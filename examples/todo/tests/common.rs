#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

pub struct TodoTest {
    pub temp_dir: TempDir,
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
}

impl TodoTest {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let home_path = temp_dir.path().to_str().unwrap().to_string();
        let cwd = temp_dir.path().into();

        let mut test = Self {
            temp_dir,
            cwd,
            env: HashMap::new(),
        };

        // Set HOME to temp dir so todo data is stored in temp dir
        test.env.insert("HOME".into(), home_path);

        test
    }

    pub fn todo(&self, args: &[&str]) -> TodoOutput {
        let mut cmd = self.todo_command();
        cmd.args(args);

        let output = cmd.output().expect("Failed to execute todo command");
        TodoOutput::new(output)
    }

    pub fn todo_command(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
        cmd.current_dir(&self.cwd);
        cmd.env_clear().envs(&self.env);
        cmd
    }
}

pub struct TodoOutput {
    pub output: std::process::Output,
}

impl TodoOutput {
    pub fn new(output: std::process::Output) -> Self {
        Self { output }
    }

    pub fn success(&self) -> bool {
        self.output.status.success()
    }

    #[track_caller]
    pub fn assert_success(&self) -> &Self {
        assert!(
            self.success(),
            "Expected command to succeed, got:\n\n# STDERR\n{}\n# STDOUT\n{}\n# STATUS {:?}",
            std::str::from_utf8(&self.output.stderr).unwrap(),
            std::str::from_utf8(&self.output.stdout).unwrap(),
            self.output.status
        );
        self
    }

    #[track_caller]
    pub fn assert_failure(&self) -> &Self {
        assert!(
            !self.success(),
            "Expected command to fail, got:\n\n# STDERR\n{}\n# STDOUT\n{}\n# STATUS {:?}",
            std::str::from_utf8(&self.output.stderr).unwrap(),
            std::str::from_utf8(&self.output.stdout).unwrap(),
            self.output.status
        );
        self
    }

    pub fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.stdout).to_string()
    }

    pub fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.output.stderr).to_string()
    }

    #[track_caller]
    pub fn assert_stdout_contains(&self, expected: &str) -> &Self {
        let stdout = self.stdout();
        assert!(
            stdout.contains(expected),
            "Expected stdout to contain '{}', got:\n{}",
            expected,
            stdout
        );
        self
    }

    #[track_caller]
    pub fn assert_stderr_contains(&self, expected: &str) -> &Self {
        let stderr = self.stderr();
        assert!(
            stderr.contains(expected),
            "Expected stderr to contain '{}', got:\n{}",
            expected,
            stderr
        );
        self
    }
}
