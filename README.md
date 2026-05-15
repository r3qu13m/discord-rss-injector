RSS feed injector for Discord
===============================

1. Create `rss.json` like a below example
2. `UID=$(id -u) GID=$(id -g) docker compose up --build`

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
