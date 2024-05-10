use std::error::Error;
use scraper::{Html, Selector};
use reqwest;
use tokio;
use tokio::time::error::Elapsed;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Fetching LinkedIn post...");

    let response = reqwest::get("https://www.linkedin.com/posts/clevy_new-grad-roles-and-internship-opportunities-activity-7190820567392870402-xKqu/")
        .await?
        .text()
        .await?;

    println!("LinkedIn post fetched successfully.");

    // Parse HTML content
    let document = Html::parse_document(&response);

    // Define a selector to target comments
    let comment_selector = Selector::parse(".comments-post-meta").unwrap();

    // Extract comments
    println!("Extracting comments...");
    let comments: Vec<_> = document
        .select(&comment_selector)
        .map(|comment| {
            let content = comment
                .select(&Selector::parse(".comments-post-meta").unwrap())
                .next()
                .map(|content_element| content_element.text().collect::<String>())
                .unwrap_or_else(|| String::from("No content"));
            content
        })
        .collect();

    // Define a regular expression pattern to match emails
    let email_regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();

    // Extract emails from comments
    let mut extracted_emails = Vec::new();
    for comment_content in comments {
        for email in email_regex.find_iter(&comment_content) {
            extracted_emails.push(email.as_str().to_string());
        }
    }

    // Print extracted emails
    println!("Extracted emails:");
    for email in extracted_emails {
        println!("{}", email);
    }

    println!("Program execution completed successfully.");
    Ok(())
}
