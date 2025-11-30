use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect(); 

    if args.len() != 2 {
        eprintln!("drop exactly 2 exes on the merger bro");
        return;
    }

    let exe1 = &args[0];
    let exe2 = &args[1];

    let output_path = Path::new("merged.exe");

    let launcher_code = format!(
        r#"
use std::process::Command;

fn main() {{
    let _ = Command::new(r"{0}").spawn();
    let _ = Command::new(r"{1}").spawn();
}}
"#,
        exe1.replace(r"\", r"\\"),
        exe2.replace(r"\", r"\\")
    );

    std::fs::write("temp_launcher.rs", launcher_code).expect("cant write launcher");

    let status = Command::new("rustc")
        .args(&["temp_launcher.rs", "-o"])
        .arg(output_path)
        .status()
        .expect("failed to run rustc");

    if status.success() {
        println!("done! merged exe -> merged.exe");
    } else {
        eprintln!("compilation failed ");
    }

    let _ = std::fs::remove_file("temp_launcher.rs");
}
