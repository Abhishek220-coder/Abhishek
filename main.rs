use std::cmp;
use std::collections::HashSet;

// 1. Check if a given string is a palindrome
fn is_palindrome(s: &str) -> bool {
    let cleaned_s: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    cleaned_s == cleaned_s.chars().rev().collect::<String>()
}

// 2. Return the index of the first occurrence of a given number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// 3. Return the shortest word in a string
fn shortest_word(s: &str) -> &str {
    s.split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
}

// 4. Check if a given number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 5. Return the median of a sorted array
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// 6. Find the longest common prefix of a set of strings
fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].to_string();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return prefix;
            }
        }
    }
    prefix
}

// 7. Return the kth smallest element in an array
// fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
//     arr.sort();
//     arr.get(k - 1).copied()
// }

// 8. Return the maximum depth of a binary tree
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => 1 + cmp::max(max_depth(node.left), max_depth(node.right)),
        None => 0,
    }
}

// 9. Reverse a string in Rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 10. Check if a number is prime in Rust
fn check_prime(n: u32) -> bool {
    is_prime(n)
}

// 11. Merge two sorted arrays in Rust
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

// 12. Find the maximum subarray sum in Rust
fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for &num in &nums[1..] {
        current_sum = cmp::max(num, current_sum + num);
        max_sum = cmp::max(max_sum, current_sum);
    }

    max_sum
}

// Main function to test all the above functions
fn main() {
    // Test palindrome function
    println!(
        "Is 'A man, a plan, a canal, Panama' a palindrome? {}",
        is_palindrome("A man, a plan, a canal, Panama")
    );

    // Test first occurrence function
    let sorted_array = [1, 2, 2, 3, 4, 5];
    println!(
        "First occurrence of 2 in {:?} is at index {:?}",
        sorted_array,
        first_occurrence(&sorted_array, 2)
    );

    // Test shortest word function
    let text = "The quick brown fox jumps over the lazy dog";
    println!(
        "The shortest word in '{}' is '{}'",
        text,
        shortest_word(text)
    );

    // Test prime check function
    println!("Is 29 a prime number? {}", is_prime(29));

    // Test median function
    let sorted_array_for_median = [1, 2, 3, 4, 5];
    println!(
        "Median of {:?} is {}",
        sorted_array_for_median,
        median(&sorted_array_for_median)
    );

    // Test longest common prefix function
    let words = ["flower", "flow", "flight"];
    println!(
        "Longest common prefix of {:?} is '{}'",
        words,
        longest_common_prefix(&words)
    );

    // Test kth smallest element function
    let mut array_for_kth_smallest = [3, 2, 1, 5, 6, 4];
    // println!("The 2nd smallest element in {:?} is {:?}", array_for_kth_smallest, kth_smallest(&mut array_for_kth_smallest, 2));

    // Test maximum depth of binary tree function
    let tree = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })),
    }));
    println!("Maximum depth of the binary tree is {}", max_depth(tree));

    // Test reverse string function
    let string_to_reverse = "Hello, world!";
    println!(
        "Reversed string of '{}' is '{}'",
        string_to_reverse,
        reverse_string(string_to_reverse)
    );

    // Test prime check in Rust function
    println!("Is 17 a prime number? {}", check_prime(17));

    // Test merge sorted arrays function
    let array1 = [1, 3, 5, 7];
    let array2 = [2, 4, 6, 8];
    println!(
        "Merged array of {:?} and {:?} is {:?}",
        array1,
        array2,
        merge_sorted_arrays(&array1, &array2)
    );

    // Test maximum subarray sum function
    let array_for_max_subarray_sum = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!(
        "Maximum subarray sum of {:?} is {}",
        array_for_max_subarray_sum,
        max_subarray_sum(&array_for_max_subarray_sum)
    );
}
