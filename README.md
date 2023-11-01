# `@napi-rs/system-shutdown`

![CI](https://github.com/Brooooooklyn/system-shutdown/workflows/CI/badge.svg)
[![install size](https://packagephobia.com/badge?p=@napi-rs/system-shutdown)](https://packagephobia.com/result?p=@napi-rs/system-shutdown)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/system-shutdown.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/system-shutdown?minimal=true)

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
