use crate::repository::tests::utils::*;

/// Test Utils
mod utils {
  use crate::repository::Repository;
  use actix_multipart::form::tempfile::TempFile;
  use std::fs::File;
  use std::io::{Read, Write};
  use std::{fs, path::PathBuf};
  use tempfile::NamedTempFile;
  use uuid::Uuid;

  pub struct TestRepo(Repository);
  type TestApk = (String, PathBuf, File, TempFile);

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

  /// Returns a list of all available test apks
  pub fn get_test_apks() -> Vec<TestApk> {
    vec![
      "com.dede.android_eggs_28",
      "fr.ralala.hexviewer_142",
      "me.hackerchick.catima_128",
      "nodomain.freeyourgadget.gadgetbridge_224",
      "org.woheller69.gpscockpit_240",
    ]
    .iter()
    .map(|name| {
      let file_path = get_repo_path().join(format!("../test-resources/{name}.apk"));
      let file = File::open(&file_path).unwrap();
      let temp_file = temp_file_from_file(&file, name);
      (name.to_string(), file_path, file, temp_file)
    })
    .collect()
  }

  /// Returns a single TestApk for testing
  pub fn get_test_apk() -> TestApk {
    get_test_apks().pop().unwrap()
  }

  /// Creates a temp file from an file
  pub fn temp_file_from_file(file: &File, file_name: &str) -> TempFile {
    let file_contents: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();
    // create temp test file
    let mut temp_test_file = NamedTempFile::new().unwrap();
    temp_test_file.write_all(&file_contents).unwrap();

    TempFile {
      file: temp_test_file,
      content_type: None,
      file_name: Some(file_name.to_owned()),
      size: 0,
    }
  }
}

/// Tests that in a new repo, all apps are empty
#[test]
fn apps_empty() {
  let repo = TestRepo::default();

  assert!(repo.get_repo().get_apps().unwrap().is_empty());
}

/// Test that uploading an app works
#[test]
fn upload_app() {
  let repo = TestRepo::default();

  // get app
  let test_apk = get_test_apk();

  // upload app
  repo.get_repo().upload_app(test_apk.3).unwrap();

  // check that one app has been created
  let apps = repo.get_repo().get_apps().unwrap();
  assert_eq!(apps.len(), 1);
}
