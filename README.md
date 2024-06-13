Here's a `README.md` file for your project using the `app-cli` crate:

```markdown
# Command-Line Application with `app-cli`

This Rust application demonstrates the use of the `app-cli` crate for building a command-line interface (CLI). The application allows users to register a person or a pet with specific command-line arguments.

## Features

- Register a person with first and last name.
- Register a pet with a pet name.
- Optional flag to indicate if the person is wearing a fluffy coat.

## Usage

### Register a Person

To register a person, use the `register-person` subcommand with the required `--first-name` (or aliases `-f`, `--fname`, `--firstname`) and `--last-name` (or aliases `-l`, `--lname`, `--lastname`) arguments.

```bash
cargo run -- register-person --first-name John --last-name Doe
```

or using the short aliases:

```bash
cargo run -- register-person -f John -l Doe
```

### Register a Pet

To register a pet, use the `register-pet` subcommand with the required `--pet-name` argument.

```bash
cargo run -- register-pet --pet-name Fluffy
```

### Optional Arguments

The `--fluffy` argument can be used with any subcommand to indicate if the person is wearing a fluffy coat.

```bash
cargo run -- register-person --first-name John --last-name Doe --fluffy
```

## Installation

To run this application, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/skbhati199/rust_app_cli
cd rust_app_cli
```

Build the project:

```bash
cargo build
```

Run the project:

```bash
cargo run -- <subcommand> [options]
```

## Example

Register a person named John Doe:

```bash
cargo run -- register-person --first-name John --last-name Doe
```

Output:

```plaintext
First Name : John Last name: Doe
```

Register a pet named Fluffy:

```bash
cargo run -- register-pet --pet-name Fluffy
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```

This `README.md` provides an overview of the application, how to use it, and example commands to get started. Adjust the `https://github.com/skbhati199/rust_app_cli` and `rust_app_cli` placeholders to match your actual repository details.
