Unit Tests: Put in the same file of the code they are testing
- Put in a module called "tests" with #[cfg(test)]

Integration Tests:
 - Create a tests directory (`./tests/integration_test.rs`)
 - Each file in tests is a separate crate


 Because we can't make integration tests for main.rs, we often offset the main functionality to lib.rs and use main.rs as the 'user' and test the actual code in the lib.rs
