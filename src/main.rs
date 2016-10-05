extern crate git2;

use std::env::Args;
use std::env;
use git2::Repository;

fn main() {
    let mut args : Args = env::args();
    let commit = args.nth(1).expect("No param supplied");

    println!("{}", commit);

    // check if there is any repo here.
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // iterate over branches
    let branches : git2::Branches = repo.branches(Some(git2::BranchType::Local)).expect("Could not get local branches");

    for branch in branches {
        let (resul,  _) = branch.expect("");

    }
}
