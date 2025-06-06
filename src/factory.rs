use rrmounter::{
    AsyncSourceResult,
    base::{
        HomeSection, HomeSectionType, LanguageKey, PaginatedResponse, SearchRequest, SourceInstance,
    },
    generate::{Chapter, Series, SeriesEntry, Status},
};

pub fn get_home() -> AsyncSourceResult<Vec<HomeSection>> {
    Box::pin(async {
        let mut sections: Vec<HomeSection> = vec![];

        let popular_section: HomeSection = HomeSection {
            id: "popular_manga".to_string(),
            title: "Popular Manga".to_string(),
            entries: vec![],
            section_type: HomeSectionType::SingleRowLarge,
            contain_more_items: false,
        };

        let latest_section: HomeSection = HomeSection {
            id: "latest_manga".to_string(),
            title: "Latest Manga".to_string(),
            entries: vec![],
            section_type: HomeSectionType::SingleRowLarge,
            contain_more_items: false,
        };

        sections.push(popular_section);
        sections.push(latest_section);

        Ok(sections)
    })
}

pub fn get_search_results(
    params: SearchRequest,
) -> AsyncSourceResult<PaginatedResponse<SeriesEntry>> {
    Box::pin(async move {
        let items: Vec<SeriesEntry> = vec![];

        let current_page: u32 = params.pagination.page;
        let per_page: u32 = params.pagination.per_page;

        let total_items: u32 = 100;
        let total_pages: u32 = (total_items as f32 / per_page as f32).ceil() as u32;

        let has_next_page: bool = current_page < total_pages;

        let paginated_response: PaginatedResponse<SeriesEntry> = PaginatedResponse {
            items,
            current_page,
            total_pages: Some(total_pages),
            has_next_page,
            total_items: Some(total_items),
        };

        Ok(paginated_response)
    })
}

pub fn get_manga_details(series_id: String) -> AsyncSourceResult<Series> {
    Box::pin(async {
        let title: String = String::new();
        let alt_titles: Vec<String> = vec![];
        let description: String = String::new();
        let status: Status = Status::Ongoing;
        let cover_url: String = String::new();
        let chapters: Vec<Chapter> = vec![];
        let author: Vec<String> = vec![];
        let artist: Vec<String> = vec![];
        let tags: Vec<String> = vec![];
        let original_language: LanguageKey = LanguageKey::Japanese;
        let hentai: bool = false;

        let series_entry: Series = Series {
            source_id: "".to_string(),
            series_id,
            title,
            alt_titles,
            description,
            status,
            cover_url,
            author,
            chapters,
            artist,
            tags,
            hentai,
            original_language,
            number_unread: 13,
            number_chapters: 40,
        };

        Ok(series_entry)
    })
}

pub fn get_chapter(
    _series_id: Option<String>,
    _chapter_id: String,
) -> AsyncSourceResult<Vec<String>> {
    Box::pin(async {
        let pages: Vec<String> = vec![];
        Ok(pages)
    })
}

pub fn example_source() -> SourceInstance {
    SourceInstance {
        home_page: get_home,
        search: get_search_results,
        series: get_manga_details,
        chapter: get_chapter,
    }
}
