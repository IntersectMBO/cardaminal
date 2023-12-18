# Contributing to Cardaminal

Hello! We're really excited that you are interested in contributing to Cardaminal. If you're unsure where to start, look for issues tagged with `good first issue`, or chat with us on our community forum. 

Please take a moment to review this document in order to make the contribution process easy and effective for everyone involved.

Following these guidelines helps to communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect in addressing your issue, assessing changes, and helping you finalize your pull requests.

## Ground Rules

- Be welcoming to newcomers and encourage diverse new contributors from all backgrounds. See the our [Code of Conduct](CODE_OF_CONDUCT.md).
- Create issues for any major changes and enhancements that you wish to make. Discuss things transparently and get community feedback.
- Be respectful of the opinions of others, and keep an open mind to different ways of doing things.
- Keep pull-requests as small as possible, one new feature or bugfix per PR. 

## Your First Contribution

Unsure where to begin contributing to Cardaminal? You can start by looking through these `beginner` and `help-wanted` issues:

- Beginner issues - issues which should only require a few lines of code, and a test or two.
- Help wanted issues - issues which should be a bit more involved than `beginner` issues.

## Getting Started

To get started with contributing to Cardaminal, follow these steps:

1. Fork the repository on GitHub.
2. Clone the forked repository to your local machine with `git clone https://github.com/IntersectMBO/cardaminal`.
3. Navigate to the cloned directory and run `cargo build` to compile the project.
4. Run `cargo run` to execute the project.
5. After making your changes, run `cargo test` to ensure all tests pass.
6. If all tests pass, commit your changes and push your branch to your fork.
7. Finally, create a pull request in the Cardaminal repository.

## Pull Request Process

When you are opening a pull request, your changes need to be approved by the codeowners before they can be merged. Additionally, all of your changes need to pass the CI validation checks. As a Rust project, some common commands that are used in our CI workflow are:

- `cargo build` - to compile the project
- `cargo clippy` - for linting to catch common mistakes and improve your Rust code
- `cargo test` - for running the tests
- `cargo fmt` - to ensure your code adheres to Rust's formatting conventions

You should run these commands locally to anticipate issues and avoid unnecessary execution on the Github CI. 

## Testing

You can run unit tests locally using the `cargo test` command. On submitting a PR, our CI workflow will run integration tests to ensure that your changes have not broken anything.

## How to report a bug

When filing an issue, make sure to answer these five questions:

1. What version of Rust are you using (`rustc --version`)?
2. What operating system and processor architecture are you using?
3. What did you do?
4. What did you expect to see?
5. What did you see instead?

## How to suggest a feature or enhancement

Open an issue which describes the feature you would like to see, why you need it, and how it should work.

## Code review process

The core team looks at Pull Requests on a regular basis. PRs will need and approval from CODE-OWNERS before merge.

After feedback has been given, we expect responses within two weeks. After two weeks, we may close the pull request if it isn't showing any activity.
