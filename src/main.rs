extern crate git2;

use std::env::Args;
use std::env;
use git2::Repository;

// TODO project name.

fn main() {
    let mut args : Args = env::args();
    let commit = args.nth(1).expect("No param supplied");

    let oid : git2::Oid = git2::Oid::from_str(&commit).expect("Wrong Commit ID");
    println!("Will look for this commit {}", commit);

    // check if there is any repo here.
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // iterate over branches
    let branches : git2::Branches = repo.branches(Some(git2::BranchType::Local)).expect("Could not get local branches");
    for branch in branches {
        println!("I'm in one branch");
        let resul = branch.expect("").0;
        let branch_name = resul.name().expect("No branch").unwrap();
        println!("{}",branch_name);
    }

    match repo.find_commit(oid) {
        Ok(_) => println!("commit/{}", oid),
        Err(_) => println!("Wrong"),
    }

    let head = repo.head().expect("Not currently in a branch");
    let shorthand = head.shorthand().unwrap();
    println!("I'm in branch {}", shorthand);


}
