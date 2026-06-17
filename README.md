# Spider Rules

Spider/crawler identification rules, extracted as a common dependency for
multiple internal services. The public surface is intentionally minimal:
callers pass a single `user_agent: &str` and get back the matched rule.

## Usage

```rust
use spider_rules::{identify, UserAgentTokenCategory};

let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
match identify(ua) {
    Some(id) => {
        println!("token={}, category={}", id.token, id.category.as_str());
    }
    None => println!("not a known spider"),
}
```

- `identify(ua) -> Option<Identification>` is case-insensitive.
- Among all matching rules the **longest** token wins; ties are broken by
  declaration order (first declared wins).
- `Identification { token, category }` exposes the matched lower-case token so
  callers can log / track *why* a UA matched.

## Categories

| `UserAgentTokenCategory`        | `as_str()`                       | Representative bots                              |
|---------------------------------|----------------------------------|--------------------------------------------------|
| `AiSearchAnswerCrawler`         | `ai_search_answer_crawler`       | GPTBot, ClaudeBot, anthropic-ai, PerplexityBot, Bytespider  |
| `SearchIndexCrawler`            | `search_index_crawler`           | Googlebot, Bingbot, DuckDuckBot, Baiduspider     |
| `AdPlatformValidator`           | `ad_platform_validator`          | AdsBot-Google, Mediapartners-Google, YandexDirect|
| `SeoMarketingCrawler`           | `seo_marketing_crawler`          | AhrefsBot, SemrushBot, MJ12bot, DotBot           |
| `SocialLinkPreviewFetcher`      | `social_link_preview_fetcher`    | facebookexternalhit, Slackbot, Twitterbot, WhatsApp |
| `UptimeMonitor`                 | `uptime_monitor`                 | Pingdom, UptimeRobot, Site24x7, StatusCake       |

## Command-line demo

The crate also ships a tiny binary for manual checks:

```sh
cargo run -- "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
# Matched: token=googlebot, category=search_index_crawler

cargo run -- "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0"
# No match
```

## Versioning & upgrades

This crate follows semantic versioning on top of the rules table:

- Rule additions or category adjustments → minor bump (e.g. `0.1.0` → `0.2.0`),
  tagged with a new release tag. Pin to a tag for reproducible builds.
- Breaking public-API changes → major bump (once `1.0.0` is reached; during the
  `0.x` period minor bumps may also include breaking changes).

See `docs/features/initial/spec.md` §四 for the release policy.
