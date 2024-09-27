# git-sessionizer

### a little project to automate the creation of a sessionized git repo in your editor of choice, with tmux, or not...

feel free to contribute. PLZ? :D

### I decided to really take the time to learn Rust and figured it would be fun to rerwite a past project. this is that.

- https://github.com/giuseppe-g-gelardi/git-sessionizer

## Started:

- git specific CLI prompts (dialoguer)

## TODO:

- [x] read/write to config file
- [x] user prompts (main)
- [x] user prompts (git specific)
- [x] github auth
- [ ] TUI (https://github.com/console-rs/dialoguer) - changed from ratatui
- [ ] migrate gh auth to device/code flow (maybe)
- [ ] command line args to start session

## config file location:

- windows:
- - APPDATA\local\session_config.yaml

- macos & linux:
- - /home/{user}/.config/session_config.yaml (~/.config/)

## contributing:

1. create a fork
2. create a branch
3. make a pr?
4. get mad at me when you realize i dont know how to merge it -- kidding, mostly

but seriously, would be cool :D
...plz
