## Logs

These are the log levels, ordered from least to most severe.

```
TRACE < DEBUG < INFO < WARN < ERROR < CRITICAL
```

The `DEBUG_LEVEL` environment variable controls which log levels are displayed.

By default, `DEBUG_LEVEL` is set to "INFO", meaning that INFO, WARN, ERROR, and CRITICAL logs will be visible in the system. To see more detailed logs, set `DEBUG_LEVEL` to a lower level, such as "DEBUG" or "TRACE".
