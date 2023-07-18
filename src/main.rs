/*
fn contains(strings: &mut Vec<String>): This line defines a function named contains that takes a
mutable reference to a vector of String (&mut Vec<String>) as a parameter.
The function will sort the strings in lexicographic order and modify the original
vector since it's passed by mutable reference.

let n = strings.len();: This line creates a variable n and
assigns it the length of the input vector strings.
This n variable will be used in the subsequent loops.
*/

fn contains(strings: &mut Vec<String>) {
    let n = strings.len();

/*
This is the implementation of the Bubble Sort algorithm within the contains function.
for i in 0..n - 1 {: This loop iterates from 0 to n - 1.
It represents the number of passes to perform the Bubble Sort algorithm
for j in 0..n - i - 1 {: This nested loop iterates from 0 to n - i - 1.
It represents the elements to be compared in each pass.
The - i ensures that the last i elements are already sorted and do not need to be compared again.
Inside the nested loop, adjacent strings at indices j and j + 1 are compared
using the relational operator >. If the left string (strings[j]) is lexicographically
greater than the right string (strings[j + 1]),
the two strings are swapped using strings.swap(j, j + 1);.
This process continues until the entire vector is sorted in lexicographic order.
*/
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            // Compare adjacent strings
            if strings[j] > strings[j + 1] {
                // Swap if the strings are in the wrong order
                strings.swap(j, j + 1);
            }
        }
    }
}

/*
In the main function, several test cases are defined
using different vectors of string literals (test1, test2, ..., test5).

The first test case (test1) is created by using the vec! macro to create a vector of
string literals, converting them to String using the .to_string() method,
and collecting them into a Vec<String>.

The contains function is called for each test case, sorting the strings lexicographically.

After sorting, each string in the vector is printed using a
range-based for loop (for str in &input1 { print!("{} ", str); }), separated by spaces.

A newline is printed after printing the strings of each test case.
*/
fn main() {
    let mut test1 = vec!["hello", "world", "apple", "banana", "cat", "dog"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contains(&mut test1);
    for str in &test1 {
        print!("{} ", str);
    }
    println!(); // Output: apple banana cat dog hello world

    let mut test2 = vec!["zebra", "apple", "banana", "cat", "dog"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contains(&mut test2);
    for str in &test2 {
        print!("{} ", str);
    }
    println!(); // Output: apple banana cat dog zebra

    let mut test3 = vec!["cat", "dog", "mouse", "elephant", "tiger"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contains(&mut test3);
    for str in &test3 {
        print!("{} ", str);
    }
    println!(); // Output: cat dog elephant mouse tiger

    let mut test4 = vec!["abcd", "abc", "abcde", "ab", "abcdef"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contains(&mut test4);
    for str in &test4 {
        print!("{} ", str);
    }
    println!(); // Output: ab abc abcd abcde abcdef

    let mut test5 = vec!["aaa", "aa", "aaaaa", "a", "aaaa"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contains(&mut test5);
    for str in &test5 {
        print!("{} ", str);
    }
    println!(); // Output: a aa aaa aaaa aaaaa
}
