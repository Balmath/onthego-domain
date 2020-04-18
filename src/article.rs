use chrono::{DateTime, Utc};
use slugify::slugify;
use std::path::PathBuf;

/// Represent an article
pub struct Article {
    pub language: String,
    pub category: String,
    pub sub_category: Option<String>,
    pub title: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub content: String,
    pub tags: Vec<String>,
}

impl Article {
    /// Returns a slugify title
    pub fn get_slug(&self) -> String {
        slugify!(&self.title, separator = "_")
    }

    /// Returns path with the language, the category, optionally the sub-category, and the slugify title
    pub fn get_path(&self) -> PathBuf {
        let mut path_elements = vec![&self.language, &self.category];

        if let Some(sub_category) = &self.sub_category {
            path_elements.push(&sub_category);
        }

        let title_slug = self.get_slug();
        path_elements.push(&title_slug);
        
        path_elements.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_get_path() {
        let article = Article {
            language: String::from("en"),
            category: String::from("test"),
            sub_category: None,
            title: String::from("Title"),
            author: String::from("me"),
            date: Utc::now(),
            content: String::from("content"),
            tags: vec![],
        };

        let article_slug = article.get_slug();
        let expected: PathBuf = ["en", "test", &article_slug].iter().collect();

        assert_eq!(expected, article.get_path());
    }

    #[test]
    fn test_article_get_path_with_sub_category() { 
        let article = Article {
            language: String::from("en"),
            category: String::from("test"),
            sub_category: Some(String::from("sub")),
            title: String::from("Title"),
            author: String::from("me"),
            date: Utc::now(),
            content: String::from("content"),
            tags: vec![],
        };

        let article_slug = article.get_slug();
        let expected: PathBuf = ["en", "test", "sub", &article_slug].iter().collect();

        assert_eq!(expected, article.get_path());
    }
}