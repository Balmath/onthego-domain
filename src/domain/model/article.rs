use chrono::{DateTime, Utc};
use slugify::slugify;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ArticleId {
    LanguageCategoryTitle(String, String, String),
    LanguageCategorySubCategoryTitle(String, String, String, String),
}

/// Represent an article
#[derive(Debug)]
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
    pub fn from(id: ArticleId) -> Self {
        let id_clone = id.clone();
        let author = String::default();
        let date = Utc::now();
        let content = String::default();
        let tags = vec![];
        match id {
            ArticleId::LanguageCategoryTitle(language, category, title) =>
                Self {
                    id: id_clone, language, category, sub_category: None, title,
                    author, date, content, tags,
                },
            ArticleId::LanguageCategorySubCategoryTitle(
                language, category, sub_category, title) =>
                Self {
                    id: id_clone, language, category,
                    sub_category: Some(sub_category),
                    title, author, date, content, tags,
                },
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

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
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

        let article = Article::from(ArticleId::LanguageCategoryTitle(
            String::from(language),
            String::from(category),
            String::from(title)));

        let article_slug = article.get_slug();

        let expected_id = ArticleId::LanguageCategoryTitle(
            String::from(language),
            String::from(category),
            String::from(&article_slug));

        let expected_path: PathBuf = 
            [language, category, &article_slug].iter().collect();

        assert_eq!(&expected_id, article.get_id());
        assert_eq!(expected_path, article.get_path());
    }

    #[test]
    fn test_article_get_path_with_sub_category() { 
        let language = "en";
        let category = "test";
        let sub_category = "sub";
        let title = "title";

        let article = Article::from(ArticleId::LanguageCategorySubCategoryTitle(
            String::from(language),
            String::from(category),
            String::from(sub_category),
            String::from(title)));

        let article_slug = article.get_slug();

        let expected_id = ArticleId::LanguageCategorySubCategoryTitle(
            String::from(language),
            String::from(category),
            String::from(sub_category),
            String::from(&article_slug));

        let expected_path: PathBuf =
            [language, category, sub_category, &article_slug].iter().collect();

        assert_eq!(&expected_id, article.get_id());
        assert_eq!(expected_path, article.get_path());
    }
}