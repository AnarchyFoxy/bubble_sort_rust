his is recruitment task I've received in C++, but in my free time I decided, to rewrite it in Rust, so here is task content:

Let S = ‘s1s2...sa’ and T = ‘t1t2...tb’ be strings of length a and b, respectively (we call si the ith letter of S). We say that S is lexicographically less than T, denoting S <lex T, if either

    a < b and si = ti for all i = 1, 2, ..., a, or
    There exists an index i ≤ min {a, b} such that sj = tj for all j = 1, 2, ..., i − 1 and si < ti.

Lexicographic sorting algorithm aims to sort a given set of n strings into lexicographically ascending order (in case of ties due to identical strings, then in non-descending order).

The length of each string in the input array is limited to 100'000 characters.

Each string contains lowercase Latin characters only

Function signature:
void contains(std::vector<std::string>& strings);

    Write the solution. Output result to the console.
    Check that your algorithm is correct.
    In comments, explain run time and space complexity for your algorithm.

Note: don't use std sort functions.

Example test cases:
Input: ["hello", "world", "apple", "banana", "cat", "dog"]
Output: apple banana cat dog hello world

Input: ["zebra", "apple", "banana", "cat", "dog"]
Output: apple banana cat dog zebra

Input: ["cat", "dog", "mouse", "elephant", "tiger"]
Output: cat dog elephant mouse tiger

Input: ["abcd", "abc", "abcde", "ab", "abcdef"]
Output: ab abc abcd abcde abcdef

Input: ["aaa", "aa", "aaaaa", "a", "aaaa"]
Output: a aa aaa aaaa aaaaa
