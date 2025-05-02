use std::io;

fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    match largest_and_smallest(&input) {
        Ok((largest, smallest)) => {
            println!("{largest} is largest and {smallest} is smallest");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn largest_and_smallest(input: &[i32]) -> Result<(i32, i32), io::Error> {
    if input.len() <= 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Input array is empty",
        ));
    }
    let mut largest = input[0];
    let mut smallest = input[0];

    for val in input {
        if *val > largest {
            largest = *val
        }

        if *val < smallest {
            smallest = *val
        }
    }

    Ok((largest, smallest))
}
