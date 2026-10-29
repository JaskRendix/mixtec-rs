use clap::{Parser, Subcommand};
use mixteco_corpus::{
    concordance, contexts, export_contexts, export_kwic, export_vocab, kwic, stats, top_words,
    vocabulary_stats, word_frequencies,
};
use mixteco_dictionary::translation_candidates;
use mixteco_orthography::normalize;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Normalize {
        text: String,
    },

    Search {
        text: String,
    },

    Ipa {
        text: String,
    },

    Stats {
        file: String,
    },

    Freq {
        file: String,
    },

    Concordance {
        file: String,
        query: String,
    },

    Vocab {
        file: String,
    },

    Top {
        file: String,

        #[arg(short, long, default_value_t = 25)]
        limit: usize,
    },

    Kwic {
        file: String,
        query: String,

        #[arg(short, long, default_value_t = 5)]
        width: usize,
    },

    Contexts {
        file: String,
        query: String,
    },

    ExportContexts {
        file: String,
        query: String,
        output: String,
    },

    ExportKwic {
        file: String,
        query: String,
        output: String,

        #[arg(short, long, default_value_t = 5)]
        width: usize,
    },

    ExportVocab {
        file: String,
        output: String,
    },

    Lexicon {
        file: String,
        word: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Normalize { text } => {
            println!("{}", normalize(&text));
        }

        Commands::Search { text } => {
            println!("{}", text.replace('ꞌ', ""));
        }

        Commands::Ipa { text } => {
            println!("{}", text.replace('ꞌ', "ʔ"));
        }

        Commands::Stats { file } => match stats(file) {
            Ok(stats) => {
                println!("Sentence pairs: {}", stats.pairs);
                println!("Source words : {}", stats.source_words);
                println!("Target words : {}", stats.target_words);
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::Freq { file } => match word_frequencies(file) {
            Ok(freq) => {
                let mut items: Vec<_> = freq.into_iter().collect();

                items.sort_by_key(|b| std::cmp::Reverse(b.1));

                for (word, count) in items.iter().take(25) {
                    println!("{:<20} {}", word, count);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::Concordance { file, query } => match concordance(file, &query) {
            Ok(results) => {
                println!("Matches: {}", results.len());

                for (i, hit) in results.iter().take(20).enumerate() {
                    println!("\n[{}]", i + 1);
                    println!("XTN: {}", hit.source);
                    println!("ES : {}", hit.target);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::Vocab { file } => match vocabulary_stats(file) {
            Ok(stats) => {
                println!("Vocabulary Statistics");
                println!();

                println!("Unique words         : {}", stats.unique_words);
                println!("Words occurring once : {}", stats.hapax_words);
                println!("Hapax rate           : {:.2}%", stats.hapax_rate);

                println!();
                println!("Most common word     : {}", stats.most_common_word);
                println!("Frequency            : {}", stats.most_common_count);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        },

        Commands::Top { file, limit } => match top_words(file, limit) {
            Ok(words) => {
                println!("Top {} Words", limit);
                println!();

                for (i, (word, count)) in words.iter().enumerate() {
                    println!("{:>2}. {:<20} {}", i + 1, word, count);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::Kwic { file, query, width } => match kwic(file, &query, width) {
            Ok(results) => {
                println!("Matches: {}", results.len());
                println!();

                for (i, hit) in results.iter().take(20).enumerate() {
                    println!("[{}]", i + 1);
                    println!("... {} [{}] {} ...", hit.left, hit.keyword, hit.right);
                    println!("ES: {}", hit.translation);
                    println!();
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::Contexts { file, query } => match contexts(file, &query) {
            Ok(stats) => {
                println!("Left contexts");
                println!();

                for (word, count) in stats.left.iter().take(10) {
                    println!("{:<20} {}", word, count);
                }

                println!();
                println!("Right contexts");
                println!();

                for (word, count) in stats.right.iter().take(10) {
                    println!("{:<20} {}", word, count);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::ExportContexts {
            file,
            query,
            output,
        } => match export_contexts(file, &query, &output) {
            Ok(()) => {
                println!("Exported contexts to {}", output);
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::ExportKwic {
            file,
            query,
            output,
            width,
        } => match export_kwic(file, &query, &output, width) {
            Ok(()) => {
                println!("Exported KWIC to {}", output);
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::ExportVocab { file, output } => match export_vocab(file, output.clone()) {
            Ok(()) => {
                println!("Exported vocabulary to {}", output);
            }
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::Lexicon { file, word } => match translation_candidates(&file, &word) {
            Ok(candidates) => {
                println!("Translation candidates for: {}", word);
                println!();

                for candidate in candidates.iter().take(25) {
                    println!("{:<20} {}", candidate.spanish, candidate.score);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },
    }
}
