use inquire::{CustomType, Confirm};
use url::Url;

pub struct IagonCreationConfig {
    pub iagon_url: Url,
}

pub fn create_iagon_interactively() -> IagonCreationConfig {
    println!("=== Create Translation with Iagon Storage ===\n");
    
    println!("This command creates a new TranslationV1 that references books data stored on Iagon.");
    println!("You'll need a URL pointing to a books.json file that was uploaded to Iagon.\n");
    
    // Get the Iagon URL
    let url_input = CustomType::<String>::new("Enter the Iagon URL for your books.json file:")
        .with_help_message(
            "This should be a direct link to a books.json file on Iagon\n\
             Example: https://iagon.network/api/v1/file/your-file-id\n\
             You can get this URL after uploading your books.json file to Iagon"
        )
        .with_parser(&|input| {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Err(());
            }
            
            Url::parse(trimmed)
                .map(|url| url.to_string())
                .map_err(|_| ())
        })
        .with_error_message("Invalid URL. Please enter a valid URL (e.g., https://iagon.network/api/v1/file/your-id)")
        .prompt()
        .expect("Failed to get Iagon URL");
    
    let iagon_url = Url::parse(&url_input).unwrap();
    
    // Validate URL scheme
    if !matches!(iagon_url.scheme(), "http" | "https") {
        eprintln!("Error: URL must use http or https protocol");
        std::process::exit(1);
    }
    
    println!("\n✓ URL validated: {}", iagon_url);
    
    // Ask if they want to test the URL
    let test_url = Confirm::new("Would you like to test if the URL is accessible?")
        .with_default(true)
        .with_help_message("This will attempt to fetch the URL to verify it's accessible")
        .prompt()
        .unwrap_or(false);
    
    if test_url {
        println!("Testing URL accessibility...");
        match test_url_accessibility(&iagon_url) {
            Ok(_) => println!("✓ URL is accessible"),
            Err(e) => {
                eprintln!("⚠ Warning: Could not access URL: {}", e);
                let continue_anyway = Confirm::new("Continue anyway?")
                    .with_default(false)
                    .prompt()
                    .unwrap_or(false);
                
                if !continue_anyway {
                    println!("Cancelled.");
                    std::process::exit(0);
                }
            }
        }
    }
    
    println!("\n✓ Ready to create translation metadata");
    
    IagonCreationConfig {
        iagon_url,
    }
}

fn test_url_accessibility(url: &Url) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    
    let output = Command::new("curl")
        .arg("-s")
        .arg("-I") // HEAD request only
        .arg("-L") // follow redirects
        .arg("-f") // fail on HTTP errors
        .arg("--max-time")
        .arg("10") // 10 second timeout
        .arg(url.as_str())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    
    if !output.status.success() {
        return Err(format!(
            "HTTP request failed with status: {}", 
            output.status.code().unwrap_or(0)
        ).into());
    }
    
    Ok(())
}