//! Shared spider/crawler identification rules for internal services.
//!
//! The crate exposes a single entry point [`identify`], which takes a
//! `user_agent: &str` and returns the matched rule, if any, as an
//! [`Identification`] containing the hit `token` and its [`UserAgentTokenCategory`].
//!
//! # Input contract
//!
//! Callers only need to pass the raw `User-Agent` string; case is normalized
//! internally via `to_ascii_lowercase()`.
//!
//! # Output semantics
//!
//! `Identification::token` is the lower-case rule substring that matched the
//! UA, useful for structured logging / metrics. `Identification::category`
//! classifies the bot for downstream routing decisions.
//!
//! Selection rule: among all matching rules the one with the longest token
//! wins; ties are broken by declaration order (first declared wins).
//!
//! # Categories
//!
//! | Variant | Representative bots |
//! |---|---|
//! | [`AiSearchAnswerCrawler`][UserAgentTokenCategory::AiSearchAnswerCrawler] | GPTBot, ClaudeBot, anthropic-ai, PerplexityBot, Bytespider |
//! | [`SearchIndexCrawler`][UserAgentTokenCategory::SearchIndexCrawler] | Googlebot, Bingbot, DuckDuckBot, Baiduspider |
//! | [`AdPlatformValidator`][UserAgentTokenCategory::AdPlatformValidator] | AdsBot-Google, Mediapartners-Google, YandexDirect |
//! | [`SeoMarketingCrawler`][UserAgentTokenCategory::SeoMarketingCrawler] | AhrefsBot, SemrushBot, MJ12bot, DotBot |
//! | [`SocialLinkPreviewFetcher`][UserAgentTokenCategory::SocialLinkPreviewFetcher] | facebookexternalhit, Slackbot, Twitterbot, WhatsApp |
//! | [`UptimeMonitor`][UserAgentTokenCategory::UptimeMonitor] | Pingdom, UptimeRobot, Site24x7, StatusCake |

mod rules;
mod token_cat;

pub use rules::{Identification, identify};
pub use token_cat::UserAgentTokenCategory;

#[cfg(test)]
mod tests {
    use super::*;
    use UserAgentTokenCategory::*;

    #[test]
    fn contract_identify_googlebot() {
        let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let id = identify(ua).expect("Googlebot UA should match");
        assert_eq!(id.token, "googlebot");
        assert_eq!(id.category, SearchIndexCrawler);
    }

    #[test]
    fn size_case_insensitive() {
        for ua in [
            "Mozilla/5.0 (compatible; Googlebot/2.1)",
            "Mozilla/5.0 (compatible; GOOGLEBOT/2.1)",
            "Mozilla/5.0 (compatible; googlebot/2.1)",
            "Mozilla/5.0 (compatible; GoOgLeBoT/2.1)",
        ] {
            let id = identify(ua).expect("should match regardless of case");
            assert_eq!(id.token, "googlebot", "token must be lowercase; ua={ua:?}");
            assert_eq!(id.category, SearchIndexCrawler);
        }
    }

    #[test]
    fn longest_wins_over_shorter_substring() {
        // UA contains both `slackbot` (8) and `slackbot-linkexpanding` (21).
        let ua = "Slackbot-LinkExpanding 1.0 (+https://api.slack.com/robots; slackbot)";
        let id = identify(ua).expect("should match");
        assert_eq!(id.token, "slackbot-linkexpanding");
        assert_eq!(id.category, SocialLinkPreviewFetcher);
    }

    #[test]
    fn tie_break_first_declared_wins_on_equal_length() {
        // Two artificial rules with equal-length tokens; both substrings present.
        // Declaration order must decide the winner (first declared).
        let rules: &[Identification] = &[
            Identification {
                token: "aaaabot",
                category: UserAgentTokenCategory::AiSearchAnswerCrawler,
            },
            Identification {
                token: "bbbbbot",
                category: UserAgentTokenCategory::SeoMarketingCrawler,
            },
        ];
        let lowered = "mozilla/5.0 aaaabot/1.0 bbbbbbot/2.0";
        let best = super::rules::pick_best(rules, lowered).expect("should match one");
        assert_eq!(
            best.token, "aaaabot",
            "equal length: first declared must win"
        );
        assert_eq!(best.category, UserAgentTokenCategory::AiSearchAnswerCrawler);
    }

