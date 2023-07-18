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
