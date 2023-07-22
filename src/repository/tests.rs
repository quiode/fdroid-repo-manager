use crate::repository::tests::utils::TestRepo;

/// Test Utils
mod utils {
  use crate::repository::Repository;
  use std::{fs, path::PathBuf};
  use uuid::Uuid;

  pub struct TestRepo(Repository);

  impl Drop for TestRepo {
    fn drop(&mut self) {
      // delete directory
      fs::remove_dir_all(&self.0.path).unwrap();
    }
  }

  impl TestRepo {
    pub fn get_repo(&self) -> &Repository {
      &self.0
    }
  }

  impl Default for TestRepo {
    fn default() -> Self {
      // create new test repo in empty, random directory

      // create unique repo path
      let repo_path = get_repo_path().join(Uuid::new_v4().to_string());
      // create repo
      fs::create_dir_all(&repo_path).unwrap();

      Self(Repository::new(repo_path))
    }
  }

  /// Returns the main path for test repos
  fn get_repo_path() -> PathBuf {
    PathBuf::from("developement/tests").canonicalize().unwrap()
  }
}

/// Tests that in a new repo, all apps are empty
#[test]
fn apps_empty() {
  let repo = TestRepo::default();

  assert!(repo.get_repo().get_apps().unwrap().is_empty());
}
