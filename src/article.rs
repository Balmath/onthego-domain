use chrono::{DateTime, Utc};
use slugify::slugify;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct ArticleId(String, String, Option<String>, String);

/// Represent an article
pub struct Article {
    id: ArticleId,
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
    /// Returns a new empty article
    pub fn from_language_category_title(
        language: String, category: String, title: String) -> Self {
        Self {
            id: ArticleId(String::from(&language),
                          String::from(&category),
                          None,
                          String::from(&title)),
            language,
            category,
            sub_category: None,
            title,
            author: String::default(),
            date: Utc::now(),
            content: String::default(),
            tags: vec![],
        }
    }

    /// Returns a new empty article with sub-category
    pub fn from_language_category_sub_category_title(
        language: String, category: String, sub_category:String, title: String) -> Self {
        Self {
            id: ArticleId(String::from(&language),
                          String::from(&category),
                          Some(String::from(&sub_category)),
                          String::from(&title)),
            language,
            category,
            sub_category: Some(sub_category),
            title,
            author: String::default(),
            date: Utc::now(),
            content: String::default(),
            tags: vec![],
        }
    }

    /// Return the id
    pub fn get_id(&self) -> &ArticleId {
        &self.id
    }

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
        let language = "en";
        let category = "test";
        let title = "title";

        let article = Article::from_language_category_title(
            String::from(language),
            String::from(category),
            String::from(title));

        let article_slug = article.get_slug();

        let expected_id = ArticleId(
            String::from(language),
            String::from(category),
            None,
            String::from(&article_slug));

        let expected_path: PathBuf = ["en", "test", &article_slug].iter().collect();

        assert_eq!(&expected_id, article.get_id());
        assert_eq!(expected_path, article.get_path());
    }

    #[test]
    fn test_article_get_path_with_sub_category() { 
        let language = "en";
        let category = "test";
        let sub_category = "sub";
        let title = "title";

        let article = Article::from_language_category_sub_category_title(
            String::from(language),
            String::from(category),
            String::from(sub_category),
            String::from(title));

        let article_slug = article.get_slug();

        let expected_id = ArticleId(
            String::from(language),
            String::from(category),
            Some(String::from(sub_category)),
            String::from(&article_slug));

        let expected_path: PathBuf =
            ["en", "test", "sub", &article_slug].iter().collect();

        assert_eq!(&expected_id, article.get_id());
        assert_eq!(expected_path, article.get_path());
    }
}