RSS feed injector for Discord
===============================

1. To generate a empty `rss.json`, `cargo run` and `Ctrl-c`
2. Edit `rss.json` like a below example
3. `cargo run`

Here is a template of `rss.json`

```json
{
  "receivers": [
    {
      "url": RSS_FEED_URL_IS_HERE,
      "webhook_url": WEBHOOK_URL_IS_HERE
    },
    {
      "url": RSS_FEED_URL_IS_HERE_2,
      "webhook_url": WEBHOOK_URL_IS_HERE_2
    },
    ...
  ]
}
```
