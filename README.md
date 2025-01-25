# guswitch

`gsu`(*g*it *s*witch *u*ser) - A straightforward tool to switch git users on your machine, written in Rust.

![demo](./demo.gif)

## Getting Started

```
$ cargo install --git https://github.com/yuk1ty/guswitch.git
```

## Commands

### Switch git user

`gsu {switch|s} [--local|--global]` can switch git users. You can omit `--local` flag and will default to `--local` as well.

```
$ gsu switch --local
or
$ gsu s --local
```

### Show all configured users in the configuration file

`gsu {list|ls}` shows git users that are defined in the configuration file.

```
$ gsu list
or
$ gsu ls
```

### Show the current user

`gsu {get|g} [--local|--global]` can display the current user that is configured on your machine. You can omit `--local` flag and will default to `--local` as well.

```
$ gsu get --local
or
$ gsu g --local
```

## Configuration File

```
[[users]]
name = <git user name>
email = <git user email>
description = <description>
```

