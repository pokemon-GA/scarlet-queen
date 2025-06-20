# Scarlet Queen

Scarlet Queen is a Rust framework for co-evolution.

## What mean Scarlet Queen?

The name is inspired by Red Queen's Hypothesis and Pokémon Scarlet and Violet.

Red Queen's Hypothesis is a hypothesis about co-evolution.

## Project Dependencies

```mermaid
flowchart RL
    initializer[initializer] --> core[core]
    fitness[fitness] --> core[core]
    selector[selector] --> core[core]
    generator[generator] --> core[core]

    generation[generation] --> fitness[fitness]
    generation[generation] --> selector[selector]
    generation[generation] --> generator[generator]

    entrypoint["entrypoint (binary crate)"] --> initializer[initializer]
    entrypoint["entrypoint (binary crate)"] --> generation[generation]
```

## Life Cycle

```mermaid
flowchart LR
    initializer[initialize] --> fitness[fitness]
    subgraph generation
        fitness[fitness] --> selector[selector]
        selector[selector] --> generator[generator]
        generator[generator] --> fitness[fitness]
    end
```

## Project Structure

### `core` (library crate)

Contains the core type definition and logic.

### `initializer` (library crate)

Contains the logic for initializing the group (environment).

- feature flags
  - `full`: enables the full initialization pattern.
  - `random`: enables the random initialization.
  - `weight`: enables the weighted initialization.

### `fitness` (library crate)

Contains the fitness evaluation logic.

### `selector` (library crate)

Contains the logic for selecting individuals for the next generation.

- feature flags
  - `full`: enables the full selection pattern.
  - `ramdom`: enables the random selection.
  - `roulette`: enables the roulette selection.
  - `tournament`: enables the tournament selection.

### `generator` (library crate)

Contains the logic for generating new individuals.

- feature flags
  - `full`: enables the full generation pattern.
  - `random`: enables the random generation.
  - `novelty`: enables the novelty search algorithm.

### `generation` (library crate)

Contains the logic for managing the generation process.

### `entrypoint` (binary crate)

The binary crate that runs the Scarlet Queen framework.

## Get Started

```sh
cargo run --bin entrypoint
```

## Tests

- run all tests

```sh
cargo test
```
- run tests for a specific package

```sh
cargo test -p <test_name>
```
