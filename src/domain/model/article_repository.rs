use super::{Article, ArticleId};

/// Trait to implement to provide an article repository
pub trait ArticleRepository {
    /// Returns an iterator over the whole collection of articles
    fn find_all(&self) -> Box<(dyn Iterator<Item = &Article> + '_)>;

    /// Returns an iterator over the whole collection of mutable articles
    fn find_all_mut(&mut self) -> Box<(dyn Iterator<Item = &mut Article> + '_)>;

    /// Returns an article with an article id
    fn find(&self, id: &ArticleId) -> Option<&Article>;

    /// Returns a mutable article with an article id
    fn find_mut(&mut self, id: &ArticleId) -> Option<&mut Article>;

    /// Save a new article
    fn save(&mut self, article: Article);

    /// Update an existing article
    fn update(&mut self, article: &mut Article);

    /// Delete a stored article
    fn delete(&mut self, id: &ArticleId);
}
