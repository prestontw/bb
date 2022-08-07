use git2::{BranchType, Error};

fn main() -> Result<(), Error> {
    let repo = git2::Repository::discover("./")?;
    for branch in repo.branches(Some(BranchType::Local))? {
        let branch = branch?.0;
        println!("{:?}: {}", branch.name(), branch.is_head());
    }

    Ok(())
}
