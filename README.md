# Balanced Base

![MIT License](https://img.shields.io/badge/license-MIT-blue)

After watching this [Youtube Video](https://youtu.be/RcQ218t8ZO0?si=YR70-XQpBoHe0F_N) I became interested in the idea of balanced bases and decided to implement a library to do simple conversions and arithmetic with them.

Since the actual implementations of these are obviously just structs for wrapping binary arithmetic there is no practical use for these and working with them is slower and more cumbersome than working with binary. However the main reason to do this was an exercise to teach myself Rust.

If you want to play around with these feel free. It may not be useful but I think they'e still kind of fun.

## Features

- Implementations of 5, 10, 20 and 40 balanced trit numbers. These were chosen since they can be entirely contained within signed 8, 16, 32 and 64 bit numbers.
- Parsing from strings and display methods for balanced ternary.
- Conversion to and from the similarly sized signed-binary types.
- Addition, subtraction, multiplication and negation for balanced ternary.
- Hopefully useful enough errors to identify problems.

## Example

    // T01 * 1T -> -8 * 2 = -16 -> -27 + 9 + 3 -1 -> T11T
    let result = "T01".parse::<T5>().unwrap() * "1T".parse::<T5>().unwrap();
    println!("Result is {}", result);

## Acknowledgements

[Cadaeic Studios](https://www.youtube.com/@cadaeicstudios) who is clearly just at the start of a bright Youtube career.

## Authors

- [@WillJBrown](https://www.github.com/WillJBrown)

## License

[MIT](https://choosealicense.com/licenses/mit/)