    #[test]
    fn category_coverage_positive_per_category() {
        let cases: &[(&str, &str, UserAgentTokenCategory)] = &[
            (
                "GPTBot/1.0",
                "gptbot",
                UserAgentTokenCategory::AiSearchAnswerCrawler,
            ),
            (
                "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
                "googlebot",
                UserAgentTokenCategory::SearchIndexCrawler,
            ),
            (
                "Mozilla/5.0 (compatible; AdsBot-Google/1.0; +http://www.google.com/adsbot.html)",
                "adsbot-google",
                UserAgentTokenCategory::AdPlatformValidator,
            ),
            (
                "Mozilla/5.0 (compatible; AhrefsBot/7.0; +http://ahrefs.com/robot/)",
                "ahrefsbot",
                UserAgentTokenCategory::SeoMarketingCrawler,
            ),
            (
                "facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)",
                "facebookexternalhit",
                UserAgentTokenCategory::SocialLinkPreviewFetcher,
            ),
        ];
        for (ua, expected_token, expected_cat) in cases {
            let id = identify(ua).unwrap_or_else(|| panic!("should match: {ua:?}"));
            assert_eq!(id.token, *expected_token, "token mismatch; ua={ua:?}");
            assert_eq!(id.category, *expected_cat, "category mismatch; ua={ua:?}");
        }
    }

    #[test]
    fn negatives_empty_and_whitespace_and_browsers() {
        let negatives = [
            "",
            "   ",
            "\t\n",
            // Chrome
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36",
            // Safari
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 Version/17.0 Mobile Safari/604.1",
            // Firefox
            "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
        ];
        for ua in negatives {
            assert!(identify(ua).is_none(), "expected None for ua={ua:?}");
        }
    }

    #[test]
    fn category_negatives_lookalike_no_match() {
        // Tokens that look related but are NOT in the rule table (no bare
        // `google`/`bing`/`bot`/`gpt`/`crawl` tokens exist, verified in rules.rs).
        let lookalikes = [
            "Mozilla/5.0 (just a generic google reference)",
            "Mozilla/5.0 some-browser bing",
            "Mozilla/5.0 some-app bot",
            "MyGPT Application/1.0",
            "Mozilla/5.0 crawly-the-friendly-crawler",
        ];
        for ua in lookalikes {
            assert!(
                identify(ua).is_none(),
                "expected None (no real token present); ua={ua:?}"
            );
        }
    }

