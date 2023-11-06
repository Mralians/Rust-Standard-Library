use std::vec;

fn main() {
    ///////////////////////////////////////////////////////////////
    // Create a vector with some elements
    ///////////////////////////////////////////////////////////////
    let fruits = vec!["apple", "orange", "tomato"];
    println!("fruits: {fruits:?}");

    ///////////////////////////////////////////////////////////////
    // Create an empty vector and fill it
    ///////////////////////////////////////////////////////////////
    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("orange");
    fruits.push("tomato");
    println!("fruits: {fruits:?}");

    ///////////////////////////////////////////////////////////////
    // Remove the last element
    ///////////////////////////////////////////////////////////////
    let last = fruits.pop();
    if let Some(last) = last {
        println!("Removed '{last}' from {fruits:?}");
    }

    ///////////////////////////////////////////////////////////////
    // Insert an  element into the middle of the vector
    ///////////////////////////////////////////////////////////////
    fruits.insert(1, "grape");
    println!("fruits after insertion: {fruits:?}");

    ///////////////////////////////////////////////////////////////
    // Swap to element
    ///////////////////////////////////////////////////////////////
    fruits.swap(1, 0);
    println!("fruits after swap: {fruits:?}");

    ///////////////////////////////////////////////////////////////
    // Access the first and last elements
    ///////////////////////////////////////////////////////////////
    let first = fruits.first();
    if let Some(first) = first {
        println!("The first fruit is {first}");
    }
    let last = fruits.last();
    if let Some(last) = last {
        println!("The last fruit is {last}");
    }

    ///////////////////////////////////////////////////////////////
    // Access arbitrary elements
    ///////////////////////////////////////////////////////////////
    let second = fruits.get(1);
    if let Some(second) = second {
        println!("Second fruit: {second}");
    }
    let first_second = fruits.get(0..=1);
    if let Some(first_second) = first_second {
        println!("First and Second fruit: {first_second:?}");
    }

    ///////////////////////////////////////////////////////////////
    // Access the first element
    ///////////////////////////////////////////////////////////////
    let first = if let [first, ..] = fruits.as_slice() {
        Some(first)
    } else {
        None
    };
    println!("First fruit: {first:?}");
    ///////////////////////////////////////////////////////////////
    // Split first element
    ///////////////////////////////////////////////////////////////
    let split_first = if let [first, tail @ ..] = fruits.as_slice() {
        Some((first, tail))
    } else {
        None
    };
    println!(
        "First fruit: {} and rest: {:?}",
        split_first.unwrap().0,
        split_first.unwrap().1
    );
    let fruits_slice = fruits.as_slice();
    let fruits_slice = fruits_slice as *const [&str] as *const &str; // or fruits_slice.as_ptr();
    println!("{fruits_slice:?}");
    ///////////////////////////////////////////////////////////////
    // Access arbitrary elements without bonds checking
    ///////////////////////////////////////////////////////////////
    let second = fruits[1];
    println!("Second fruit: {second}");
    ///////////////////////////////////////////////////////////////
    // Remove some item and shift all that come after into place
    ///////////////////////////////////////////////////////////////
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.remove(1);
    println!("Removed {second_num} from {nums:?}");

    ///////////////////////////////////////////////////////////////
    //Filter the vector in place
    ///////////////////////////////////////////////////////////////
    let mut names = vec!["Aaron", "Felicia", "Alex", "Daniel"];
    names.retain(|name| name.starts_with('A'));
    println!("Names starting A: {names:?}");

    ///////////////////////////////////////////////////////////////
    //Check if the vector contains an elements
    ///////////////////////////////////////////////////////////////
    let names = vec!["Aaron", "Felicia", "Alex", "Daniel"];
    println!(
        "Does 'names' contain \"Alex\"? {} ",
        names.contains(&"Alex")
    );
    ///////////////////////////////////////////////////////////////
    //Remove consecutive(!) duplicates
    ///////////////////////////////////////////////////////////////
    let mut nums = vec![1, 2, 2, 3, 4, 4, 4, 5];
    nums.dedup();
    println!("Deduped, pre-sorted nums: {nums:?}");

    ///////////////////////////////////////////////////////////////
    // Be careful if your data is not sorted!
    ///////////////////////////////////////////////////////////////
    let mut nums = vec![2, 1, 4, 2, 4, 5, 1, 2];
    nums.dedup();
    println!("Deduped, unsorted nums: {nums:?}");

    ///////////////////////////////////////////////////////////////
    // Sort a vector
    ///////////////////////////////////////////////////////////////
    let mut nums = vec![2, 1, 4, 2, 4, 5, 1, 2];
    nums.sort();
    println!("Manually sorted nums: {nums:?}");
    nums.dedup();
    println!("Deduped, pre-sorted nums: {nums:?}");
    ///////////////////////////////////////////////////////////////
    // Reverse a vector
    ///////////////////////////////////////////////////////////////
    let mut nums = vec![2, 1, 4, 2, 4, 5, 1, 2];
    nums.sort();
    nums.reverse();
    println!("nums after being reversed: {nums:?}");

    ///////////////////////////////////////////////////////////////
    // Create a consuming iterator over a range
    ///////////////////////////////////////////////////////////////
    let mut alphabet = vec!['a', 'b', 'c'];
    print!("The first two letters of the alphabet are: ");
    for letter in alphabet.drain(..2) {
        print!("{letter} ");
    }
    println!();
    ///////////////////////////////////////////////////////////////
    // Check if a vector is empty
    ///////////////////////////////////////////////////////////////
    let mut alphabet = vec!['a', 'b', 'c'];
    println!("Is the fridge empty?  {}", alphabet.is_empty());
    ///////////////////////////////////////////////////////////////
    // Remove all elements
    ///////////////////////////////////////////////////////////////
    alphabet.clear();
    println!("Is the alphabet now empty? {} ", alphabet.is_empty());
    ///////////////////////////////////////////////////////////////
    // Split a vector into two pieces
    ///////////////////////////////////////////////////////////////
    let mut colors = vec!["red", "green", "blue", "yellow"];
    println!("colors before splitting: {colors:?}");
    let index = colors.len() / 2;
    let mut second_half = colors.split_off(index);
    println!("color before splitting: {colors:?}");
    println!("second_half: {second_half:?}");
    ///////////////////////////////////////////////////////////////
    // Put two vector together
    ///////////////////////////////////////////////////////////////
    colors.append(&mut second_half);
    println!("colors after appending: {colors:?}");
    println!("second_half after appending: {second_half:?}");

    ///////////////////////////////////////////////////////////////
    //splite
    ///////////////////////////////////////////////////////////////
    let mut stuff = vec!["1", "2", "3", "4", "5"];
    println!("Original stuff: {:?}", stuff);
    let stuff_to_insert = vec!["a", "b", "c"];
    let removed_stuff: Vec<_> = stuff.splice(0..3, stuff_to_insert).collect();
    println!("Splited stuff: {stuff:?}");
    println!("Removed stuff: {removed_stuff:?}");
    ///////////////////////////////////////////////////////////////
    // If your are working with very big dataset, you can optimize the performance of your vector
    ///////////////////////////////////////////////////////////////
    // Initialize the vector with a certain capacity
    let mut large_vec: Vec<i32> = Vec::with_capacity(1_000_000);
    println!("large_vec after creation:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());
    ///////////////////////////////////////////////////////////////
    // Shrink the vector as close as possible to its length
    ///////////////////////////////////////////////////////////////
    large_vec.append(&mut [23, 21, 25].to_vec());
    large_vec.shrink_to_fit();
    println!("large_vec after shrink_to_fit");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());
    // Remove some items, replacing it with the last
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.swap_remove(1);
    println!("Second number is: {second_num}");
    println!("nums after swap_remove: {nums:?}");
}
