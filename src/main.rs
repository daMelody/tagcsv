use serde_derive::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
struct Post {
    title: String,
    tags: HashSet<String>,
}

fn main() -> Result<(), std::io::Error> {
    // Collect all tags
    let mut all_tags: HashSet<String> = HashSet::new();
    // colect all posts
    let mut posts: Vec<Post> = Vec::new();

    // Read in the files from the "posts" directory
    let dir = std::fs::read_dir("posts")?;
    for entry in dir {
        // Handle errors
        let entry = entry?;
        // Read file contents as String
        let contents = std::fs::read_to_string(entry.path())?;
        // Parse contents with `toml` crate
        let post: Post = toml::from_str(&contents)?;
        // Add all tags to all_tags set
        for tag in &post.tags {
            all_tags.insert(tag.clone());
        }
        // Update posts vector
        posts.push(post);
    }
    // Generage the CSV ouput
    gen_csv(&all_tags, &posts)?;
    Ok(())
}

fn gen_csv(all_tags: &HashSet<String>, posts: &[Post]) -> Result<(), std::io::Error> {
    // Open file for output
    let mut writer = csv::Writer::from_path("tag-matrix.csv")?;

    // Generate the header, with the word "Title" and then all of the tags
    let mut header = vec!["Title"];
    for tag in all_tags.iter() {
        header.push(tag);
    }
    writer.write_record(header)?;

    // Print out a separate row for each post
    for post in posts {
        // Create a record with the post title
        let mut record = vec![post.title.as_str()];
        for tag in all_tags {
            let field = if post.tags.contains(tag) {
                "true"
            } else {
                "false"
            };
            record.push(field);
        }
        writer.write_record(record)?;
    }
    writer.flush()?;
    Ok(())
}
