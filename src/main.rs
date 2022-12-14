use git2::{Branch, BranchType, Error, Repository};

struct RepoState<'repo> {
    main_exists: bool,
    master_exists: bool,
    branch_to_delete: Option<Branch<'repo>>,
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
    delete_branch(branch_to_delete)?;

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
        (false, false) => panic!("Don't know which branch to reset to"),
    };
    let (object, reference) = repo.revparse_ext(refname)?;

    repo.checkout_tree(&object, None)?;
    repo.set_head(
        &reference
            .map(|rference| rference.name().unwrap().to_owned())
            .unwrap(),
    )
}

fn delete_branch(branch: Option<Branch>) -> Result<(), Error> {
    if let Some(mut branch) = branch {
        branch.delete()
    } else {
        Ok(())
    }
}
