# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

![image](https://github.com/onkr0d/transparadon/assets/155204136/8e34d122-2df3-4011-b89d-09499b406134)
![image](https://github.com/onkr0d/transparadon/assets/155204136/a495ce3b-22a7-4c72-a6c6-b36976b45c0c)
![image](https://github.com/onkr0d/transparadon/assets/155204136/181bb428-c355-4d03-9c01-4b8b4fd5dc0c)

