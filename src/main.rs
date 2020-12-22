#![feature(hash_set_entry)]

use serde_derive::Deserialize;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Deserialize)]
struct RawPost {
    title: String,
    tags: HashSet<String>,
}

struct Post {
    title: String,
    tags: HashSet<Rc<String>>,
}

fn main() -> Result<(), std::io::Error> {
    // Collect all tags
    let mut all_tags: HashSet<Rc<String>> = HashSet::new();
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
        let raw_post: RawPost = toml::from_str(&contents)?;
        let mut post_tags: HashSet<Rc<String>> = HashSet::new();
        // Add all tags to all_tags set
        for tag in raw_post.tags {
            let tag = Rc::new(tag);
            let tag = all_tags.get_or_insert(tag);
            post_tags.insert(tag.clone());
        }
        let post = Post {
            title: raw_post.title,
            tags: post_tags,
        };
        // Update posts vector
        posts.push(post);
    }
    // Generage the CSV ouput
    gen_csv(&all_tags, &posts)?;
    Ok(())
}

fn gen_csv(all_tags: &HashSet<Rc<String>>, posts: &[Post]) -> Result<(), std::io::Error> {
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
