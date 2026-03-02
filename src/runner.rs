pub mod tester;

#[derive(Debug)]
pub enum TestResult {
    Ac,
    Wa { actual: String, expected: String },
    Re { stderr: String },
    Tle,
}
