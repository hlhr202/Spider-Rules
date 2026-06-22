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

/// Whether a matched request was triggered by a real human user and whether
/// the fetching tool can interact with the page beyond merely retrieving
/// content.
///
/// This is an orthogonal dimension to [`UserAgentTokenCategory`]: the category
/// describes *what kind* of bot it is, while `FetchOrigin` describes *who is
/// behind* the request and *what the tool can do* — the combination that
/// determines whether a downstream ad-network conversion should be allowed to
/// pass.
///
/// The three-way split mirrors Google's own official classification of its
/// crawlers and fetchers into "common crawlers", "special-case crawlers", and
/// **"user-triggered fetchers"** (the latter ignoring robots.txt because the
/// fetch was requested by a human). OpenAI similarly distinguishes
/// `ChatGPT-User` ("not used for crawling the web in an automatic fashion;
/// because these actions are initiated by a user") from `GPTBot` /
/// `OAI-SearchBot`.
///
/// Each variant maps to a stable snake_case string via [`Self::as_str`].
/// `Display` is intentionally NOT implemented, consistent with
/// [`UserAgentTokenCategory`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchOrigin {
    /// The bot crawls on its own schedule; no real user is behind the request.
    /// Representative: Googlebot, GPTBot, AhrefsBot, Pingdom.
    ///
    /// Cross-verified against Google's "common crawlers" / "special-case
    /// crawlers" categories and OpenAI's `GPTBot` / `OAI-SearchBot`.
    Autonomous,
    /// A real user triggered the request, but the tool only retrieves page
    /// content — it cannot click, fill forms, or submit actions on the page.
    /// Representative: chatgpt-user, claude-user, google-read-aloud.
    ///
    /// Cross-verified against Google's "user-triggered fetchers" category
    /// and OpenAI's `ChatGPT-User` ("not used for crawling the web in an
    /// automatic fashion; because these actions are initiated by a user,
    /// robots.txt rules may not apply").
    UserTriggeredFetch,
    /// A real user triggered the request **and** the tool is an agentic browser
    /// capable of interacting with the page (clicking, filling, submitting).
    /// Representative: googleagent (Project Mariner).
    ///
    /// Inferred from Project Mariner's public description as an agentic
    /// browser; no official UA-level documentation has been published as of
    /// the time of writing.
    UserTriggeredAgentic,
}

impl FetchOrigin {
    /// Returns a stable snake_case string identifying the fetch origin.
    ///
    /// Intended for structured-log field values and metrics labels, consistent
    /// with [`UserAgentTokenCategory::as_str`].
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Autonomous => "autonomous",
            Self::UserTriggeredFetch => "user_triggered_fetch",
            Self::UserTriggeredAgentic => "user_triggered_agentic",
        }
    }

    /// Returns `true` when a real user is behind the request, i.e. the origin
    /// is [`UserTriggeredFetch`][Self::UserTriggeredFetch] or
    /// [`UserTriggeredAgentic`][Self::UserTriggeredAgentic].
    ///
    /// Convenience for ad-network callers that treat any user-initiated visit
    /// as a valid CPC click regardless of the tool's interaction capability.
    pub fn is_user_initiated(self) -> bool {
        matches!(
            self,
            Self::UserTriggeredFetch | Self::UserTriggeredAgentic
        )
    }
}
