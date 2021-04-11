use crate::errors::ApiError;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Error, Write},
};

use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

static SEPARATOR: &str = "-----";

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub metadata: Option<Vec<String>>,
    pub filename: String,
    pub logs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: String,
    pub status: bool,
}

impl Response {
    pub fn new(message: String, status: bool) -> Self {
        Response { message, status }
    }
}

impl Info {
    pub fn check_file(path: &str) -> Result<bool, Error> {
        if !Path::new(path).exists() {
            File::create(path)?;
            return Ok(false);
        }
        return Ok(true);
    }

    pub fn new(filename: String, logs: Vec<String>, metadata: Vec<String>) -> Info {
        Info {
            filename,
            logs: Some(logs),
            metadata: Some(metadata),
        }
    }

    pub fn save(info: Self) -> Result<Response, ApiError> {
        let path: String = format!("./logs/{}.log", &info.filename);
        let flag = Info::check_file(path.as_str())?;

        let file = OpenOptions::new().append(true).open(path)?;
        let mut buffered_file: BufWriter<File> = BufWriter::new(file.try_clone()?);

        let metadata = {
            match info.metadata {
                Some(list) => list,
                None => Vec::new(),
            }
        };

        let logs = {
            match info.logs {
                Some(list) => list,
                None => Vec::new(),
            }
        };

        if !flag {
            for log in metadata {
                buffered_file.write_fmt(format_args!("{}\n", log))?;
            }
            buffered_file.write_fmt(format_args!("{}\n", SEPARATOR))?;
        }

        for log in logs {
            buffered_file.write_fmt(format_args!("{}\n", log))?;
        }

        return Ok(Response::new("ok".to_string(), true));
    }

    pub fn get(filename: String) -> Result<Info, ApiError> {
        let path: String = format!("./logs/{}.log", &filename);

        if !Path::new(&path).exists() {
            return Err(ApiError::new(404, "Not found".to_string()));
        }

        let file = OpenOptions::new().read(true).open(path)?;
        let buffered_file: BufReader<File> = BufReader::new(file);

        let mut metadata = Vec::<String>::new();
        let mut logs = Vec::<String>::new();
        let mut flag = false;

        for line in buffered_file.lines() {
            let data = line?;
            if !flag {
                metadata.push(data.clone());
            } else {
                logs.push(data.clone());
            }

            if data == SEPARATOR.to_string() {
                flag = true;
            }
        }
        metadata.remove(metadata.len() - 1);
        let result = Info::new(filename, logs, metadata);
        return Ok(result);
    }

    pub fn get_filenames() -> Result<Vec<String>, ApiError> {
        let mut files: Vec<String> = Vec::new();

        for entry in WalkDir::new("./logs") {
            let name: String = match entry {
                Ok(directory) => {
                    let aux = directory.file_name().to_str();
                    if let Some(aux_name) = aux {
                        aux_name.to_string()
                    } else {
                        "Error".to_string()
                    }
                }
                Err(_) => "Error".to_string(),
            };
            files.push(name);
        }
        return Ok(files);
    }

    pub fn delete_log(filename: String) -> Result<Response, ApiError> {
        std::fs::remove_file(format!("./logs/{}.log", &filename))?;
        return Ok(Response::new("deleted".to_string(), true));
    }
}
