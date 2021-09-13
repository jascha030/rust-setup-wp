# rust-setup-wp

Small CLI tool to set up a WordPress environment.
Weirdly written in Rust, to try out the language.

## Usage

```shell
setup-wp -- <name> <user>
```

Arguments:
* **name**: Project name, doubles as dirname. `name` prefixed with `wp_` will be used as database name
* **user**: Database username

User will be prompted for the mysql user password during execution.