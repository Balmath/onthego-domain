use chrono::{DateTime, Utc};
use std::io::{Read, Write};
use std::path::PathBuf;

pub enum Language {
    English,
    French,
}

pub struct Category {
    name: String,
}

pub struct SubCategory {
    name: String,
}

pub struct Title {
    name: String,
}

pub struct Author {
    name: String,
}

pub struct PublishedDate {
    date: DateTime<Utc>,
}

pub struct Content {
    content: String,
}

pub struct Tag {
    name: String,
}

pub trait Buffer: Read + Write {}

pub struct Image {
    content: Box<dyn Buffer>,
}

pub struct EditedArticle {
    language: Language,
    category: Category,
    sub_category: Option<SubCategory>,
    title: Title,
    author: Author,
    published_date: PublishedDate,
    content: Content,
    tags: Vec<Tag>,
    images: Vec<Image>,
}

pub struct ArticlesEdited {
    edited_articles: Vec<EditedArticle>,
}

pub struct HtmlPage {
    content: Box<dyn Write>,
}

pub struct Directory {
    path: PathBuf,
}

pub struct ArticlePackageGenerated {
    directory: Directory,
    htmlPage: HtmlPage,
}

#[derive(Debug, Eq, PartialEq)]
pub struct HtmlPageIndex {
    value: usize,
}

pub struct HomeHtmlPageGenerated {
    directory: Directory,
    index: HtmlPageIndex,
    htmlPage: HtmlPage,
}

pub struct CategoryHtmlPageGenerated {
    directory: Directory,
    index: HtmlPageIndex,
    htmlPage: HtmlPage,
}

pub struct SubCategoryHtmlPageGenerated {
    directory: Directory,
    index: HtmlPageIndex,
    htmlPage: HtmlPage,
}

pub struct TagHtmlPageGenerated {
    directory: Directory,
    index: HtmlPageIndex,
    htmlPage: HtmlPage,
}

pub enum GenerateWebsiteEvent {
    ArticlePackageGenerated(ArticlePackageGenerated),
    HomeHtmlPageGenerated(HomeHtmlPageGenerated),
    CategoryHtmlPageGenerated(CategoryHtmlPageGenerated),
    SubCategoryHtmlPageGenerated(SubCategoryHtmlPageGenerated),
    TagHtmlPageGenerated(TagHtmlPageGenerated),
}

pub fn generate_website(articlesEdited: ArticlesEdited) -> Vec<GenerateWebsiteEvent> {
    vec![]
}

impl HtmlPageIndex {
    pub fn new(index: usize) -> Self {
        Self { value: index }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_page_index_creation() {
        assert_eq!(HtmlPageIndex { value: 10 }, HtmlPageIndex::new(10));
        assert_eq!(HtmlPageIndex { value: 0 }, HtmlPageIndex::new(0));
    }

    #[test]
    fn test_html_page_index_value() {
        assert_eq!(210, HtmlPageIndex::new(210).value());
        assert_eq!(0, HtmlPageIndex::new(0).value());
    }
}
