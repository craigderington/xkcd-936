use random_word::Lang;
use std::env;

fn print_usage(program: &str) {
    eprintln!("Usage: {} [OPTIONS] [num_words] [separator]", program);
    eprintln!("\nOptions:");
    eprintln!("  -s, --stats    Show password strength statistics");
    eprintln!("  -h, --help     Show this help message");
    eprintln!("\nArguments:");
    eprintln!("  num_words      Number of words to generate (default: 4)");
    eprintln!("  separator      Character to separate words (default: -)");
    eprintln!("\nExamples:");
    eprintln!("  {}              # Generate 4 words with hyphens", program);
    eprintln!("  {} 5 _          # Generate 5 words with underscores", program);
    eprintln!("  {} -s 6         # Generate 6 words and show stats", program);
}

fn calculate_entropy(num_words: usize, dictionary_size: usize) -> f64 {
    // Entropy = log2(combinations) = log2(dictionary_size^num_words)
    // = num_words * log2(dictionary_size)
    (num_words as f64) * (dictionary_size as f64).log2()
}

// ANSI color codes
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";

fn get_strength_rating(entropy: f64) -> (&'static str, &'static str) {
    // NIST guidelines and general security recommendations
    // Returns (rating, color)
    match entropy {
        e if e < 28.0 => ("Very Weak", RED),
        e if e < 36.0 => ("Weak", YELLOW),
        e if e < 60.0 => ("Reasonable", BLUE),
        e if e < 80.0 => ("Strong", GREEN),
        e if e < 128.0 => ("Very Strong", CYAN),
        _ => ("Extremely Strong", MAGENTA),
    }
}

fn print_stats(num_words: usize, dictionary_size: usize, entropy: f64, password_len: usize) {
    let combinations = format!("~{:.2e}", (dictionary_size as f64).powi(num_words as i32));
    let (strength, strength_color) = get_strength_rating(entropy);

    eprintln!("\n{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", BOLD, RESET);
    eprintln!("{}Password Strength Analysis{}", BOLD, RESET);
    eprintln!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", BOLD, RESET);
    eprintln!("Words used:          {}{}{}", BOLD, num_words, RESET);
    eprintln!("Dictionary size:     {}{}{} words", BOLD, dictionary_size, RESET);
    eprintln!("Password length:     {}{}{} characters", BOLD, password_len, RESET);
    eprintln!("Possible combos:     {}{}{}", BOLD, combinations, RESET);
    eprintln!("Entropy:             {}{}{:.2} bits{}", BOLD, strength_color, entropy, RESET);
    eprintln!("Strength rating:     {}{}{}{}", BOLD, strength_color, strength, RESET);
    eprintln!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", BOLD, RESET);
    eprintln!("\n{}For reference:", DIM);
    eprintln!("  • <28 bits:    {}Very Weak{} (crackable instantly)", RED, DIM);
    eprintln!("  • 28-36 bits:  {}Weak{} (crackable in hours/days)", YELLOW, DIM);
    eprintln!("  • 36-60 bits:  {}Reasonable{} (crackable in months/years)", BLUE, DIM);
    eprintln!("  • 60-80 bits:  {}Strong{} (secure for most purposes)", GREEN, DIM);
    eprintln!("  • 80-128 bits: {}Very Strong{} (military grade)", CYAN, DIM);
    eprintln!("  • >128 bits:   {}Extremely Strong{} (overkill){}\n", MAGENTA, DIM, RESET);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse flags
    let mut show_stats = false;
    let mut positional_args = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-s" | "--stats" => show_stats = true,
            "-h" | "--help" => {
                print_usage(&args[0]);
                return;
            }
            _ => positional_args.push(arg.clone()),
        }
    }

    // Parse positional arguments
    let num_words = if !positional_args.is_empty() {
        positional_args[0].parse::<usize>().unwrap_or_else(|_| {
            print_usage(&args[0]);
            std::process::exit(1);
        })
    } else {
        4
    };

    let separator = if positional_args.len() > 1 {
        &positional_args[1]
    } else {
        "-"
    };

    // Validate num_words
    if num_words == 0 {
        eprintln!("Error: num_words must be at least 1");
        std::process::exit(1);
    }

    // Get dictionary size
    let dictionary_size = random_word::all(Lang::En).len();

    // Generate random words
    let words: Vec<&str> = (0..num_words)
        .map(|_| random_word::get(Lang::En))
        .collect();

    // Join them with the separator
    let password = words.join(separator);

    // Print the password
    println!("{}", password);

    // Show statistics if requested
    if show_stats {
        let entropy = calculate_entropy(num_words, dictionary_size);
        print_stats(num_words, dictionary_size, entropy, password.len());
    }
}
