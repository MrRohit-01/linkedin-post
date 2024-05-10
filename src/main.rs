use std::error::Error;
use scraper::{Html, Selector};
use reqwest;
use tokio;
use tokio::time::error::Elapsed;


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
    let comment_selector = Selector::parse(".app-aware-link").unwrap();

    // Increase timeout for waiting for comments to load
    let timeout = std::time::Duration::from_secs(120);

    // Wait for comments section to load
    println!("Waiting for comments section to load...");
    let result = tokio::time::timeout(timeout, async {
        loop {
            if document.select(&comment_selector).next().is_some() {
                println!("Comments section loaded.");
                break;
            }
            println!("Comments section not loaded yet. Retrying...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    })
    .await.map_err(|e| Box::new(e) as Box<dyn Error>); // Convert Elapsed error to Box<dyn Error>

    if let Err(e) = result {
        println!("Error waiting for comments section to load: {:?}", e);
        return Err(e);
    }

    // Extract comments
    println!("Extracting comments...");
    let comments: Vec<_> = document
        .select(&comment_selector)
        .map(|comment| {
            let author = comment
                .select(&Selector::parse(".comments-post-meta__profile-link").unwrap())
                .next()
                .map(|author_element| author_element.text().collect::<String>())
                .unwrap_or_else(|| String::from("Unknown author"));
            let content = comment
                .select(&Selector::parse(".comments-comment-item-content").unwrap())
                .next()
                .map(|content_element| content_element.text().collect::<String>())
                .unwrap_or_else(|| String::from("No content"));
            (author, content)
        })
        .collect();

    // Print comments
    println!("Printing comments:");
    for (index, (author, content)) in comments.iter().enumerate() {
        println!("Comment {}: Author: {}, Content: {}", index + 1, author, content);
    }

    println!("Program execution completed successfully.");
    Ok(())
}
