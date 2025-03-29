//! SmashLang Testing Framework
//!
//! This module provides a simple, expressive testing framework for SmashLang.
//! It includes functions for defining tests, making assertions, and organizing
//! tests into logical groups.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::panic;

/// Global test registry to store all registered tests
static TEST_REGISTRY: Mutex<Option<TestRegistry>> = Mutex::new(None);

/// Represents a single test case
pub struct TestCase {
    description: String,
    test_fn: Box<dyn Fn() -> TestResult + Send + 'static>,
    tags: Vec<String>,
}

/// Represents a test result
pub enum TestResult {
    Pass,
    Fail(String),
    Skip(String),
}

/// Test registry to store and manage tests
pub struct TestRegistry {
    tests: Vec<TestCase>,
    before_each_hooks: Vec<Box<dyn Fn() + Send + 'static>>,
    after_each_hooks: Vec<Box<dyn Fn() + Send + 'static>>,
    current_describe: Vec<String>,
}

impl TestRegistry {
    /// Create a new test registry
    pub fn new() -> Self {
        TestRegistry {
            tests: Vec::new(),
            before_each_hooks: Vec::new(),
            after_each_hooks: Vec::new(),
            current_describe: Vec::new(),
        }
    }

    /// Register a new test
    pub fn register_test(&mut self, description: &str, test_fn: Box<dyn Fn() -> TestResult + Send + 'static>, tags: Vec<String>) {
        let full_description = if self.current_describe.is_empty() {
            description.to_string()
        } else {
            format!("{} {}", self.current_describe.join(" "), description)
        };

        self.tests.push(TestCase {
            description: full_description,
            test_fn,
            tags,
        });
    }

    /// Register a before_each hook
    pub fn register_before_each(&mut self, hook: Box<dyn Fn() + Send + 'static>) {
        self.before_each_hooks.push(hook);
    }

    /// Register an after_each hook
    pub fn register_after_each(&mut self, hook: Box<dyn Fn() + Send + 'static>) {
        self.after_each_hooks.push(hook);
    }

    /// Push a describe block onto the stack
    pub fn push_describe(&mut self, description: &str) {
        self.current_describe.push(description.to_string());
    }

    /// Pop a describe block from the stack
    pub fn pop_describe(&mut self) {
        self.current_describe.pop();
    }

    /// Run all registered tests
    pub fn run_tests(&self, filter_tag: Option<&str>) -> TestResults {
        let mut results = TestResults::new();

        for test in &self.tests {
            // Skip tests that don't match the tag filter
            if let Some(tag) = filter_tag {
                if !test.tags.iter().any(|t| t == tag) {
                    continue;
                }
            }

            // Run before_each hooks
            for hook in &self.before_each_hooks {
                hook();
            }

            // Run the test and capture the result
            let result = panic::catch_unwind(|| {
                (test.test_fn)()
            });

            // Process the result
            match result {
                Ok(TestResult::Pass) => {
                    results.passed += 1;
                    results.test_results.insert(test.description.clone(), TestResultStatus::Pass);
                }
                Ok(TestResult::Fail(message)) => {
                    results.failed += 1;
                    results.test_results.insert(test.description.clone(), TestResultStatus::Fail(message));
                }
                Ok(TestResult::Skip(message)) => {
                    results.skipped += 1;
                    results.test_results.insert(test.description.clone(), TestResultStatus::Skip(message));
                }
                Err(_) => {
                    results.failed += 1;
                    results.test_results.insert(test.description.clone(), 
                        TestResultStatus::Fail("Test panicked".to_string()));
                }
            }

            // Run after_each hooks
            for hook in &self.after_each_hooks {
                hook();
            }
        }

        results
    }
}

/// Status of a test result
pub enum TestResultStatus {
    Pass,
    Fail(String),
    Skip(String),
}

/// Collection of test results
pub struct TestResults {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub test_results: HashMap<String, TestResultStatus>,
}

impl TestResults {
    /// Create a new test results collection
    pub fn new() -> Self {
        TestResults {
            passed: 0,
            failed: 0,
            skipped: 0,
            test_results: HashMap::new(),
        }
    }
}

/// Initialize the test registry
pub fn init_testing() {
    let mut registry = TEST_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(TestRegistry::new());
    }
}

/// Define a test
pub fn test(description: &str, test_fn: impl Fn() -> TestResult + Send + 'static) {
    let mut registry = TEST_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(TestRegistry::new());
    }
    
    if let Some(registry) = registry.as_mut() {
        registry.register_test(description, Box::new(test_fn), Vec::new());
    }
}

