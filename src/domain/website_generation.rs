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

// Dependencies

type CreateDirectory<'a> = dyn Fn(&std::path::PathBuf) -> bool + 'a;

// Output types

pub struct HtmlPagePath(PathBuf);

pub struct ArticlePageGenerated(HtmlPagePath);

#[derive(Debug, Eq, PartialEq)]
pub struct HtmlPageIndex(usize);

pub enum GenerateWebsiteEvent {
    ArticlePageGenerated(HtmlPagePath),
    CategoryHtmlPageGenerated(HtmlPagePath),
    HomeHtmlPageGenerated(HtmlPagePath),
    SubCategoryHtmlPageGenerated(HtmlPagePath),
    TagHtmlPageGenerated(HtmlPagePath),
}

// Articles steps types

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

// Workflow

struct WebsiteGenerator<'a> {
    articles: Option<Articles>,                // input
    create_directory: &'a CreateDirectory<'a>, // dependency
    events: Option<Vec<GenerateWebsiteEvent>>, // output
}

pub fn generate_website<'a>(
    create_directory: &'a CreateDirectory,
    articles: ArticlesEdited,
) -> Vec<GenerateWebsiteEvent> {
    WebsiteGenerator::new(articles, create_directory)
        .sort_articles_by_most_recent_publication()
        .generate_article_html_pages()
        .generate_sub_category_html_pages()
        .generate_category_html_pages()
        .generate_tag_html_pages()
        .generate_home_html_pages()
        .get_events()
        .unwrap()
}

// Steps functions

fn get_article_directory(article: &EditedArticle) -> std::path::PathBuf {
    panic!("Not implemented")
}

fn generate_article_html_page<'a>(
    create_directory: &'a CreateDirectory,
    article: EditedArticle,
) -> Option<GeneratedArticle> {
    let article_directory = get_article_directory(&article);

    if create_directory(&article_directory) {
        panic!("Not implemented")
    } else {
        None
    }
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

impl<'a> WebsiteGenerator<'a> {
    fn new(articles: ArticlesEdited, create_directory: &'a CreateDirectory) -> Self {
        Self {
            articles: Some(Articles::Edited(articles)),
            create_directory: create_directory,
            events: Some(Vec::default()),
        }
    }

    fn sort_articles_by_most_recent_publication(&mut self) -> &mut Self {
        if let Some(Articles::Edited(ArticlesEdited(mut articles))) = self.articles.take() {
            articles.sort_by(|a, b| a.published_date.partial_cmp(&b.published_date).unwrap());

            self.articles = Some(Articles::Sorted(articles));
        }

        self
    }

    fn generate_article_html_pages(&mut self) -> &mut Self {
        if let Some(articles) = self.articles.take() {
            match articles {
                Articles::Edited(ArticlesEdited(articles)) | Articles::Sorted(articles) => {
                    self.articles = Some(Articles::Generated(
                        articles
                            .into_iter()
                            .map(|article| {
                                generate_article_html_page(&self.create_directory, article).unwrap()
                            })
                            .collect(),
                    ))
                }
                _ => (),
            }
        }

        self
    }

    fn generate_sub_category_html_pages(&mut self) -> &mut Self {
        if let Some(Articles::Generated(articles)) = &self.articles {
            // TODO: generate sub category HTML pages
        }

        self
    }

    fn generate_category_html_pages(&mut self) -> &mut Self {
        if let Some(Articles::Generated(articles)) = &self.articles {
            // TODO: generate category HTML pages
        }

        self
    }

    fn generate_tag_html_pages(&mut self) -> &mut Self {
        if let Some(Articles::Generated(articles)) = &self.articles {
            // TODO: generate tag HTML pages
        }

        self
    }

    fn generate_home_html_pages(&mut self) -> &mut Self {
        if let Some(Articles::Generated(articles)) = &self.articles {
            // TODO: generate home HTML pages
        }

        self
    }

    fn get_events(&mut self) -> Option<Vec<GenerateWebsiteEvent>> {
        self.articles = None;

        self.events.take()
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
    fn test_year_creation() {
        assert_eq!(Year(2020), Year::new(2020));
        assert_eq!(Year(0), Year::new(0));
    }

    #[test]
    fn test_year_value() {
        assert_eq!(2010, Year::new(2010).value());
        assert_eq!(0, Year::new(0).value());
    }

    #[test]
    fn test_day_creation() {
        assert_eq!(Some(Day(1)), Day::new(1));
        assert_eq!(Some(Day(31)), Day::new(31));
        assert_eq!(None, Day::new(0));
        assert_eq!(None, Day::new(32));
    }

    #[test]
    fn test_day_value() {
        assert_eq!(4, Day::new(4).unwrap().value());
        assert_eq!(25, Day::new(25).unwrap().value());
    }
}
