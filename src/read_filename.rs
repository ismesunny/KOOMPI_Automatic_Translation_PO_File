use std::fs;

pub fn read_fname(f_readname: &mut Vec<String>) {
    let entries = fs::read_dir("./data/import_po"); //read file directory

    match entries {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let metadata_rst = fs::metadata(&path);
                        match metadata_rst {
                            Ok(metadata) => {
                                if metadata.is_file() {
                                    match path.extension() {
                                        Some(ext) => {
                                            if ext == "po" {
                                                match path.file_name() {
                                                    Some(filename) => {
                                                        println!(
                                                            "file name {}",
                                                            String::from(
                                                                filename.to_string_lossy()
                                                            )
                                                        );
                                                        f_readname.push(String::from(
                                                            filename.to_string_lossy(),
                                                        ));
                                                    }
                                                    None => eprintln!("{}", "No filename"),
                                                }
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                            Err(err) => eprintln!("{}", err),
                        }
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
