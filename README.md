# AOC 2024 day 11

CLI syntax: `stones ["apply"|"count"] [#steps] [#stone 1] [#stone 2] [#stone 3] ...`

If no stones are provided the default value will be `[125, 17]`

Examples:
- `cargo run -- apply 25 1 4 7` to apply 25 blinks to stones 1, 4 and 7
- `cargo run -- count 25` to count results after 25 blinks at stones 125 and 17

