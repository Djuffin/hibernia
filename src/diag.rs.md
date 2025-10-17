# `diag.rs` - Diagnostic Logging

This file provides a simple logging implementation for the Hibernia decoder. It uses the `log` crate to provide a basic logging infrastructure.

## Key Data Structures

### `SimpleLogger`

A struct that implements the `log::Log` trait. It provides a simple logging implementation that prints log messages to the console.

## Core Functionality

### `init`

Initializes the logger. It takes a boolean `trace` flag, which, if set to `true`, enables `Trace` level logging. Otherwise, it defaults to `Info` level.

## Usage

The `init` function should be called at the beginning of the program to set up the logger. Once initialized, the `log` macros (`info!`, `warn!`, `error!`, `debug!`, `trace!`) can be used throughout the application to log messages.
