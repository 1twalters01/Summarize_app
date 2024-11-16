pub async fn initialise_process() {
    println!("Enter the publishers you would like to scrape below:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_buffer: &str = buffer.trim();
    let mut publishers_str: Vec<&str> = trimmed_buffer.split(",").collect::<Vec<&str>>();
    publishers = publishers_str.into_iter().map(|publisher| publisher.trim().to_string()).collect::<Vec<String>>();
    let mut publishers: Vec<String> = Vec::new();
    println!("publishers: {:?}", publishers);

    if publishers.is_empty() {
        publishers = Vec::from([
            "Penguin Random House".to_string(),
            "HarperCollins".to_string(),
            "Simon & Schuster".to_string(),
            "Palgrave Macmillan".to_string(),
            "Hachette".to_string(),
        ]);
    }

    println!("publishers: {:?}", publishers);

    run_publishers_module(publishers).await.unwrap();
    run_authors_module().await.unwrap();
    run_books_modules().await.unwrap();
    run_genres_modules().await.unwrap();
}

pub async fn continue_process() {
}