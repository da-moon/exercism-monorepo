# Rust Refactoring Guidelines

## Rules Table

| Rule ID | Rule Summary                                                     |
| ------- | ---------------------------------------------------------------- |
| R1      | Maximum 100 lines per file, split immediately when exceeded      |
| R2      | Maximum 20 lines per function, single responsibility             |
| R3      | All modules must be directories, never single files              |
| R4      | Each module directory has local errors.rs file                   |
| R5      | Use mod.rs for clean re-exports from submodules                  |
| R6      | One-to-one tests in flat tests/ directory (no subdirectories)    |
| R7      | Test filenames flatten source paths with underscores              |
| R8      | Common functionality goes in common/ directory                   |
| R9      | Shared traits in common/traits.rs or common/traits/              |
| R10     | Constants in constants.rs or common/constants.rs                 |
| R11     | Use functional combinators over imperative control flow          |
| R12     | Prefer .map(), .filter(), .fold() over explicit loops            |
| R13     | Use .and_then(), .or_else() instead of if-let for Option/Result  |
| R14     | Use .transpose() for Option<Result<T, E>> → Result<Option<T>, E> |
| R15     | Use iterator chains for data transformation pipelines            |
| R16     | Use derive macros instead of manual implementations              |
| R17     | Prefer #[derive(Default)] over manual Default::default()         |
| R18     | Use serde macros for serialization instead of manual impl        |
| R19     | Use derive_builder for builder patterns                          |
| R20     | Use thiserror macros for error types                             |
| R21     | Design interfaces using traits for flexibility                   |
| R22     | Implement traits for structs rather than standalone functions    |
| R23     | Use trait objects for runtime polymorphism when needed           |
| R24     | Define generic functions with trait bounds                       |
| R25     | Prefer struct methods over functions taking struct pointers      |
| R26     | Use impl blocks to group related functionality                   |
| R27     | Use &self, &mut self, or self appropriately                      |
| R28     | Keep struct methods focused and atomic                           |
| R29     | Use enums instead of strings for classification/known types      |
| R30     | Use constants instead of static strings                          |

---

## R1: Maximum 100 lines per file, split immediately when exceeded

```rust
// ❌ BAD - Large file with multiple responsibilities
// src/parser.rs (150 lines)
pub struct JsonParser { ... }
impl JsonParser { ... }
pub struct XmlParser { ... }
impl XmlParser { ... }
pub struct YamlParser { ... }
impl YamlParser { ... }

// ✅ GOOD - Split into focused modules
// src/parser/mod.rs
pub mod json;
pub mod xml;
pub mod yaml;

// src/parser/json/mod.rs (< 100 lines)
pub struct JsonParser { ... }
impl JsonParser { ... }
```

## R2: Maximum 20 lines per function, single responsibility

```rust
// ❌ BAD - Large function with multiple responsibilities
pub fn process_document(content: String) -> Result<Document, Error> {
    if content.is_empty() {
        return Err(Error::EmptyContent);
    }
    let cleaned = content.trim().to_string();
    let lines: Vec<&str> = cleaned.lines().collect();
    let mut sections = Vec::new();
    for line in lines {
        if line.starts_with("##") {
            sections.push(Section::new(line));
        }
    }
    // ... 15 more lines
}

// ✅ GOOD - Atomic functions with single responsibility
pub fn process_document(content: String) -> Result<Document, Error> {
    let validated = validate_content(content)?;
    let cleaned = clean_content(validated)?;
    let sections = parse_sections(cleaned)?;
    build_document(sections)
}

fn validate_content(content: String) -> Result<String, Error> { ... }
fn clean_content(content: String) -> Result<String, Error> { ... }
fn parse_sections(content: String) -> Result<Vec<Section>, Error> { ... }
fn build_document(sections: Vec<Section>) -> Result<Document, Error> { ... }
```

## R3: All modules must be directories, never single files

```rust
// ❌ BAD - Single file modules
src/
├── parser.rs
├── validator.rs
└── serializer.rs

// ✅ GOOD - Directory-based modules (uniform pattern)
src/
├── parser/
│   ├── mod.rs
│   ├── errors.rs
│   └── json.rs
├── validator/
│   ├── mod.rs
│   ├── errors.rs
│   └── schema.rs
└── serializer/
    ├── mod.rs
    ├── errors.rs
    └── json.rs
```

## R4: Each module directory has local errors.rs file

