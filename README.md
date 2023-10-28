# `@napi-rs/system-shutdown`

This package provides a cross platform way to shut down, reboot or log out operations.

Supported platforms: `Linux`, `Windows` and `macOS`.

# API

```ts
export function shutdown(): void
export function forceShutdown(): void
export function reboot(): void
export function forceReboot(): void
export function logout(): void
export function forceLogout(): void
export function sleep(): void
export function hibernate(): void
```
