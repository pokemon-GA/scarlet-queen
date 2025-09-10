# Structure

## Project Dependencies

```mermaid
flowchart RL
    initializer[scarlet-queen-initializer] --> core[scarlet-queen-core]
    fitness[scarlet-queen-fitness] --> core[scarlet-queen-core]
    selector[scarlet-queen-selector] --> core[scarlet-queen-core]
    replenisher[scarlet-queen-replenisher] --> core[scarlet-queen-core]
    generation[scarlet-queen-generation] --> core[scarlet-queen-core]
    entrypoint["scarlet-queen-entrypoint (binary crate)"] --> core[scarlet-queen-core]

    generation[scarlet-queen-generation] --> fitness[scarlet-queen-fitness]
    generation[scarlet-queen-generation] --> selector[scarlet-queen-selector]
    generation[scarlet-queen-generation] --> replenisher[scarlet-queen-replenisher]

    entrypoint["scarlet-queen-entrypoint (binary crate)"] --> initializer[scarlet-queen-initializer]
    entrypoint["scarlet-queen-entrypoint (binary crate)"] --> generation[scarlet-queen-generation]
```

## Life Cycle

```mermaid
flowchart LR
    initializer["scarlet-queen-initialize (initializer)"] --> fitness["scarlet-queen-arithmetic of fitness (fitness)"]
    subgraph generation
        fitness["scarlet-queen-arithmetic of fitness (fitness)"] --> selector["scarlet-queen-select (selector)"]
        selector["scarlet-queen-select (selector)"] --> replenisher["scarlet-queen-replenisher (replenisher)"]
        replenisher["scarlet-queen-replenisher (replenisher)"] --> fitness["scarlet-queen-arithmetic of fitness (fitness)"]
    end
```

## Project Structure

### `scarlet-queen-core` (library crate)

https://scarlet-queen.netlify.app/doc/scarlet_queen_core/

Contains the core type definition and logic.

### `scarlet-queen-initializer` (library crate)

https://scarlet-queen.netlify.app/doc/scarlet_queen_initializer/

Contains the logic for initializing the group (environment).

- modules
  - `random`: enables the random initialization.

### `scarlet-queen-fitness` (library crate)

https://scarlet-queen.netlify.app/doc/scarlet_queen_fitness/

Contains the fitness evaluation logic.

### `scarlet-queen-selector` (library crate)

https://scarlet-queen.netlify.app/doc/scarlet_queen_selector/

Contains the logic for selecting individuals for the next generation.

- modules
  - `random`: enables the random selection.
  - `roulette`: enables the roulette selection.
  - `tournament`: enables the tournament selection.

### `scarlet-queen-replenisher` (library crate)

https://scarlet-queen.netlify.app/doc/scarlet_queen_replenisher/

Contains the logic for replenishing new individuals.

- modules
  - `random`: enables the random generation.
  - `roulette`: enables the roulette selection.
  - `tournament`: enables the tournament generation.

### `scarlet-queen-generation` (library crate)

https://scarlet-queen.netlify.app/doc/scarlet_queen_generation/

Contains the logic for managing the generation process.

### `scarlet-queen-entrypoint` (binary crate)

https://scarlet-queen.netlify.app/doc/scarlet_queen_entrypoint/

The binary crate that runs the Scarlet Queen framework.