```rust
// ✅ GOOD - Local error types
// src/parser/errors.rs
#[derive(Debug, Error, Diagnostic)]
pub enum ParserError {
    #[error("Invalid format: {format}")]
    #[diagnostic(code(parser::invalid_format))]
    InvalidFormat { format: String },

    #[error("Parse failed: {reason}")]
    #[diagnostic(code(parser::parse_failed))]
    ParseFailed { reason: String },
}

// src/parser/mod.rs
pub mod errors;
pub use errors::ParserError;
```

## R5: Use mod.rs for clean re-exports from submodules

```rust
// ✅ GOOD - Clean re-exports
// src/parser/mod.rs
mod json;
mod xml;
mod yaml;
pub mod errors;

pub use json::JsonParser;
pub use xml::XmlParser;
pub use yaml::YamlParser;
pub use errors::ParserError;

// Public API is clean
use crate::parser::{JsonParser, XmlParser, ParserError};
```

## R6: One-to-one mapping between test files and source files

```rust
// ✅ GOOD - Flat tests/ directory with one file per source file
// Place all test files directly under tests/, with no subdirectories.
// Each workspace member crate maintains its own top-level tests/ directory.

src/
├── file.rs
└── module/
    └── file.rs

tests/
├── file.rs          // Tests src/file.rs
└── module_file.rs   // Tests src/module/file.rs

// Deeper paths flatten similarly:
src/a/b/c/file.rs  →  tests/a_b_c_file.rs
```

## R7: Test file naming mirrors directory structure

```rust
// ✅ GOOD - Underscore-delimited filenames in a flat tests/ directory
// Mapping examples:
// src/file.rs                   → tests/file.rs
// src/module/file.rs            → tests/module_file.rs
// src/a/b/c/file.rs             → tests/a_b_c_file.rs
// src/parser/xml/parser.rs      → tests/parser_xml_parser.rs
//
// ❌ NOT nested under tests/:
// tests/module/file.rs          // do not nest test files
// tests/a/b/c/file.rs           // do not nest test files
//
// ❌ Do not include the src/ prefix in names:
// tests/src_module_file.rs      // incorrect
```

## R8: Common functionality goes in common/ directory

```rust
// ✅ GOOD - Common directory structure
src/parser/
├── mod.rs
├── common/
│   ├── mod.rs
│   ├── traits.rs    // Shared traits
│   ├── utils.rs     // Shared utilities
│   └── constants.rs // Shared constants
├── json/
│   ├── mod.rs
│   ├── errors.rs
│   └── parser.rs
└── xml/
    ├── mod.rs
    ├── errors.rs
    └── parser.rs
```

## R9: Shared traits in common/traits.rs or common/traits/

```rust
// ✅ GOOD - Shared traits
// src/parser/common/traits.rs
pub trait Parser {
    type Output;
    type Error;

    fn parse(&self, input: String) -> Result<Self::Output, Self::Error>;
    fn validate(&self, input: &str) -> bool;
}

pub trait Serializer {
    type Error;

    fn serialize<T: serde::Serialize>(&self, value: &T) -> Result<String, Self::Error>;
}

// src/parser/common/mod.rs
pub mod traits;
pub use traits::*;
```

## R10: Constants in constants.rs or common/constants.rs

```rust
// ✅ GOOD - Centralized constants
// src/parser/constants.rs
pub const MAX_DEPTH: usize = 100;
pub const DEFAULT_TIMEOUT: u64 = 30;
pub const XML_HEADER: &str = "<?xml version=\"1.0\"?>";

// src/parser/common/constants.rs
pub const BUFFER_SIZE: usize = 8192;
pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB

// ❌ BAD - Inline magic strings
fn validate_xml(content: &str) -> bool {
    content.starts_with("<?xml version=\"1.0\"?>")
}

// ✅ GOOD - Use constants
fn validate_xml(content: &str) -> bool {
    content.starts_with(XML_HEADER)
}
```

## R11: Use functional combinators over imperative control flow

```rust
// ❌ BAD - Imperative style
let mut results = Vec::new();
for item in items {
    if let Some(processed) = process_item(item) {
        if processed.is_valid() {
            results.push(processed);
        }
    }
}

// ✅ GOOD - Functional combinators
let results: Vec<_> = items
    .into_iter()
    .filter_map(|item| process_item(item))
    .filter(|processed| processed.is_valid())
    .collect();
```

