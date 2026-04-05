mod common;

use common::TodoTest;

#[test]
fn test_add_todo() {
    let test = TodoTest::new();
    let output = test.todo(&["add", "Learn Rust basics"]);
    output
        .assert_success()
        .assert_stdout_contains("Added todo #1")
        .assert_stdout_contains("Learn Rust basics");
}

#[test]
fn test_list_todos() {
    let test = TodoTest::new();
    test.todo(&["add", "First todo"]).assert_success();
    test.todo(&["add", "Second todo"]).assert_success();

    let output = test.todo(&["list"]);
    output
        .assert_success()
        .assert_stdout_contains("First todo")
        .assert_stdout_contains("Second todo");
}

#[test]
fn test_mark_todo_done() {
    let test = TodoTest::new();
    test.todo(&["add", "Learn Rust"]).assert_success();

    let output = test.todo(&["done", "1"]);
    output
        .assert_success()
        .assert_stdout_contains("Marked todo #1 as done");

    let list_output = test.todo(&["list"]);
    list_output.assert_stdout_contains("✅");
}

#[test]
fn test_delete_todo() {
    let test = TodoTest::new();
    test.todo(&["add", "To delete"]).assert_success();

    let output = test.todo(&["delete", "1"]);
    output
        .assert_success()
        .assert_stdout_contains("Deleted todo #1");

    let list_output = test.todo(&["list"]);
    assert!(!list_output.stdout().contains("To delete"));
}

#[test]
fn test_done_nonexistent_todo() {
    let test = TodoTest::new();
    let output = test.todo(&["done", "999"]);
    output.assert_failure();
}

#[test]
fn test_delete_nonexistent_todo() {
    let test = TodoTest::new();
    let output = test.todo(&["delete", "999"]);
    output.assert_failure();
}

#[test]
fn test_list_empty() {
    let test = TodoTest::new();
    let output = test.todo(&["list"]);
    output.assert_stdout_contains("No todos yet");
}

#[test]
fn test_multiple_todos_persistence() {
    let test = TodoTest::new();
    test.todo(&["add", "Task 1"]).assert_success();
    test.todo(&["add", "Task 2"]).assert_success();
    test.todo(&["done", "1"]).assert_success();

    // Verify state is persisted
    let output = test.todo(&["list"]);
    output
        .assert_stdout_contains("Task 1")
        .assert_stdout_contains("Task 2")
        .assert_stdout_contains("✅"); // Task 1 should be marked done
}
