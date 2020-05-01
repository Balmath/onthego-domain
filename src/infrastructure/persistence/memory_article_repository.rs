use crate::{Article, ArticleId, ArticleRepository};
use std::collections::HashMap;

/// In-memory article repository
pub struct MemoryArticleRepository {
    article_ids_to_article: HashMap<ArticleId, Article>,
}

impl MemoryArticleRepository {
    /// Create an empty in-memory article repository
    pub fn new() -> Self {
        Self {
            article_ids_to_article: HashMap::new(),
        }
    }
}

impl ArticleRepository for MemoryArticleRepository {
    fn find_all(&self) -> Box<(dyn Iterator<Item = &Article> + '_)> {
        Box::from(self.article_ids_to_article.values())
    }
    
    fn find_all_mut(&mut self) -> Box<(dyn Iterator<Item = &mut Article> + '_)> {
        Box::from(self.article_ids_to_article.values_mut())
    }

    fn find(&self, id: &ArticleId) -> Option<&Article> {
        self.article_ids_to_article.get(id)
    }

    fn find_mut(&mut self, id: &ArticleId) -> Option<&mut Article> {
        self.article_ids_to_article.get_mut(id)
    }
    
    fn save(&mut self, article: Article) {
        if !self.article_ids_to_article.contains_key(article.get_id()) {
            self.article_ids_to_article.insert(article.get_id().clone(), article);
        }
    }

    fn update(&mut self, _article: &mut Article) {
        // Nothing to do
    }

    fn delete(&mut self, id: &ArticleId) {
        self.article_ids_to_article.remove(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_memory_article_repository() {
        let repository = MemoryArticleRepository::new();

        let mut iter = repository.find_all();

        assert_eq!(None, iter.next());
    }

    #[test]
    fn save_article() {
        let mut repository = MemoryArticleRepository::new();

        let article = Article::from(ArticleId::LanguageCategoryTitle(
            String::from("fr"),
            String::from("Catégorie"),
            String::from("Titre")));

        repository.save(article);

        let mut iter = repository.find_all();

        let expected_article = Article::from(ArticleId::LanguageCategoryTitle(
            String::from("fr"),
            String::from("Catégorie"),
            String::from("Titre")));

        assert_eq!(Some(&expected_article), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn find_article() {
        let mut repository = MemoryArticleRepository::new();

        let article = Article::from(ArticleId::LanguageCategoryTitle(
            String::from("fr"),
            String::from("Catégorie"),
            String::from("Titre")));

        repository.save(article);

        let id = ArticleId::from(ArticleId::LanguageCategoryTitle(
            String::from("fr"),
            String::from("Catégorie"),
            String::from("Titre")));
        
        let article_found = repository.find(&id).unwrap();

        let expected_article = Article::from(ArticleId::LanguageCategoryTitle(
            String::from("fr"),
            String::from("Catégorie"),
            String::from("Titre")));

        assert_eq!(&expected_article, article_found);
    }

    #[test]
    fn delete_article() {
        let mut repository = MemoryArticleRepository::new();

        let article = Article::from(ArticleId::LanguageCategoryTitle(
            String::from("fr"),
            String::from("Catégorie"),
            String::from("Titre")));

        repository.save(article);

        let id = ArticleId::from(ArticleId::LanguageCategoryTitle(
            String::from("fr"),
            String::from("Catégorie"),
            String::from("Titre")));

        repository.delete(&id);

        let mut iter = repository.find_all();

        assert_eq!(None, iter.next());
    }
}