## R12: Prefer .map(), .filter(), .fold() over explicit loops

```rust
// ❌ BAD - Explicit loops
let mut sum = 0;
for item in items {
    sum += item.value;
}

let mut filtered = Vec::new();
for item in items {
    if item.is_active {
        filtered.push(item);
    }
}

// ✅ GOOD - Iterator combinators
let sum: i32 = items.iter().map(|item| item.value).sum();

let filtered: Vec<_> = items
    .into_iter()
    .filter(|item| item.is_active)
    .collect();
```

## R13: Use .and_then(), .or_else() instead of if-let for Option/Result

```rust
// ❌ BAD - if-let pattern
let result = if let Some(config) = config_option {
    if let Ok(loaded) = load_config(config) {
        Ok(loaded)
    } else {
        Err(Error::LoadFailed)
    }
} else {
    Ok(Default::default())
};

// ✅ GOOD - Functional combinators
let result = config_option
    .map(|config| load_config(config))
    .transpose()
    .map_err(|_| Error::LoadFailed)?
    .unwrap_or_default();
```

## R14: Use .transpose() for Option<Result<T, E>> → Result<Option<T>, E>

```rust
// ❌ BAD - Manual handling
let result = match config_file {
    Some(path) => match load_config(path) {
        Ok(config) => Ok(Some(config)),
        Err(e) => Err(e),
    },
    None => Ok(None),
};

// ✅ GOOD - Use transpose
let result: Result<Option<Config>, Error> = config_file
    .map(|path| load_config(path))
    .transpose();
```

## R15: Use iterator chains for data transformation pipelines

```rust
// ❌ BAD - Multiple intermediate collections
let mut step1 = Vec::new();
for item in input {
    if item.is_valid() {
        step1.push(item);
    }
}

let mut step2 = Vec::new();
for item in step1 {
    step2.push(transform(item));
}

let mut final_result = Vec::new();
for item in step2 {
    if let Ok(processed) = process(item) {
        final_result.push(processed);
    }
}

// ✅ GOOD - Single iterator chain
let final_result: Vec<_> = input
    .into_iter()
    .filter(|item| item.is_valid())
    .map(|item| transform(item))
    .filter_map(|item| process(item).ok())
    .collect();
```

## R16: Use derive macros instead of manual implementations

```rust
// ❌ BAD - Manual implementations
impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("name", &self.name)
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            enabled: self.enabled,
        }
    }
}

// ✅ GOOD - Derive macros
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub name: String,
    pub enabled: bool,
}
```

## R17: Prefer #[derive(Default)] over manual Default::default()

```rust
// ❌ BAD - Manual Default implementation
impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::new(),
            enabled: false,
            timeout: 30,
        }
    }
}

// ✅ GOOD - Derive Default with custom defaults
#[derive(Default)]
pub struct Config {
    #[default]
    pub name: String,
    pub enabled: bool,
    #[default = "30"]
    pub timeout: u64,
}
```

## R18: Use serde macros for serialization instead of manual impl

```rust
// ❌ BAD - Manual serialization
impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Config", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("enabled", &self.enabled)?;
        state.end()
    }
}

// ✅ GOOD - Serde derive macros
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub enabled: bool,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 { 30 }
```

## R19: Use derive_builder for builder patterns

```rust
// ❌ BAD - Manual builder implementation
pub struct SearchRequestBuilder {
    query: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl SearchRequestBuilder {
    pub fn new() -> Self { ... }
    pub fn query(mut self, query: String) -> Self { ... }
    pub fn limit(mut self, limit: usize) -> Self { ... }
    pub fn build(self) -> Result<SearchRequest, BuildError> { ... }
}

// ✅ GOOD - derive_builder macro
#[derive(Builder)]
#[builder(setter(into))]
pub struct SearchRequest {
    pub query: String,
    #[builder(default = "10")]
    pub limit: usize,
    #[builder(default = "0")]
    pub offset: usize,
}
```

## R20: Use thiserror macros for error types

```rust
// ❌ BAD - Manual error implementation
#[derive(Debug)]
pub enum ParserError {
    InvalidFormat(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            Self::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for ParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(err) => Some(err),
            _ => None,
        }
    }
}

// ✅ GOOD - thiserror macros
#[derive(Debug, Error, Diagnostic)]
pub enum ParserError {
    #[error("Invalid format: {format}")]
    #[diagnostic(code(parser::invalid_format))]
    InvalidFormat { format: String },

    #[error("IO error occurred")]
    #[diagnostic(code(parser::io_error))]
    IoError(#[from] std::io::Error),
}
```

