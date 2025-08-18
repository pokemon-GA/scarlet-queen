# Setup

To format toml file and rust code, when you  commit, you need to install `husky` and `lint-staged`.

Under the commands is setup commands for `husky`, `lint-staged` and `prettier`.

```sh
npm i && npm exec husky-init -y &&  npm exec husky set .husky/pre-commit "npm exec lint-staged --allow-empty"
```
