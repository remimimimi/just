use executable_path::executable_path;
use std::{path, process, str};

use test_utilities::tmptree;

fn search_test<P: AsRef<path::Path>>(path: P, args: &[&str]) {
    let binary = executable_path("just");

    let output = process::Command::new(binary)
        .current_dir(path)
        .args(args)
        .output()
        .expect("just invocation failed");

    assert_eq!(output.status.code().unwrap(), 0);

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!(stdout, "ok\n");

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert_eq!(stderr, "echo ok\n");
}

#[test]
fn test_justfile_search() {
    let tmp = tmptree! {
      justfile: "default:\n\techo ok",
      a: {
        b: {
          c: {
            d: {},
          },
        },
      },
    };

    search_test(tmp.path().join("a/b/c/d"), &[]);
}

#[test]
fn test_capitalized_justfile_search() {
    let tmp = tmptree! {
      Justfile: "default:\n\techo ok",
      a: {
        b: {
          c: {
            d: {},
          },
        },
      },
    };

    search_test(tmp.path().join("a/b/c/d"), &[]);
}

#[test]
fn test_upwards_path_argument() {
    let tmp = tmptree! {
      justfile: "default:\n\techo ok",
      a: {
        justfile: "default:\n\techo bad",
      },
    };

    search_test(&tmp.path().join("a"), &["../"]);
    search_test(&tmp.path().join("a"), &["../default"]);
}

#[test]
fn test_downwards_path_argument() {
    let tmp = tmptree! {
      justfile: "default:\n\techo bad",
      a: {
        justfile: "default:\n\techo ok",
      },
    };

    let path = tmp.path();

    search_test(&path, &["a/"]);
    search_test(&path, &["a/default"]);
    search_test(&path, &["./a/"]);
    search_test(&path, &["./a/default"]);
    search_test(&path, &["./a/"]);
    search_test(&path, &["./a/default"]);
}

#[test]
fn test_upwards_multiple_path_argument() {
    let tmp = tmptree! {
      justfile: "default:\n\techo ok",
      a: {
        b: {
          justfile: "default:\n\techo bad",
        },
      },
    };

    let path = tmp.path().join("a").join("b");
    search_test(&path, &["../../"]);
    search_test(&path, &["../../default"]);
}

#[test]
fn test_downwards_multiple_path_argument() {
    let tmp = tmptree! {
      justfile: "default:\n\techo bad",
      a: {
        b: {
          justfile: "default:\n\techo ok",
        },
      },
    };

    let path = tmp.path();

    search_test(&path, &["a/b/"]);
    search_test(&path, &["a/b/default"]);
    search_test(&path, &["./a/b/"]);
    search_test(&path, &["./a/b/default"]);
    search_test(&path, &["./a/b/"]);
    search_test(&path, &["./a/b/default"]);
}

#[test]
fn single_downards() {
    let tmp = tmptree! {
      justfile: "default:\n\techo ok",
      child: {},
    };

    let path = tmp.path();

    search_test(&path, &["child/"]);
}

#[test]
fn single_upwards() {
    let tmp = tmptree! {
      justfile: "default:\n\techo ok",
      child: {},
    };

    let path = tmp.path().join("child");

    search_test(&path, &["../"]);
}
