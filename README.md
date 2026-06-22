# Spider Rules

Spider/crawler identification rules, extracted as a common dependency for
multiple internal services. The public surface is intentionally minimal:
callers pass a single `user_agent: &str` and get back the matched rule.

## Usage

```rust
use spider_rules::{identify, FetchOrigin, UserAgentTokenCategory};

let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
match identify(ua) {
 Some(id) => {
 println!("token={}, category={}, fetch_origin={}",
 id.token, id.category.as_str(), id.fetch_origin().as_str());
 }
 None => println!("not a known spider"),
}
```

- `identify(ua) -> Option<Identification>` is case-insensitive.
- Among all matching rules the **longest** token wins; ties are broken by
  declaration order (first declared wins).
- `Identification { token, category }` exposes the matched lower-case token so
 callers can log / track *why* a UA matched.
- `Identification::fetch_origin()` returns a [`FetchOrigin`] indicating
 whether a real user triggered the request and whether the tool can interact
 with the page — the signal ad networks use for CPC/CPA conversion decisions.

## Categories

| `UserAgentTokenCategory`        | `as_str()`                       | Representative bots                              |
|---------------------------------|----------------------------------|--------------------------------------------------|
| `AiSearchAnswerCrawler`         | `ai_search_answer_crawler`       | GPTBot, ClaudeBot, anthropic-ai, PerplexityBot, Bytespider  |
| `SearchIndexCrawler`            | `search_index_crawler`           | Googlebot, Bingbot, DuckDuckBot, Baiduspider     |
| `AdPlatformValidator`           | `ad_platform_validator`          | AdsBot-Google, Mediapartners-Google, YandexDirect|
| `SeoMarketingCrawler`           | `seo_marketing_crawler`          | AhrefsBot, SemrushBot, MJ12bot, DotBot           |
| `SocialLinkPreviewFetcher`      | `social_link_preview_fetcher`    | facebookexternalhit, Slackbot, Twitterbot, WhatsApp |
| `UptimeMonitor`                 | `uptime_monitor`                 | Pingdom, UptimeRobot, Site24x7, StatusCake       |

## Fetch origin

`Identification::fetch_origin()` provides an orthogonal axis to `category`:
whether the request was triggered by a real human user and whether the tool
can interact with the page. Ad networks use this to decide whether a
conversion should pass.

| `FetchOrigin` | `as_str()` | CPC | CPA |
|---|---|---|---|
| `Autonomous` | `autonomous` | block | block |
| `UserTriggeredFetch` | `user_triggered_fetch` | allow | allow attribution; conversion requires real-user action |
| `UserTriggeredAgentic` | `user_triggered_agentic` | allow | configurable — default block/flag |

The classification is cross-verified against:
- **Google**'s official "user-triggered fetchers" category (fetchers that
 "ignore robots.txt rules" because "the fetch was requested by a user")
- **OpenAI**'s `ChatGPT-User` documentation ("not used for crawling the web
 in an automatic fashion; because these actions are initiated by a user")

User-triggered tokens: `chatgpt-user`, `claude-user`, `perplexity-user`,
`amzn-user`, `mistralai-user`, `gemini-deep-research`, `grok-deepsearch`,
`google-read-aloud`, `yandexuserproxy` (fetch-only); `googleagent` /
GoogleAgent-Mariner (agentic).

## Command-line demo

The crate also ships a tiny binary for manual checks:

```sh
cargo run -- "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
# Matched: token=googlebot, category=search_index_crawler, fetch_origin=autonomous

cargo run -- "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; ChatGPT-User/1.0; +https://openai.com/bot"
# Matched: token=chatgpt-user, category=ai_search_answer_crawler, fetch_origin=user_triggered_fetch

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
