use std::fs;
use std::io::{BufReader, Read};
use std::process::Command;

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();
    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;
    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;
    Ok(file_content)
}

pub fn systemctl_research(service: String) -> String {
    println!("{}", service);
    let output = match Command::new("systemctl")
        .arg("status")
        .arg(service)
        .output()
    {
        Ok(output) => output,
        Err(e) => panic!("failed to execute process: {}", e),
    };
    if output.status.success() {
        return String::from_utf8_lossy(&output.stdout).to_string();
    }
    return String::from_utf8_lossy(&output.stderr).to_string();
}

pub fn read_toml<T>(path: String) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let file_content = read_file(path)?;
    let toml: T = toml::from_str(&file_content).map_err(|e| e.to_string())?;
    Ok(toml)
}
