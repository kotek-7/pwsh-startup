use std::process::Command;
use std::time::Instant;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        if args[1].to_lowercase() == "-noprofile" {
            // Measure startup time without loading profile
            println!("Measuring PowerShell startup time without loading profile...");
            let mut total_startup_time = 0;
            for i in 0..10 {
                match measure_no_profile_startup_time() {
                    Ok(startup_time) => {
                        println!("({}) Startup time: {} ms", i + 1, startup_time);
                        total_startup_time += startup_time;
                    }
                    Err(e) => panic!("Error: {}", e),
                }
            }
            println!();
            println!(
                "Average startup time (No Profile): {} ms",
                total_startup_time / 10
            );
        } else {
            // Invalid argument
            println!("Invalid argument: {}", args[1]);
        }
    }
    if args.len() == 1 {
        // Measure startup time with loading profile
        println!("Measuring PowerShell startup time with loading profile...");
        println!("(Run with -noprofile argument to measure startup time without loading profile)");
        println!();
        let mut total_startup_time = 0;
        for i in 0..10 {
            match measure_startup_time() {
                Ok(startup_time) => {
                    println!("({}) Startup time: {} ms", i + 1, startup_time);
                    total_startup_time += startup_time;
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
        println!();
        println!("Average startup time: {} ms", total_startup_time / 10);
    }
}

fn measure_startup_time() -> Result<u128, String> {
    let start_time = Instant::now();
    let output_result = Command::new("pwsh").args(["-c", "exit"]).output();
    let output = match output_result {
        Ok(output) => output,
        Err(e) => return Err(String::from(format!("PowerShell command failed: {}", e))),
    };
    if !output.status.success() {
        return Err(String::from("PowerShell command failed"));
    }
    let startup_time = start_time.elapsed().as_millis();
    Ok(startup_time)
}

fn measure_no_profile_startup_time() -> Result<u128, String> {
    let start_time = Instant::now();
    let output_result = Command::new("pwsh")
        .args(["-noprofile", "-c", "exit"])
        .output();
    let output = match output_result {
        Ok(output) => output,
        Err(e) => return Err(String::from(format!("PowerShell command failed: {}", e))),
    };
    if !output.status.success() {
        return Err(String::from("PowerShell command failed"));
    }
    let startup_time = start_time.elapsed().as_millis();
    Ok(startup_time)
}
