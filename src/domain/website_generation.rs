use std::io::{Read, Write};
use std::path::PathBuf;

// Input types

pub enum Language {
    English,
    French,
}

pub struct Category(String);

pub struct SubCategory(String);

pub struct Title(String);

pub struct Author(String);

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Year(u16);

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Day(u8);

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct PublishedDate {
    year: Year,
    month: Month,
    day: Day,
}

pub struct Content(String);

pub struct Tag(String);

pub struct Tags(Vec<Tag>);

pub trait Buffer: Read + Write {}

pub struct ImageName(String);

pub struct Image {
    name: ImageName,
    buffer: Box<dyn Buffer>,
}

pub struct EditedArticle {
    language: Language,
    category: Category,
    sub_category: Option<SubCategory>,
    title: Title,
    author: Author,
    published_date: PublishedDate,
    content: Content,
    tags: Tags,
    images: Vec<Image>,
}

pub struct ArticlesEdited(Vec<EditedArticle>);

// Output types

pub struct HtmlPagePath(PathBuf);

pub struct ArticlePageGenerated(HtmlPagePath);

#[derive(Debug, Eq, PartialEq)]
pub struct HtmlPageIndex(usize);

pub enum GenerateWebsiteEvent {
    ArticlePageGenerated(HtmlPagePath),
    HomeHtmlPageGenerated(HtmlPagePath),
    CategoryHtmlPageGenerated(HtmlPagePath),
    SubCategoryHtmlPageGenerated(HtmlPagePath),
    TagHtmlPageGenerated(HtmlPagePath),
}

// Articles steps

struct HtmlContent(Box<dyn Buffer>);

struct SlugifiedTitle(String);

struct GeneratedArticle {
    language: Language,
    category: Category,
    sub_category: Option<SubCategory>,
    title: SlugifiedTitle,
    tags: Tags,
    htmlContent: HtmlContent,
}

enum Articles {
    Edited(ArticlesEdited),
    Sorted(Vec<EditedArticle>),
    Generated(Vec<GeneratedArticle>),
}

// First step: sort articles by most recent publication date

fn sort_articles_by_most_recent_publication(articles: Articles) -> Articles {
    match articles {
        Articles::Edited(ArticlesEdited(mut articles)) => {
            articles.sort_by(|a, b| a.published_date.partial_cmp(&b.published_date).unwrap());

            Articles::Sorted(articles)
        }
        _ => articles,
    }
}

// Second step: generate article HTML pages

fn generate_articles_from_edited_articles(articles: Vec<EditedArticle>) -> Articles {
    Articles::Generated(vec![])
}

fn generate_articles(articles: Articles) -> Articles {
    match articles {
        Articles::Edited(ArticlesEdited(articles)) | Articles::Sorted(articles) => {
            generate_articles_from_edited_articles(articles)
        }
        _ => articles,
    }
}

// Create article page generated events

fn create_article_page_generated_events(articles: &Articles) -> Vec<GenerateWebsiteEvent> {
    match articles {
        Articles::Generated(_) => vec![],
        _ => panic!("The article page generated events can only be created with generate articles"),
    }
}

// Workflow

pub fn generate_website(articles: ArticlesEdited) -> Vec<GenerateWebsiteEvent> {
    let mut articles = Articles::Edited(articles);

    articles = sort_articles_by_most_recent_publication(articles);

    articles = generate_articles(articles);

    create_article_page_generated_events(&articles)
}

// Simple types implementation

impl Year {
    pub fn new(year: u16) -> Self {
        Self(year)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl Month {
    pub fn to_str(&self, language: &Language) -> &str {
        match language {
            Language::English => match self {
                Month::January => "January",
                Month::February => "February",
                Month::March => "March",
                Month::April => "April",
                Month::May => "May",
                Month::June => "June",
                Month::July => "July",
                Month::August => "August",
                Month::September => "September",
                Month::October => "October",
                Month::November => "November",
                Month::December => "December",
            },
            Language::French => match self {
                Month::January => "Janvier",
                Month::February => "Février",
                Month::March => "Mars",
                Month::April => "Avril",
                Month::May => "Mai",
                Month::June => "Juin",
                Month::July => "Juillet",
                Month::August => "Août",
                Month::September => "Septembre",
                Month::October => "Octobre",
                Month::November => "Novembre",
                Month::December => "Décembre",
            },
        }
    }
}

impl Day {
    pub fn new(day: u8) -> Option<Self> {
        if day > 0 && day < 32 {
            Some(Self(day))
        } else {
            None
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl ArticlesEdited {
    pub fn new(articles: Vec<EditedArticle>) -> Option<Self> {
        if !articles.is_empty() {
            Some(ArticlesEdited(articles))
        } else {
            None
        }
    }

    pub fn value(&self) -> &Vec<EditedArticle> {
        &self.0
    }
}

impl HtmlPageIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

// Complex types implementation

impl PublishedDate {
    pub fn new(year: Year, month: Month, day: Day) -> Option<Self> {
        // TODO: check valid day of month
        Some(Self { year, month, day })
    }

    pub fn year(&self) -> &Year {
        &self.year
    }

    pub fn month(&self) -> &Month {
        &self.month
    }

    pub fn day(&self) -> &Day {
        &self.day
    }

    pub fn to_string(&self, language: &Language) -> String {
        let year = self.year().value();
        let month = self.month().to_str(&language);
        let day = self.day().value();

        match language {
            Language::English => format!("{} {}, {}", month, day, year),
            Language::French => format!("{} {} {}", day, month, year),
        }
    }
}

impl EditedArticle {
    pub fn published_date(&self) -> &PublishedDate {
        &self.published_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_page_index_creation() {
        assert_eq!(HtmlPageIndex(10), HtmlPageIndex::new(10));
        assert_eq!(HtmlPageIndex(0), HtmlPageIndex::new(0));
    }

    #[test]
    fn test_html_page_index_value() {
        assert_eq!(210, HtmlPageIndex::new(210).value());
        assert_eq!(0, HtmlPageIndex::new(0).value());
    }
}
