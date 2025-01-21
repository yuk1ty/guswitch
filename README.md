# guswitch

`gsu`(*g*it *s*witch *u*ser) - A straightforward tool to switch git users on your machine, written in Rust.

![demo](./demo.gif)

## Getting Started

```
$ cargo install --git https://github.com/yuk1ty/guswitch.git
```

## Commands

### Switch global git user

```
$ gsu
```

### Switch local git user

```
$ gsu --local
```

### Show configured users on your machine

```
$ gsu list
```

## Configuration File

```
[[users]]
name = <git user name>
email = <git user email>
description = <description>
```

