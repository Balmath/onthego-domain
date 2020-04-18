use crate::Article;
use std::path::Path;

/// Trait to implement to provide an article repository
pub trait ArticleRepository {
    /// Returns an iterator over the whole collection of articles
    fn find_all(&self) -> Box<dyn Iterator<Item = &mut Article>>;

    /// Returns an article with a specific path (c.f. `Article::get_path` function)
    fn find<P: AsRef<Path>>(&self, p: P) -> &mut Article;

    /// Stores a modified article
    fn store(&mut self, a: &Article);

    /// Delete a stored article
    fn delete<P: AsRef<Path>>(&mut self, p: P);
}