    #[test]
    fn whatsapp_crawler_matches_but_bare_word_does_not() {
        // Real WhatsApp link-preview crawler UA — bare `WhatsApp/2.<ver>`.
        let crawler = "WhatsApp/2.23.20.0";
        let id = identify(crawler).expect("WhatsApp crawler should match");
        assert_eq!(id.token, "whatsapp/2.");
        assert_eq!(id.category, SocialLinkPreviewFetcher);

        // A human in-app-browser style UA that merely embeds the word
        // "WhatsApp" WITHOUT the version slash must NOT match. This is the
        // false-positive class the v0.2.0 tightening is designed to prevent.
        let human_like = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) \
            AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 \
            Safari/604.1 WhatsApp";
        assert!(
            identify(human_like).is_none(),
            "bare 'WhatsApp' word must not match after tightening"
        );
    }

    #[test]
    fn googleagent_mariner_matches() {
        // Real token is `GoogleAgent-Mariner` (no hyphen between google/agent).
        let ua = "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; \
            GoogleAgent-Mariner/1.0; +https://google.com)";
        let id = identify(ua).expect("GoogleAgent-Mariner should match");
        assert_eq!(id.token, "googleagent");
        assert_eq!(id.category, AiSearchAnswerCrawler);
    }

    #[test]
    fn new_ai_crawlers_match() {
        let cases: &[(&str, &str)] = &[
            ("Mozilla/5.0 (compatible; anthropic-ai/1.0; +http://www.anthropic.com/bot.html)", "anthropic-ai"),
            ("DuckAssistBot/1.2; (+http://duckduckgo.com/duckassistbot.html)", "duckassistbot"),
            ("Mozilla/5.0 (compatible; cohere-ai/1.0; +http://www.cohere.ai/bot.html)", "cohere-ai"),
            ("Mozilla/5.0 (compatible; Diffbot/0.1; +http://www.diffbot.com)", "diffbot"),
        ];
        for (ua, expected_token) in cases {
            let id = identify(ua).unwrap_or_else(|| panic!("should match: {ua:?}"));
            assert_eq!(id.token, *expected_token, "ua={ua:?}");
            assert_eq!(id.category, AiSearchAnswerCrawler, "ua={ua:?}");
        }
    }

    #[test]
    fn applebot_extended_wins_over_applebot_and_is_ai() {
        // UA contains both `applebot` (SearchIndexCrawler) and the longer
        // `applebot-extended` token. Longest-token rule must pick the latter
        // and classify it as an AI crawler.
        let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 \
            (KHTML, like Gecko) Version/17.4 Safari/605.1.15 \
            (Applebot/0.1; +http://www.apple.com/go/applebot) (Applebot-Extended)";
        let id = identify(ua).expect("should match");
        assert_eq!(id.token, "applebot-extended");
        assert_eq!(id.category, AiSearchAnswerCrawler);
    }

    #[test]
    fn amazon_search_and_user_bots() {
        let search = "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; \
            Amzn-SearchBot/0.1) Chrome/119.0.6045.214 Safari/537.36";
        let id = identify(search).expect("Amzn-SearchBot should match");
        assert_eq!(id.token, "amzn-searchbot");
        assert_eq!(id.category, SearchIndexCrawler);

        let user = "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; \
            Amzn-User/0.1) Chrome/119.0.6045.214 Safari/537.36";
        let id = identify(user).expect("Amzn-User should match");
        assert_eq!(id.token, "amzn-user");
        assert_eq!(id.category, AiSearchAnswerCrawler);
    }

    #[test]
    fn removed_suspicious_google_tokens_no_longer_match() {
        // These tokens were removed in v0.2.0 (absent from Google's official
        // common-crawlers list). Ensure they no longer match.
        for ua in [
            "Mozilla/5.0 (compatible; Google-Speakr/1.0)",
            "Mozilla/5.0 (compatible; Google-CWS/1.0)",
            "Mozilla/5.0 (compatible; GoogleProducer/1.0)",
            "Mozilla/5.0 (compatible; Google-Pinpoint/1.0)",
            "Mozilla/5.0 (compatible; Google Favicon/1.0)",
        ] {
            assert!(
                identify(ua).is_none(),
                "removed token should not match; ua={ua:?}"
            );
        }
    }

    #[test]
    fn uptime_monitor_tokens_match() {
        let cases: &[(&str, &str)] = &[
            ("Mozilla/5.0 (compatible; Pingdom.com_bot_version_1.4; +https://pingdom.com)", "pingdom"),
            ("Mozilla/5.0 (compatible; UptimeRobot/2.0; http://uptimerobot.com/)", "uptimerobot"),
            ("Mozilla/5.0 (compatible; Site24x7)", "site24x7"),
            ("Mozilla/5.0 (compatible; NewRelicPinger/1.0)", "newrelicpinger"),
        ];
        for (ua, expected_token) in cases {
            let id = identify(ua).unwrap_or_else(|| panic!("should match: {ua:?}"));
            assert_eq!(id.token, *expected_token, "ua={ua:?}");
            assert_eq!(id.category, UserAgentTokenCategory::UptimeMonitor, "ua={ua:?}");
        }
    }
}
