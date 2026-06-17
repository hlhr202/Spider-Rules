/// Coarse-grained classification of a matched spider/crawler token.
///
/// Each variant maps to a stable snake_case string via [`Self::as_str`] for use
/// as structured-log field values or metrics labels. `Display` is intentionally
/// NOT implemented (by design decision) to keep the textual form explicit and
/// stable; use [`Self::as_str`] when a string is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserAgentTokenCategory {
    /// AI / LLM crawlers and answer-search bots.
    /// Representative: GPTBot, OAI-SearchBot, ClaudeBot, PerplexityBot, Bytespider.
    AiSearchAnswerCrawler,
    /// General-purpose search-engine indexers.
    /// Representative: Googlebot, Bingbot, DuckDuckBot, Baiduspider, Yandexbot.
    SearchIndexCrawler,
    /// Ad-platform validators / quality crawlers.
    /// Representative: AdsBot-Google, Mediapartners-Google, YandexDirect.
    AdPlatformValidator,
    /// SEO / marketing research crawlers.
    /// Representative: AhrefsBot, SemrushBot, MJ12bot, DotBot.
    SeoMarketingCrawler,
    /// Social / messaging link-preview fetchers.
    /// Representative: facebookexternalhit, Slackbot, Twitterbot, WhatsApp.
    SocialLinkPreviewFetcher,
    /// Uptime / synthetic monitoring & ping checkers.
    /// Representative: Pingdom, UptimeRobot, Site24x7, StatusCake.
    UptimeMonitor,
}

impl UserAgentTokenCategory {
    /// Returns a stable snake_case string identifying the category.
    ///
    /// Intended for structured-log field values and metrics labels. By design
    /// decision this crate does NOT implement `Display`; callers that need a
    /// string form should use this method explicitly so the textual identifier
    /// remains stable and intentional.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AiSearchAnswerCrawler => "ai_search_answer_crawler",
            Self::SearchIndexCrawler => "search_index_crawler",
            Self::AdPlatformValidator => "ad_platform_validator",
            Self::SeoMarketingCrawler => "seo_marketing_crawler",
            Self::SocialLinkPreviewFetcher => "social_link_preview_fetcher",
            Self::UptimeMonitor => "uptime_monitor",
        }
    }
}