## R21: Design interfaces using traits for flexibility

```rust
// ❌ BAD - Concrete type dependencies
pub struct DocumentProcessor {
    json_parser: JsonParser,
    xml_parser: XmlParser,
}

impl DocumentProcessor {
    pub fn process_json(&self, input: String) -> Result<Document, Error> {
        self.json_parser.parse(input)
    }

    pub fn process_xml(&self, input: String) -> Result<Document, Error> {
        self.xml_parser.parse(input)
    }
}

// ✅ GOOD - Trait-based design
pub trait Parser {
    type Output;
    type Error;

    fn parse(&self, input: String) -> Result<Self::Output, Self::Error>;
}

pub struct DocumentProcessor<P: Parser> {
    parser: P,
}

impl<P: Parser> DocumentProcessor<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn process(&self, input: String) -> Result<P::Output, P::Error> {
        self.parser.parse(input)
    }
}
```

## R22: Implement traits for structs rather than standalone functions

```rust
// ❌ BAD - Standalone functions
pub fn parse_json(parser: &JsonParser, input: String) -> Result<Document, JsonError> {
    // parsing logic
}

pub fn validate_json(parser: &JsonParser, input: &str) -> bool {
    // validation logic
}

// ✅ GOOD - Trait implementation for struct
impl Parser for JsonParser {
    type Output = Document;
    type Error = JsonError;

    fn parse(&self, input: String) -> Result<Self::Output, Self::Error> {
        // parsing logic
    }
}

impl Validator for JsonParser {
    fn validate(&self, input: &str) -> bool {
        // validation logic
    }
}
```

## R23: Use trait objects for runtime polymorphism when needed

```rust
// ✅ GOOD - Trait objects for runtime polymorphism
pub struct DocumentProcessor {
    parsers: Vec<Box<dyn Parser<Output = Document, Error = ParseError>>>,
}

impl DocumentProcessor {
    pub fn add_parser(&mut self, parser: Box<dyn Parser<Output = Document, Error = ParseError>>) {
        self.parsers.push(parser);
    }

    pub fn process_with_all(&self, input: String) -> Vec<Result<Document, ParseError>> {
        self.parsers
            .iter()
            .map(|parser| parser.parse(input.clone()))
            .collect()
    }
}
```

## R24: Define generic functions with trait bounds

```rust
// ❌ BAD - Specific type parameters
pub fn process_json_document(parser: JsonParser, input: String) -> Result<Document, JsonError> {
    parser.parse(input)
}

pub fn process_xml_document(parser: XmlParser, input: String) -> Result<Document, XmlError> {
    parser.parse(input)
}

// ✅ GOOD - Generic with trait bounds
pub fn process_document<P>(parser: P, input: String) -> Result<P::Output, P::Error>
where
    P: Parser,
{
    parser.parse(input)
}

pub fn process_and_validate<P>(parser: P, input: String) -> Result<P::Output, P::Error>
where
    P: Parser + Validator,
{
    if parser.validate(&input) {
        parser.parse(input)
    } else {
        Err(P::Error::validation_failed())
    }
}
```

## R25: Prefer struct methods over functions taking struct pointers

```rust
// ❌ BAD - Functions taking struct pointers
pub fn parse_document(parser: &JsonParser, input: String) -> Result<Document, JsonError> {
    // parsing logic
}

pub fn validate_input(parser: &JsonParser, input: &str) -> bool {
    // validation logic
}

pub fn configure_parser(parser: &mut JsonParser, config: Config) {
    // configuration logic
}

// ✅ GOOD - Struct methods
impl JsonParser {
    pub fn parse(&self, input: String) -> Result<Document, JsonError> {
        // parsing logic
    }

    pub fn validate(&self, input: &str) -> bool {
        // validation logic
    }

    pub fn configure(&mut self, config: Config) {
        // configuration logic
    }
}
```

## R26: Use impl blocks to group related functionality

