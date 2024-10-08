use calamine::{open_workbook, Reader, Xlsx};
use s4n::init::{
    check_git_installation, create_arc_folder_structure, create_investigation_excel_file,
    create_minimal_folder_structure, init_git_repo, init_s4n, is_git_repo,
};
use std::{
    path::PathBuf,
    process::Command,
};
use tempfile::{Builder, NamedTempFile};

#[test]
fn test_is_git_repo() {
    let repo_dir = Builder::new().prefix("valid_git_repo").tempdir().unwrap();

    let repo_dir_str = repo_dir.path().to_str().unwrap();
    let repo_dir_string = String::from(repo_dir_str);

    // Write simple script to init git repository in directory
    let init_script = r#"
            mkdir -p {repo_dir}
            cd {repo_dir}
            git init
            echo "Hello World" > file.txt
            git add .
        "#;

    //execute script to init git repo
    let output = Command::new("bash")
        .arg("-c")
        .arg(init_script.replace("{repo_dir}", &repo_dir_str))
        .status()
        .expect("Failed to execute bash script");

    assert!(output.success(), "Expected success of running command, got {:?}", output);

    // Check if this directory is a git repository
    let result = is_git_repo(Some(&repo_dir_string));

    // Assert that directory is a git repo
    assert!(result, "Expected directory to be a git repo true, got false");
}


#[test]
fn test_check_git_installation_success() {
    // Test case: Git is installed and accessible
    assert!(check_git_installation().is_ok(), "Expected git to be installed and in PATH. Please install git.");
}



#[test]
fn test_is_not_git_repo() {
    //create directory that is not a git repo
    let empty_dir = Builder::new().prefix("empty_repo").tempdir().unwrap();

    let empty_dir_str = empty_dir.path().to_str().unwrap();
    let empty_dir_string = String::from(empty_dir_str);

    // call is_git repo_function
    let result = is_git_repo(Some(&empty_dir_string));

    // assert that it is not a git repo
    assert!(!result, "Expected not to be a git repo");
}

#[test]
fn test_init_git_repo() {
    let temp_dir = tempfile::tempdir().unwrap();
    let base_folder = temp_dir.path().join("my_repo");

    let result = init_git_repo(Some(base_folder.to_str().unwrap()));
    assert!(result.is_ok(), "Expected successful initialization");

    // Verify that the .git directory was created
    let git_dir = base_folder.join(".git");
    assert!(git_dir.exists(), "Expected .git directory to be created");
}

#[test]
fn test_create_minimal_folder_structure_invalid() {
    //create an invalid file input
    let temp_file = NamedTempFile::new().unwrap();
    let base_folder = Some(temp_file.path().to_str().unwrap());

    println!("Base folder path: {:?}", base_folder.as_deref());
    //path to file instead of a directory, assert that it fails
    let result = create_minimal_folder_structure(base_folder.as_deref());
    assert!(!result.is_ok(), "Expected failed initialization");
}

#[test]
fn test_create_minimal_folder_structure() {
    let temp_dir = Builder::new().prefix("minimal_folder").tempdir().unwrap();

    let base_folder = Some(temp_dir.path().to_str().unwrap());

    let result = create_minimal_folder_structure(base_folder.as_deref());

    //test if result is ok
    assert!(result.is_ok(), "Expected successful initialization");

    let expected_dirs = vec!["workflows", "workflows/tools", "workflows/wf"];
    //assert that folders exist
    for dir in &expected_dirs {
        let full_path = PathBuf::from(temp_dir.path()).join(dir);
        assert!(full_path.exists(), "Directory {} does not exist", dir);
    }
}

#[test]
fn test_create_investigation_excel_file() {
    //create directory
    let temp_dir = Builder::new()
        .prefix("investigation_excel_test_")
        .tempdir()
        .unwrap();
    let directory = temp_dir.path().to_str().unwrap();

    //call the function
    assert!(
        create_investigation_excel_file(directory).is_ok(),
        "Unexpected function create_investigation_excel fail"
    );

    //verify file exists
    let excel_path = PathBuf::from(directory).join("isa_investigation.xlsx");
    assert!(excel_path.exists(), "Excel file does not exist");

    let workbook: Xlsx<_> = open_workbook(excel_path).expect("Cannot open file");

    let sheets = workbook.sheet_names().to_owned();

    //verify sheet name
    assert_eq!(
        sheets[0], "isa_investigation",
        "Worksheet name is incorrect"
    );
}

#[test]
fn test_create_arc_folder_structure_invalid(
) {
    //this test only gives create_arc_folder_structure a file instead of a directory
    let temp_file = NamedTempFile::new().unwrap();
    let base_path = Some(temp_file.path().to_str().unwrap());

    let result = create_arc_folder_structure(base_path.as_deref());
    //result should not be okay because of invalid input
    assert!(!result.is_ok(), "Expected failed initialization");

}

#[test]
fn test_create_arc_folder_structure() {
    let temp_dir = Builder::new().prefix("arc_folder_test").tempdir().unwrap();

    let base_folder = Some(temp_dir.path().to_str().unwrap());

    let result = create_arc_folder_structure(base_folder.as_deref());

    assert!(result.is_ok(), "Expected successful initialization");

    let expected_dirs = vec!["assays", "studies", "workflows", "runs"];
    //assert that folders are created
    for dir in &expected_dirs {
        let full_path = PathBuf::from(temp_dir.path()).join(dir);
        assert!(full_path.exists(), "Directory {} does not exist", dir);
    }
}


#[test]
fn test_init_s4n_with_arc() {
    let temp_dir = Builder::new().prefix("init_with_arc_test").tempdir().unwrap();
    let arc = Some(true);
    
    let base_folder = Some(temp_dir.path().to_str().unwrap().to_string());
    
    //call method with temp dir
    let result = init_s4n(base_folder, arc);

    assert!(result.is_ok(), "Expected successful initialization");

    //check if directories were created
    let expected_dirs = vec!["workflows", "workflows/tools", "workflows/wf", "assays", "studies", "runs"];

    for dir in &expected_dirs {
        let full_path = PathBuf::from(temp_dir.path()).join(dir);
        assert!(full_path.exists(), "Directory {} does not exist", dir);
    }
}
#[test]
fn test_init_s4n_minimal() {
    let temp_dir = Builder::new().prefix("init_without_arc_test").tempdir().unwrap();
    let arc = None;
    
    let base_folder = Some(temp_dir.path().to_str().unwrap().to_string());
    
    //call method with temp dir
    let result = init_s4n(base_folder, arc);

    assert!(result.is_ok(), "Expected successful initialization");

    //check if directories were created
    let expected_dirs = vec!["workflows", "workflows/tools", "workflows/wf"];
    //check that other directories are not created
    let unexpected_dirs = vec!["assays", "studies", "runs"];
    
    //assert minimal folders do exist
    for dir in &expected_dirs {
        let full_path = PathBuf::from(temp_dir.path()).join(dir);
        assert!(full_path.exists(), "Directory {} does not exist", dir);
    }
    //assert other arc folders do not exist
    for dir in &unexpected_dirs {
        let full_path = PathBuf::from(temp_dir.path()).join(dir);
        assert!(!full_path.exists(), "Directory {} does exist, but should not exist", dir);
    }
}

