# srl — Spaced Repetition Learning TUI in Rust

<img width="1100" height="410" alt="Screenshot 2025-12-29 at 10 14 42 PM" src="https://github.com/user-attachments/assets/338eefc1-c7bb-44f2-a9dc-1794d1e9af86" />

based heavily on [this project](https://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-clihttps://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-clihttps://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-clihttps://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-clihttps://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-clihttps://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-clihttps://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-clihttps://github.com/HayesBarber/spaced-repetition-learning?tab=readme-ov-file#srl--spaced-repetition-learning-cli)



## Overview

This tool is a general purpose spaced repetition learning tool, meant for helping users attempt to do better at leet code style
interview questions. Using repetition as a guide to enhance learning, this tool allows users to easily track and update their progress.

> [!NOTE]
> This project currently uses `rustc 1.91.1`

To check your rust version run:

```
rustc --version
```

if you do not have rust installed on your machine, visit the following [link](https://rust-lang.org/tools/install/)

To update to a stable version run:

```
rustup update
```

To run this applicaton use the following:
```
cargo run 
```

## Data Storage

We use sqlite to store all data locally, so nothing ever leaves your device.

Furthermore we use [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) as our library for storing data.

If you wish to add functionality, look below as an example from [add_problem_screen.rs](https://github.com/AlessandroB1298/srl_r/blob/main/src/screens/add_problem_screen.rs)

```rust
fn insert_new_problem(
    db: &Arc<rusqlite::Connection>,
    problem_name: &String,
    problem_rating: &String,
    entry_date: &String,
) -> rusqlite::Result<bool> {
    // Changed return type
    let table_name = "user_problems";

    // Ensure table exists
    if !check_table(db, table_name).unwrap() {
        create_table(db)?;
    }

    // Check if row exists
    if !check_row_exists(db, problem_name).unwrap() {
        let problem = Problem {
            name: problem_name.to_string(),
            rating: problem_rating.to_string(),
            entry_date: entry_date.to_string(),
        };
        db.execute(
            "INSERT INTO user_problems (problem_name, problem_rating, entry_date) VALUES (?1, ?2, ?3)",
            (&problem.name, &problem.rating, &problem.entry_date),
        )?;
        return Ok(true); // Signifies a new row was added
    }

    Ok(false) // Signifies nothing was added, but no error occurred
}

```

## TUI

Leveraging [ratatui](https://ratatui.rs/) to create stunning visuals, with minimal latency.

<img width="1700" height="186" alt="Screenshot 2025-12-29 at 10 47 38 PM" src="https://github.com/user-attachments/assets/ec95084b-2960-4555-9cfd-0dc39143f8fe" />


## Todo
- [ ] Add delete capabilities in view screen
- [ ] Add usage graph & screen
- [ ] Clean up code and test

## Open Source Notes

This is an open sourced project, so feel free to contribute, and leave feedback!


