use crate::cwl::clt::Command;
use rand::{distributions::Alphanumeric, Rng};
use sha1::{Digest, Sha1};
use std::{
    cell::RefCell,
    fs::{self, File},
    io::{self, Error, Read, Write},
    path::{Path, MAIN_SEPARATOR_STR},
    process::Command as SystemCommand,
    vec,
};
pub fn get_filename_without_extension(relative_path: &str) -> Option<String> {
    let path = Path::new(relative_path);

    path.file_name().and_then(|name| name.to_str().map(|s| s.split('.').next().unwrap_or(s).to_string()))
}

fn get_basename(filename: &str) -> String {
    let path = Path::new(filename);

    path.file_name().unwrap_or_default().to_string_lossy().into_owned()
}

fn get_extension(filename: &str) -> String {
    let path = Path::new(filename);

    path.extension().unwrap_or_default().to_string_lossy().into_owned()
}

pub fn get_workflows_folder() -> String {
    "workflows/".to_string()
}

pub fn create_and_write_file(filename: &str, contents: &str) -> Result<(), Error> {
    let path = Path::new(filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create_new(&filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn create_and_write_file_forced(filename: &str, contents: &str) -> Result<(), Error> {
    let path = Path::new(filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(filename)?; //here ist the difference
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn copy_file(from: &str, to: &str) -> Result<(), Error> {
    let path = Path::new(to);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?
    }

    fs::copy(from, to)?;
    Ok(())
}

pub fn copy_dir(src: &str, dest: &str) -> Result<Vec<String>, Error> {
    let mut files = vec![];
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = Path::new(dest).join(entry.file_name());
        if src_path.is_dir() {
            files.extend(copy_dir(src_path.to_str().unwrap(), dest_path.to_str().unwrap())?);
        } else {
            copy_file(src_path.to_str().unwrap(), dest_path.to_str().unwrap())?;
            files.push(dest_path.to_string_lossy().into_owned())
        }
    }
    Ok(files)
}

pub fn resolve_path(filename: &str, relative_to: &str) -> String {
    let path = Path::new(filename);
    let relative_path = Path::new(relative_to);
    let base_dir = match relative_path.extension() {
        Some(_) => relative_path.parent().unwrap_or_else(|| Path::new(".")),
        None => relative_path,
    };

    pathdiff::diff_paths(path, base_dir).expect("path diffs not valid").to_string_lossy().into_owned()
}

pub fn get_qualified_filename(command: &Command, the_name: Option<String>) -> String {
    //decide over filename
    let mut filename = match &command {
        Command::Multiple(cmd) => get_filename_without_extension(cmd[1].as_str()).unwrap_or(cmd[1].clone()),
        Command::Single(cmd) => cmd.to_string(),
    };

    if let Some(name) = the_name {
        filename = name.clone();
        if filename.ends_with(".cwl") {
            filename = filename.replace(".cwl", "");
        }
    }

    let foldername = filename.clone();
    filename.push_str(".cwl");

    get_workflows_folder() + &foldername + "/" + &filename
}

pub fn get_file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn get_file_checksum<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn get_shell_command() -> SystemCommand {
    let shell = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
    let param = if cfg!(target_os = "windows") { "/C" } else { "-c" };
    let mut cmd = SystemCommand::new(shell);
    cmd.arg(param);
    cmd
}

pub fn get_file_property(filename: &str, property_name: &str) -> String {
    match property_name {
        "size" => get_file_size(filename).unwrap_or(1).to_string(),
        "basename" => get_basename(filename),
        "nameroot" => get_filename_without_extension(filename).unwrap(),
        "nameext" => get_extension(filename),
        "path" => filename.to_string(),
        "dirname" => {
            let path = Path::new(filename);
            let parent = path.parent().unwrap_or(path).to_string_lossy().into_owned();
            if parent.is_empty() {
                return ".".to_string();
            }
            parent
        }
        _ => fs::read_to_string(filename).unwrap_or_else(|_| panic!("Could not read file {}", filename)),
    }
}

pub fn join_path_string(path: &Path, location: &str) -> String {
    let new_location = path.join(location);
    new_location.to_string_lossy().into_owned()
}

pub fn get_random_filename(prefix: &str, extension: &str) -> String {
    let rnd: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    format!("{}_{}.{}", prefix, rnd, extension)
}

pub fn get_first_file_with_prefix(location: &str, prefix: &str) -> Option<String> {
    let path = Path::new(location);

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();

            if filename_str.starts_with(prefix) {
                return Some(filename_str.into_owned());
            }
        }
    }

    None
}

pub fn make_relative_to<'a>(path: &'a str, dir: &str) -> &'a str {
    let prefix = if !dir.ends_with(MAIN_SEPARATOR_STR) {
        &format!("{}{}", dir, MAIN_SEPARATOR_STR)
    } else {
        dir
    };
    path.strip_prefix(prefix).unwrap_or(path)
}

thread_local!(static PRINT_OUTPUT: RefCell<bool> = const { RefCell::new(true) });

pub fn set_print_output(value: bool) {
    PRINT_OUTPUT.with(|print_output| {
        *print_output.borrow_mut() = value;
    });
}

pub fn print_output() -> bool {
    PRINT_OUTPUT.with(|print_output| *print_output.borrow())
}