/// Define a test with tags
pub fn test_with_tags(description: &str, tags: Vec<&str>, test_fn: impl Fn() -> TestResult + Send + 'static) {
    let mut registry = TEST_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(TestRegistry::new());
    }
    
    if let Some(registry) = registry.as_mut() {
        let tags = tags.into_iter().map(|s| s.to_string()).collect();
        registry.register_test(description, Box::new(test_fn), tags);
    }
}

/// Group tests together
pub fn describe(description: &str, suite_fn: impl Fn()) {
    let mut registry = TEST_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(TestRegistry::new());
    }
    
    if let Some(registry) = registry.as_mut() {
        registry.push_describe(description);
    }
    
    suite_fn();
    
    let mut registry = TEST_REGISTRY.lock().unwrap();
    if let Some(registry) = registry.as_mut() {
        registry.pop_describe();
    }
}

/// Run before each test in the current describe block
pub fn before_each(hook: impl Fn() + Send + 'static) {
    let mut registry = TEST_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(TestRegistry::new());
    }
    
    if let Some(registry) = registry.as_mut() {
        registry.register_before_each(Box::new(hook));
    }
}

/// Run after each test in the current describe block
pub fn after_each(hook: impl Fn() + Send + 'static) {
    let mut registry = TEST_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(TestRegistry::new());
    }
    
    if let Some(registry) = registry.as_mut() {
        registry.register_after_each(Box::new(hook));
    }
}

/// Run all registered tests
pub fn run_tests(filter_tag: Option<&str>) -> TestResults {
    let registry = TEST_REGISTRY.lock().unwrap();
    if let Some(registry) = registry.as_ref() {
        registry.run_tests(filter_tag)
    } else {
        TestResults::new()
    }
}

/// Assertion value wrapper
pub struct Expect<T> {
    actual: T,
}

impl<T: PartialEq + std::fmt::Debug> Expect<T> {
    /// Create a new expectation
    pub fn new(actual: T) -> Self {
        Expect { actual }
    }
    
    /// Assert that the actual value equals the expected value
    pub fn to_be(&self, expected: T) -> TestResult {
        if self.actual == expected {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected {:?} to be {:?}", self.actual, expected))
        }
    }
    
    /// Assert that the actual value does not equal the expected value
    pub fn not_to_be(&self, expected: T) -> TestResult {
        if self.actual != expected {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected {:?} not to be {:?}", self.actual, expected))
        }
    }
}

impl Expect<bool> {
    /// Assert that the value is true
    pub fn to_be_true(&self) -> TestResult {
        if self.actual {
            TestResult::Pass
        } else {
            TestResult::Fail("Expected true but got false".to_string())
        }
    }
    
    /// Assert that the value is false
    pub fn to_be_false(&self) -> TestResult {
        if !self.actual {
            TestResult::Pass
        } else {
            TestResult::Fail("Expected false but got true".to_string())
        }
    }
}

impl<T: std::fmt::Debug> Expect<Option<T>> {
    /// Assert that the option has a value
    pub fn to_be_some(&self) -> TestResult {
        if self.actual.is_some() {
            TestResult::Pass
        } else {
            TestResult::Fail("Expected Some but got None".to_string())
        }
    }
    
    /// Assert that the option is None
    pub fn to_be_none(&self) -> TestResult {
        if self.actual.is_none() {
            TestResult::Pass
        } else {
            TestResult::Fail(format!("Expected None but got Some({:?})", self.actual.as_ref().unwrap()))
        }
    }
}

impl<T: std::fmt::Debug, E: std::fmt::Debug> Expect<Result<T, E>> {
    /// Assert that the result is Ok
    pub fn to_be_ok(&self) -> TestResult {
        match &self.actual {
            Ok(_) => TestResult::Pass,
            Err(e) => TestResult::Fail(format!("Expected Ok but got Err({:?})", e)),
        }
    }
    
    /// Assert that the result is Err
    pub fn to_be_err(&self) -> TestResult {
        match &self.actual {
            Ok(v) => TestResult::Fail(format!("Expected Err but got Ok({:?})", v)),
            Err(_) => TestResult::Pass,
        }
    }
}

/// Create a new expectation
pub fn expect<T>(actual: T) -> Expect<T> {
    Expect::new(actual)
}

/// Skip a test with a reason
pub fn skip(reason: &str) -> TestResult {
    TestResult::Skip(reason.to_string())
}

/// Assert that a function throws an error
pub fn expect_throws<F, T, E>(f: F) -> TestResult 
where
    F: FnOnce() -> Result<T, E>,
    E: std::fmt::Debug,
{
    match f() {
        Ok(_) => TestResult::Fail("Expected function to throw an error, but it didn't".to_string()),
        Err(_) => TestResult::Pass,
    }
}
