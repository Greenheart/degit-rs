# degit-rs

Forked from [psnszsn/degit-rs](https://github.com/psnszsn/degit-rs) with additional features to support:

- Tags
- Branches
- Commits

`degit-rs` is a rust rewrite of [degit](https://github.com/Rich-Harris/degit/). It downloads copies of git repositories from the internet, without the need for `git clone`. This is faster, since it does not download the `.git` folder (which contains all the git history) and allows you to initialize a new repository afterwards. It is useful for downloading project boilerplate templates.

[![Crates.io](https://img.shields.io/crates/v/degit-rs.svg)](https://crates.io/crates/degit-rs)

## Installation

```sh
cargo install degit-rs
```

Or install build and install the latest version from a local copy of this repo:

```sh
cargo install --path .
```

## Usage examples

Simplest usage downloads the main branch on GitHub to the current working directory, or to a target directory if you provide it.

```sh
degit user/repo
degit user/repo path/to/project
```

You can also download from GitLab and Bitbucket using `gitlab:user/repo` and `bitbucket:user/repo`, respectively. Or you can specify something other than the main branch with:

```sh
degit user/repo#dev         # by branch
degit user/repo#v1.2.3      # by tag
degit user/repo#1234abcd    # by commit hash
```

Only download a specific subdirectory:

```sh
degit user/repo/some/directory
```

## Advantages over original degit

- Does not require Node.js
- Does not create files in your home directory
- Supports hosted GitLab instances
- Progress bar
