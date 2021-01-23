use std::process::Command;

pub fn set_source(source: String) -> Result<String, String> {
    let output = Command::new("paswitch").arg(source).output().unwrap();

    if !output.status.success() {
        println!("error");

        return Err("Error".to_owned());
    }

    Ok("Set".to_owned())
}
