# 🎲 random_word

[![Crates.io](https://img.shields.io/crates/v/random_word.svg)](https://crates.io/crates/random_word)
[![Documentation](https://docs.rs/random_word/badge.svg)](https://docs.rs/random_word)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

> **Efficient random word generation in 7 languages + XKCD-style password generator**

`random_word` is a lightweight, high-performance Rust crate for generating random words across multiple languages. Perfect for generating memorable passwords, test data, placeholder text, or adding linguistic variety to your applications.

## ✨ Features

- 🌍 **7 Languages**: English, German, Spanish, French, Japanese, Russian, Chinese
- ⚡ **Blazingly Fast**: Optimized with brotli compression and efficient data structures
- 🔐 **Password Generator**: CLI tool for generating secure, memorable passwords
- 🎯 **Flexible Filtering**: Filter by word length or starting character
- 📦 **Tiny Binary Size**: Feature flags minimize compiled size
- 🔬 **Strength Analysis**: Built-in entropy calculation for password strength

## 🚀 Quick Start

### As a Library

Add to your `Cargo.toml`:
```toml
[dependencies]
random_word = { version = "0.5.2", features = ["en"] }
```

Generate a random word:
```rust
use random_word::Lang;

fn main() {
    let word = random_word::get(Lang::En);
    println!("{}", word); // "adventure"
}
```

### As a CLI Tool

Install the password generator:
```bash
cargo install random_word
```

Generate a secure password:
```bash
$ passgen
correct-horse-battery-staple

$ passgen -s
correct-horse-battery-staple

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Password Strength Analysis
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Words used:          4
Dictionary size:     7776 words
Password length:     28 characters
Possible combos:     ~3.66e+15
Entropy:             51.70 bits
Strength rating:     Reasonable
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## 📖 Usage

### Library Examples

#### Generate a random word
```rust
use random_word::Lang;

let word = random_word::get(Lang::En);
```

#### Filter by starting character
```rust
let word = random_word::get_starts_with('c', Lang::En);
assert!(word.is_some());
// Could return: "computer", "cloud", "castle", etc.
```

#### Get all words of a specific length
```rust
let words = random_word::all_len(4, Lang::Fr);
assert!(!words.is_empty());
// Returns: ["chat", "pain", "lune", ...]
```

#### Get all words in a language
```rust
let all_words = random_word::all(Lang::En);
println!("Dictionary size: {}", all_words.len());
```

### CLI Password Generator

```bash
# Generate with default settings (4 words, hyphen separator)
passgen

# Custom number of words
passgen 6
# Output: orange-mountain-river-crystal-thunder-phone

# Custom separator
passgen 5 _
# Output: quick_brown_fox_jumps_over

# Show strength analysis
passgen -s
passgen --stats

# Combine options
passgen -s 6 _
# Output: word_word_word_word_word_word
# (followed by strength analysis)

# Get help
passgen --help
```

#### Password Strength Ratings

The `-s/--stats` flag provides detailed entropy analysis:

| Bits | Rating | Security Level |
|------|--------|----------------|
| <28 | Very Weak | Crackable instantly |
| 28-36 | Weak | Crackable in hours/days |
| 36-60 | Reasonable | Crackable in months/years |
| 60-80 | Strong | Secure for most purposes |
| 80-128 | Very Strong | Military grade |
| >128 | Extremely Strong | Overkill |

**Recommendation**: Use at least 4 words (51+ bits) for general purpose passwords, 6+ words (77+ bits) for high-security accounts.

## 🌍 Supported Languages

| Language | Feature Flag | Dictionary Size |
|----------|--------------|-----------------|
| 🇩🇪 German | `de` | ~7,700 words |
| 🇬🇧 English | `en` | ~7,700 words |
| 🇪🇸 Spanish | `es` | ~7,700 words |
| 🇫🇷 French | `fr` | ~7,700 words |
| 🇯🇵 Japanese | `ja` | ~7,700 words |
| 🇷🇺 Russian | `ru` | ~7,700 words |
| 🇨🇳 Chinese | `zh` | ~7,700 words |

**⚠️ Important**: You **must** enable at least one language feature to use this crate. This design choice keeps binary sizes minimal.

### Enabling Multiple Languages

```toml
[dependencies]
random_word = { version = "0.5.2", features = ["en", "es", "fr"] }
```

```rust
use random_word::Lang;

let english = random_word::get(Lang::En);
let spanish = random_word::get(Lang::Es);
let french = random_word::get(Lang::Fr);
```

## 🎯 Use Cases

### Password Generation
Generate memorable, secure passwords that are easy to type and remember:
```rust
let password: String = (0..4)
    .map(|_| random_word::get(Lang::En))
    .collect::<Vec<_>>()
    .join("-");
// "sunset-piano-garden-river"
```

### Test Data
Create realistic test data for your applications:
```rust
let username = random_word::get(Lang::En);
let project_name = format!("{}-{}",
    random_word::get(Lang::En),
    random_word::get(Lang::En)
);
```

### Placeholder Text
Generate placeholder content:
```rust
let words: Vec<&str> = (0..50)
    .map(|_| random_word::get(Lang::En))
    .collect();
let placeholder = words.join(" ");
```

### Multi-language Applications
Add linguistic variety:
```rust
use random_word::Lang;

let languages = [Lang::En, Lang::Es, Lang::Fr, Lang::De];
for lang in languages {
    println!("{}: {}",
        format!("{:?}", lang),
        random_word::get(lang)
    );
}
```

## ⚙️ How It Works

- **Compression**: Word lists are compressed with brotli and embedded in the binary
- **Lazy Decompression**: Words are only decompressed when first accessed
- **Efficient Storage**: Uses `ahash` for fast hashing and optimized data structures
- **Feature Flags**: Only include the languages you need

## 🔧 Advanced Configuration

### No Default Features
If you want complete control over which languages are included:

```toml
[dependencies]
random_word = { version = "0.5.2", default-features = false, features = ["es"] }
```

### Build from Source
```bash
git clone https://github.com/MitchellRhysHall/random_word
cd random_word
cargo build --release --features en
```

## 📊 Performance

Random word generation is extremely fast:
- **Cold start**: ~10µs (first word requires decompression)
- **Warm**: ~100ns per word (direct dictionary access)
- **Binary overhead**: ~50-100KB per language

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

- 🐛 Report bugs and issues
- 💡 Suggest new features or languages
- 📝 Improve documentation
- 🔧 Submit pull requests

### Adding a New Language

1. Add word list to `src/txt/<lang>.txt`
2. Update build script in `build.rs`
3. Add feature flag to `Cargo.toml`
4. Update documentation

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the [XKCD password strength comic](https://xkcd.com/936/)
- Word lists curated from various open-source dictionaries
- Built with [Rust](https://www.rust-lang.org/) 🦀

## 🔗 Links

- [Documentation](https://docs.rs/random_word)
- [Crates.io](https://crates.io/crates/random_word)
- [Repository](https://github.com/MitchellRhysHall/random_word)
- [Issue Tracker](https://github.com/MitchellRhysHall/random_word/issues)

---

<div align="center">

**Made with ❤️ and 🦀**

*Because `Tr0ub4dor&3` is harder to remember than `correct-horse-battery-staple`*

</div>