```rust
// ❌ BAD - Scattered implementations
impl JsonParser {
    pub fn parse(&self, input: String) -> Result<Document, JsonError> { ... }
}

impl JsonParser {
    pub fn new() -> Self { ... }
}

impl JsonParser {
    pub fn validate(&self, input: &str) -> bool { ... }
}

// ✅ GOOD - Grouped functionality
impl JsonParser {
    // Construction
    pub fn new() -> Self { ... }

    pub fn with_config(config: Config) -> Self { ... }

    // Core operations
    pub fn parse(&self, input: String) -> Result<Document, JsonError> { ... }

    pub fn validate(&self, input: &str) -> bool { ... }

    // Configuration
    pub fn set_config(&mut self, config: Config) { ... }
}
```

## R27: Use &self, &mut self, or self appropriately

```rust
// ✅ GOOD - Appropriate self usage
impl JsonParser {
    // Immutable access - use &self
    pub fn parse(&self, input: String) -> Result<Document, JsonError> { ... }

    pub fn validate(&self, input: &str) -> bool { ... }

    // Mutable access - use &mut self
    pub fn configure(&mut self, config: Config) { ... }

    pub fn reset(&mut self) { ... }

    // Consuming - use self
    pub fn into_config(self) -> Config { ... }

    pub fn finalize(self) -> Result<ProcessedParser, JsonError> { ... }
}
```

## R28: Keep struct methods focused and atomic

```rust
// ❌ BAD - Large method with multiple responsibilities
impl DocumentProcessor {
    pub fn process_document(&mut self, input: String) -> Result<Document, ProcessError> {
        // Validation (5 lines)
        if input.is_empty() { return Err(ProcessError::EmptyInput); }
        let trimmed = input.trim();

        // Parsing (10 lines)
        let parsed = match self.format {
            Format::Json => self.parse_json(trimmed)?,
            Format::Xml => self.parse_xml(trimmed)?,
        };

        // Transformation (8 lines)
        let transformed = self.transform_document(parsed)?;

        // Validation (5 lines)
        self.validate_result(&transformed)?;

        Ok(transformed)
    }
}

// ✅ GOOD - Atomic methods
impl DocumentProcessor {
    pub fn process_document(&mut self, input: String) -> Result<Document, ProcessError> {
        let validated = self.validate_input(input)?;
        let parsed = self.parse_document(validated)?;
        let transformed = self.transform_document(parsed)?;
        self.validate_result(&transformed)?;
        Ok(transformed)
    }

    fn validate_input(&self, input: String) -> Result<String, ProcessError> { ... }

    fn parse_document(&self, input: String) -> Result<RawDocument, ProcessError> { ... }

    fn transform_document(&self, raw: RawDocument) -> Result<Document, ProcessError> { ... }

    fn validate_result(&self, doc: &Document) -> Result<(), ProcessError> { ... }
}
```

## R29: Use enums instead of strings for classification/known types

```rust
// ❌ BAD - String-based classification
pub struct DocumentProcessor {
    pub format: String,  // "json", "xml", "yaml"
    pub mode: String,    // "strict", "lenient"
}

pub fn process_document(format: &str, input: String) -> Result<Document, Error> {
    match format {
        "json" => parse_json(input),
        "xml" => parse_xml(input),
        "yaml" => parse_yaml(input),
        _ => Err(Error::UnsupportedFormat),
    }
}

// ✅ GOOD - Enum-based classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentFormat {
    Json,
    Xml,
    Yaml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessingMode {
    Strict,
    Lenient,
}

pub struct DocumentProcessor {
    pub format: DocumentFormat,
    pub mode: ProcessingMode,
}

pub fn process_document(format: DocumentFormat, input: String) -> Result<Document, Error> {
    match format {
        DocumentFormat::Json => parse_json(input),
        DocumentFormat::Xml => parse_xml(input),
        DocumentFormat::Yaml => parse_yaml(input),
    }
}
```

## R30: Use constants instead of static strings

```rust
// ❌ BAD - Magic strings scattered throughout code
pub fn validate_xml(content: &str) -> bool {
    content.starts_with("<?xml version=\"1.0\"?>") &&
    content.contains("</root>")
}

pub fn create_xml_template() -> String {
    format!("{}\n<root></root>", "<?xml version=\"1.0\"?>")
}

// ✅ GOOD - Centralized constants
pub const XML_DECLARATION: &str = "<?xml version=\"1.0\"?>";
pub const XML_ROOT_OPEN: &str = "<root>";
pub const XML_ROOT_CLOSE: &str = "</root>";
pub const DEFAULT_ENCODING: &str = "UTF-8";
pub const MAX_DEPTH: usize = 100;

pub fn validate_xml(content: &str) -> bool {
    content.starts_with(XML_DECLARATION) &&
    content.contains(XML_ROOT_CLOSE)
}

pub fn create_xml_template() -> String {
    format!("{}\n{}{}", XML_DECLARATION, XML_ROOT_OPEN, XML_ROOT_CLOSE)
}
```

