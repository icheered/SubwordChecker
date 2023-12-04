use std::{time::Instant};


struct Node {
    character: char,
    is_word: bool,
    children: Vec<Box<Node>>,
}

#[allow(dead_code)]
fn print_tree(node: &Node, level: usize) {
    let indent = "  ".repeat(level);
    println!("{}{}", indent, node.character);
    for child in &node.children {
        print_tree(child, level + 1);
    }
}

fn generate_combinations(word: &str) -> Vec<String> {
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    let mut combinations = Vec::new();

    for i in 0..(1 << n) {
        let mut combination = String::new();
        for j in 0..n {
            if i & (1 << j) != 0 {
                combination.push(chars[j]);
            }
        }
        if !combination.is_empty() {
            combinations.push(combination);
        }
    }

    combinations
}

fn construct_tree(input: &str) -> Node {
    let mut root = Node {
        character: ' ',
        is_word: false,
        children: Vec::new(),
    };

    for line in input.lines() {
        let mut current_node = &mut root;
        for character in line.chars() {
            let child_index = current_node.children.iter().position(|n| n.character == character);

            match child_index {
                Some(index) => current_node = current_node.children[index].as_mut(),
                None => {
                    let new_node = Node {
                        character,
                        is_word: false,
                        children: Vec::new(),
                    };
                    current_node.children.push(Box::new(new_node));
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        }
        current_node.is_word = true;
    }

    root
}

fn get_valid_words(tree: &Node, combinations: Vec<String>) -> Vec<String> {
    let mut valid_words = Vec::new();
    for combination in combinations {
        let mut current_node = tree;
        let mut is_valid_word = true;
        for character in combination.chars() {
            match current_node.children.iter().find(|n| n.character == character) {
                Some(node) => current_node = node,
                None => {
                    is_valid_word = false;
                    break;
                }
            }
        }
        if is_valid_word && current_node.is_word && !valid_words.contains(&combination) {
            valid_words.push(combination);
        }
    }
    valid_words
}

// This is just for fun
use indicatif::{ProgressBar, ProgressStyle};
fn slow_method(input: &str, combinations: Vec<String>) -> Vec<String> {
    let mut valid_words = Vec::new();

    // Create a new progress bar
    let bar = ProgressBar::new(combinations.len() as u64);

    // Set the style of the progress bar
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap_or_else(|e| {
            eprintln!("Error setting progress bar style: {}", e);
            ProgressStyle::default_bar()
        });

    bar.set_style(style);

    for combination in combinations {
        for line in input.lines() {
            if line == combination {
                valid_words.push(line.to_string());
                break;
            }
        }
        bar.inc(1); // Increment the progress bar after processing each combination
    }

    bar.finish_with_message("Processing complete");
    valid_words
}


pub fn analyze(input: &str, word: &str) {
    let t1 = Instant::now();

    let tree = construct_tree(input);
    let t2 = Instant::now();
    println!("Tree constructed in: {:?}", t2 - t1);

    let combinations = generate_combinations(word);
    let t3 = Instant::now();
    println!("Combinations generated in: {:?}", t3 - t2);
    println!("\tCombinations length: {:?}", combinations.len());

    let mut valid_words = get_valid_words(&tree, combinations);
    // let valid_words = slow_method(input, combinations);
    let t4 = Instant::now();
    println!("Valid words found in: {:?}", t4 - t3);
    println!("\tValid words length: {:?}", valid_words.len());

    // Write to file, sort by longest word first
    let mut output = String::new();
    valid_words.sort_by(|a, b| b.len().cmp(&a.len()));
    for word in valid_words {
        output.push_str(&format!("{}\n", word));
    }
    std::fs::write("output.txt", output).unwrap();
}