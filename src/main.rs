use git2::{Branch, BranchType, Error, Repository};

struct RepoState {
    main_exists: bool,
    master_exists: bool,
    branch_to_delete: Option<Branch>,
}

fn main() -> Result<(), Error> {
    // add a trial run variable
    let repo = git2::Repository::discover("./")?;
    let RepoState {
        main_exists,
        master_exists,
        branch_to_delete,
    } = current_branch(&repo)?;
    checkout_main(&repo, main_exists, master_exists)?;
    delete_branch(&repo, branch_to_delete)?;

    Ok(())
}

fn current_branch(repo: &Repository) -> Result<RepoState, Error> {
    let mut state = RepoState {
        main_exists: false,
        master_exists: false,
        branch_to_delete: None,
    };
    for branch in repo.branches(Some(BranchType::Local))? {
        let branch = branch?.0;
        if branch.name().ok().flatten() == Some("main") {
            state.main_exists = true
        } else if branch.name().ok().flatten() == Some("master") {
            state.master_exists = true
        } else {
            if branch.is_head() {
                state.branch_to_delete = Some(branch)
            }
        }
    }
    return Ok(state);
}

fn checkout_main(repo: &Repository, main_exists: bool, master_exists: bool) -> Result<(), Error> {
    let refname = match (main_exists, master_exists) {
        (true, _) => "main",
        (false, true) => "master",
        (false, false) => return Err("jibbly"),
    };
    repo.set_head(refname)
}

fn delete_branch(repo: &Repository, mut branch: Option<Branch>) -> Result<(), Error> {
    branch.delete()
}