---

## Common Libraries Pattern

```rust
// Cargo.toml dependencies
[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
miette = { version = "5.0", features = ["fancy"] }
derive_builder = "0.12"
```

## Directory Structure Pattern

```
src/
├── parser/
│   ├── mod.rs              # Re-exports
│   ├── constants.rs        # Parser constants
│   ├── common/
│   │   ├── mod.rs
│   │   ├── traits.rs       # Common traits
│   │   └── constants.rs    # Shared constants
│   ├── json/
│   │   ├── mod.rs
│   │   ├── errors.rs
│   │   ├── constants.rs
│   │   └── parser.rs
│   └── xml/
│       ├── mod.rs
│       ├── errors.rs
│       ├── constants.rs
│       └── parser.rs
```

## Testing Pattern

Place all integration tests directly in the crate's tests/ directory (no subdirectories). In a workspace, each member crate keeps its own tests/ directory following the same flat, underscore-delimited convention.

```
tests/
├── common_traits.rs   # Tests src/parser/common/traits.rs
├── json_parser.rs     # Tests src/parser/json/parser.rs
└── xml_parser.rs      # Tests src/parser/xml/parser.rs
```

## Error Handling Pattern

```rust
#[derive(Debug, Error, Diagnostic)]
pub enum ModuleError {
    #[error("Invalid input: {input}")]
    #[diagnostic(code(module::invalid_input))]
    InvalidInput { input: String },

    #[error("Processing failed: {reason}")]
    #[diagnostic(code(module::process_failed))]
    ProcessFailed { reason: String },

    #[error("IO error occurred")]
    #[diagnostic(code(module::io_error))]
    IoError(#[from] std::io::Error),
}
```

## Functional Style Pattern

```rust
// Functional transformation pipeline
let result: Result<Vec<Document>, ProcessError> = input_files
    .into_iter()
    .map(|path| self.read_file(path))
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .filter(|content| !content.is_empty())
    .map(|content| self.parse_document(content))
    .collect();

// Option/Result combinators
let config = config_path
    .as_ref()
    .map(|path| self.load_config(path))
    .transpose()
    .map_err(|e| ConfigError::LoadFailed(e))?
    .unwrap_or_default();
```

## Trait-Based Design Pattern

```rust
pub trait Parser {
    type Output;
    type Error;

    fn parse(&self, input: String) -> Result<Self::Output, Self::Error>;
}

pub trait Validator {
    fn validate(&self, input: &str) -> bool;
}

pub struct DocumentProcessor<P>
where
    P: Parser + Validator,
{
    parser: P,
}

impl<P> DocumentProcessor<P>
where
    P: Parser + Validator,
{
    pub fn process(&self, input: String) -> Result<P::Output, P::Error> {
        if self.parser.validate(&input) {
            self.parser.parse(input)
        } else {
            Err(P::Error::validation_failed())
        }
    }
}
```

## Testing Checklist

- [ ] **All files under 100 lines**
- [ ] **All functions under 20 lines**
- [ ] All tests in tests/ directory, never in source files
- [ ] Each workspace member has its own tests/ directory
- [ ] One-to-one mapping between source and test files
- [ ] Test filenames flatten source paths with underscores; no subdirectories in tests/
- [ ] No unwrap() or expect() in production code
- [ ] Derive macros used instead of manual implementations
- [ ] Traits defined for extensibility and flexibility
- [ ] Struct methods preferred over standalone functions
- [ ] Enums used instead of string constants for classification
- [ ] Constants defined instead of magic strings
- [ ] Functional combinators used over imperative control flow
- [ ] Each module directory has errors.rs file
- [ ] All modules are directories, not single files
- [ ] Common functionality centralized in common/ directories
- [ ] Iterator chains used for data transformation
- [ ] Proper use of &self, &mut self, and self in methods
- [ ] Trait objects used appropriately for runtime polymorphism
- [ ] Generic functions defined with proper trait bounds
- [ ] Owned types preferred over references for simplicity
