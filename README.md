# List The Docs CLI

`listthedocs-cli` is a command line client for [List The Docs](https://github.com/allebacco/listthedocs),
a simple Python server that can be used to host a landing page for the documentation
of multiple projects.

## Installation

At the moment, it must be compiled from sources:

    cd /project/root/dir
    cargo build --release

Then add `target/release/listthedocs` to your *PATH*.

## Quick start

Set URL and user API key using environment variables:

    export DOCS_URL=http://localhost:5000
    export DOCS_API_KEY=<user-api-key>

Then list all the projects

    listthedocs project list

add a new project

    listthedocs project add my-project "project-description"

or use a file with a definition (in JSON)

    listthedocs project add -f /path/to/file/json

and add a new version

    listthedocs version add my-project 1.0.0 http://docs.example.com

If you do not remember how a command works, just add `-h` at the end:

    listthedocs version add -h

## Run tests

Make sure to have a *List The Docs* server instance running at
`http://localhost:5000` with root API key equal to `ROOT-API-KEY`.
Then run:

    cargo test -- --ignored
