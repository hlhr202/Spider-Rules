use crate::token_cat::UserAgentTokenCategory;

/// The result of matching a User-Agent against the static rule table.
///
/// * `token` — the lower-case rule substring that matched the UA, exposed so
///   callers can record "why" it matched in logs / metrics.
/// * `category` — the coarse-grained bot classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identification {
    pub token: &'static str,
    pub category: UserAgentTokenCategory,
}

pub(crate) const STATIC_USER_AGENT_RULES: &[Identification] = &[
    Identification {
        token: "gptbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "oai-searchbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "oai-adsbot",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "chatgpt-user",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "claudebot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "claude-user",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "claude-searchbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "perplexitybot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "perplexity-user",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        // Amazon's user-triggered fetcher (e.g. Alexa answering a live user
        // query). Classified as AI answer-search, consistent with the other
        // `*-user` bots; NOT used for generative-AI training per Amazon's docs.
        token: "amzn-user",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "meta-externalagent",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "meta-webindexer",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "meta-externalfetcher",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "google-cloudvertexbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        // Agentic browser from Google (Project Mariner). Real UA token is
        // `GoogleAgent-Mariner` (lower-cased `googleagent-mariner`, NO hyphen
        // between "google" and "agent"), so the token must be `googleagent`.
        token: "googleagent",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "google-notebooklm",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "bytespider",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "anthropic-ai",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "claude-web",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "duckassistbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "cohere-ai",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "ai2bot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "diffbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "omgili",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "webzio",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "timpibot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "youbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "mistralai-user",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "gemini-deep-research",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "grokbot",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "xai-grok",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "grok-deepsearch",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "googlebot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "googlebot-image",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "googlebot-video",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "google-inspectiontool",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "googleother",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "storebot-google",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "feedfetcher-google",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "google-read-aloud",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "google-site-verification",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "bingbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "bingpreview",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "adidxbot",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "microsoftpreview",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "bingvideopreview",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "duckduckbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "amazonbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        // Amazon's search-experience crawler (Alexa search). Explicitly NOT used
        // for generative-AI training per Amazon's docs.
        token: "amzn-searchbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "applebot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        // Apple's AI-training data-usage token. Its UA contains `applebot`,
        // but `applebot-extended` is the longer/more-specific token so it wins
        // via the longest-token rule and is classified as an AI crawler rather
        // than a plain search indexer. (Applebot-Extended does not crawl pages
        // itself; it controls usage of Applebot-crawled data.)
        token: "applebot-extended",
        category: UserAgentTokenCategory::AiSearchAnswerCrawler,
    },
    Identification {
        token: "ccbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "petalbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "baiduspider",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        // Official UA substring (contains space).
        token: "sogou web spider",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        // Official UA substring (contains space).
        token: "sogou pic spider",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "sogou spider",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "360spider",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "haosouspider",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yisouspider",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandeximages",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexvideo",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexmedia",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexmobilebot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexrenderresourcesbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexaccessibilitybot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexadditionalbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexadditional",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexcombot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexwebmaster",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexfavicons",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexpagechecker",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexsitelinks",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexscreenshotbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexverticals",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexvertis",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexontodb",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexontodbapi",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexblogs",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexspravbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexrca",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexcheckbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexuserproxy",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexcalendar",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandexsearchshop",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yandextracker",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "naverbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        // Naver's crawler also advertises the `Yeti` product token. Short (4)
        // token: keep an eye on substring collisions if a human UA ever embeds
        // the word "yeti".
        token: "yeti",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "seznambot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "mojeekbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "qwantbot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "yodaobot",
        category: UserAgentTokenCategory::SearchIndexCrawler,
    },
    Identification {
        token: "adsbot-google",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "adsbot-google-mobile",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "mediapartners-google",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "google-safety",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "meta-externalads",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "yandexadnet",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "yandexdirect",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "yandexdirectdyn",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "yadirectfetcher",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "yandexmetrika",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "yandexpartner",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "yandexmarket",
        category: UserAgentTokenCategory::AdPlatformValidator,
    },
    Identification {
        token: "ahrefsbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "ahrefssiteaudit",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "semrushbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "siteauditbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "semrushbot-ba",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "semrushbot-si",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "semrushbot-swa",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "splitsignalbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "semrushbot-ocob",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "semrushbot-ft",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "semrushbot-esi",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "rytebot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "mj12bot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "dotbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "serpstatbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "dataforseobot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        // Official UA substring (contains space).
        token: "screaming frog seo spider",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "blexbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "botifybot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "bomborabot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "piplbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "linkdexbot",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "seokicks",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "backlinkcrawler",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "sistrix",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "macocu",
        category: UserAgentTokenCategory::SeoMarketingCrawler,
    },
    Identification {
        token: "facebookexternalhit",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "facebot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "facebookbot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "slackbot-linkexpanding",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "slack-imgproxy",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "slackbot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "pinterestbot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "twitterbot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "discordbot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "linkedinbot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "googlemessages",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "telegrambot",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        // WhatsApp's link-preview crawler UA is a bare product string of the
        // form `WhatsApp/2.<ver>` (e.g. `WhatsApp/2.23.20.0`). The token is
        // intentionally tightened to `whatsapp/2.` (version slash) so that a
        // human in-app-browser UA which merely embeds the word "WhatsApp"
        // without the version does NOT match. Residual limitation: substring
        // matching cannot distinguish the crawler from an in-app browser that
        // embeds the identical `WhatsApp/2.<ver>` token — full elimination
        // requires rDNS / IP verification.
        token: "whatsapp/2.",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "skypeuripreview",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "iframely",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "embedly",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "vkshare",
        category: UserAgentTokenCategory::SocialLinkPreviewFetcher,
    },
    Identification {
        token: "pingdom",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
    Identification {
        token: "uptimerobot",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
    Identification {
        token: "site24x7",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
    Identification {
        token: "newrelicpinger",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
    Identification {
        token: "statuscake",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
    Identification {
        token: "hetrixtools",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
    Identification {
        token: "better-uptime",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
    Identification {
        token: "checkly",
        category: UserAgentTokenCategory::UptimeMonitor,
    },
];

/// Identifies a spider/crawler from the given `User-Agent` string.
///
/// Matching is case-insensitive (the UA is lower-cased first). Among all rules
/// whose token is a substring of the UA, the one with the **longest** token
/// wins; equal-length ties are broken by **declaration order** (first declared
/// wins). Returns `None` when nothing matches.
pub fn identify(user_agent: &str) -> Option<Identification> {
    let lowered = user_agent.to_ascii_lowercase();
    pick_best(STATIC_USER_AGENT_RULES, &lowered)
}

/// Pick the best matching rule from `rules` against an already-lower-cased UA.
///
/// Selection rule: the matching rule with the longest token wins; ties are
/// broken by declaration order (first declared wins).
pub(crate) fn pick_best(rules: &[Identification], lowered: &str) -> Option<Identification> {
    let mut best = None;
    for rule in rules {
        if lowered.contains(rule.token) {
            match best {
                None => best = Some(*rule),
                Some(b) if rule.token.len() > b.token.len() => best = Some(*rule),
                _ => {} // equal length or shorter: keep the earlier (first-declared)
            }
        }
    }
    best
}
