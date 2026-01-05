Title: GitHub - powersync-ja/powersync-sqlite-core: PowerSync SQLite Extension

Description: PowerSync SQLite Extension. Contribute to powersync-ja/powersync-sqlite-core development by creating an account on GitHub.                                            

Skip to content  

You signed in with another tab or window. Reload to refresh your session. You signed out in another tab or window. Reload to refresh your session. You switched accounts on another tab or window. Reload to refresh your session. Dismiss alert

powersync-ja / **powersync-sqlite-core** Public

*   Notifications You must be signed in to change notification settings
*   Fork 7
*   Star 32

PowerSync SQLite Extension

### License

Apache-2.0 license

32 stars 7 forks Branches Tags Activity

Star

Notifications You must be signed in to change notification settings

powersync-ja/powersync-sqlite-core
==================================

 main

BranchesTags

Go to file

Code

Open more actions menu

Folders and files
-----------------

| Name | Name | 
Last commit message

| 

Last commit date

|
| --- | --- | --- | --- |
| 

Latest commit
-------------

History
-------

364 Commits

|
| 

.cargo

| 

.cargo

| 

| 

|
| 

.github

| 

.github

| 

| 

|
| 

android

| 

android

| 

| 

|
| 

crates

| 

crates

| 

| 

|
| 

dart

| 

dart

| 

| 

|
| 

docs

| 

docs

| 

| 

|
| 

sqlite-rs-embedded @ 6a868dd

| 

sqlite-rs-embedded @ 6a868dd

| 

| 

|
| 

tool

| 

tool

| 

| 

|
| 

wasm

| 

wasm

| 

| 

|
| 

.gitattributes

| 

.gitattributes

| 

| 

|
| 

.gitignore

| 

.gitignore

| 

| 

|
| 

.gitmodules

| 

.gitmodules

| 

| 

|
| 

Cargo.lock

| 

Cargo.lock

| 

| 

|
| 

Cargo.toml

| 

Cargo.toml

| 

| 

|
| 

LICENSE

| 

LICENSE

| 

| 

|
| 

NOTICE

| 

NOTICE

| 

| 

|
| 

Package.swift

| 

Package.swift

| 

| 

|
| 

README.md

| 

README.md

| 

| 

|
| 

UUID.md

| 

UUID.md

| 

| 

|
| 

powersync-sqlite-core.podspec

| 

powersync-sqlite-core.podspec

| 

| 

|
| 

rust-toolchain.toml

| 

rust-toolchain.toml

| 

| 

|
| 

View all files

|

Repository files navigation
---------------------------

PowerSync SQLite Extension
==========================

This extension is used by PowerSync client SDKs.

The APIs here not currently stable, and may change in any release. The APIs are intended to be used by PowerSync SDKs only.

API
===

Primary APIs:

\-- Load the extension
\-- Sets up functions and views, but does not touch the database itself.
.load powersync

\-- Configure the schemas.
\-- Creates data tables, indexes and views.
SELECT powersync\_replace\_schema('{"tables": \[{"name": "test", "columns": \[{"name": "name", "type": "text"}\]}\]}');

Other APIs:

\-- Initialize the extension data (creates internal tables).
\-- Optional - also called as part of powersync\_replace\_schema().
\-- Only useful to ensure internal tables are configured without touching the schema.
SELECT powersync\_init();

Building and running
====================

Initialize submodules recursively

```
git submodule update --init --recursive
```

# Build the shell
cargo build -p powersync\_sqlite
./target/debug/powersync\_sqlite test.db "select powersync\_rs\_version()"

# Build the loadable extension
cargo build -p powersync\_loadable
sqlite3 ":memory:" ".load ./target/debug/libpowersync" "select powersync\_rs\_version()" #This requires sqlite3 installed

# Build the release loadable extension
cargo build -p powersync\_loadable --release

# Build for iOS
./tool/build\_xcframework.sh

Acknowledgements
================

Structure of the SQLite extension using Rust is inspired by cr-sqlite.

About
-----

PowerSync SQLite Extension

### Resources

Readme

### License

Apache-2.0 license

### Uh oh!

There was an error while loading. Please reload this page.

Activity

Custom properties

### Stars

**32** stars

### Watchers

**12** watching

### Forks

**7** forks

Report repository

Releases 30
-----------

v0.4.4 Latest

Aug 6, 2025

\+ 29 releases

Packages 0
----------

### Uh oh!

There was an error while loading. Please reload this page.

### Uh oh!

There was an error while loading. Please reload this page.

Contributors 8
--------------

Languages
---------

*   C 96.0%
*   Rust 2.5%
*   Dart 1.4%
*   Other 0.1%

You can’t perform that action at this time.