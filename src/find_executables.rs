use std::{collections::HashMap, os::unix::fs::MetadataExt};

pub(crate) fn find_executables(path: &str) -> HashMap<String, String> {
    let mut executables = HashMap::new();
    // get all the elements in the path
    let dirs_in_path = path
        .split(":")
        .filter(|dir| !dir.is_empty())
        .collect::<Vec<_>>();

    // for each element
    let subdirs_and_execs = dirs_in_path
        .iter()
        .flat_map(|dir| std::fs::read_dir(dir).unwrap())
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.metadata().unwrap().mode() & 0o001 != 0)
        .collect::<Vec<_>>();

    // split into executables and subdirectories
    let execs = subdirs_and_execs
        .iter()
        .filter(|entry| entry.metadata().unwrap().mode() & 0o001 != 0);

    for exec in execs {
        let name = exec.file_name().to_string_lossy().into_owned();
        let exec_path = exec.path().to_string_lossy().into_owned();

        if executables.contains_key(&name) {
            continue;
        }

        executables.insert(name, exec_path);
    }

    executables
}
