// @ts-check

/** @type {Record<string, string[]>} */
module.exports = {
  "*.{rs}": ["cargo fmt --all"],
  "*.{toml}": ["npm run fmt"],
};
