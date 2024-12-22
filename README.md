# guswitch

A straightforward tool to switch git users on your machine, written in Rust.

![demo](./demo.gif)

## Getting Started

```
$ cargo install --git https://github.com/yuk1ty/guswitch.git
```

## Commands

### Switch global git user

```
$ gus
```

### Switch local git user

```
$ gus --local
```

### Show configured users on your machine

```
$ gus list
```

## Configuration File

```
[[users]]
name = <git user name>
email = <git user email>
description = <description>
```

