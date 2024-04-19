## R-Calculator

R-Calculator is a simple command-line calculator implemented in Rust. It allows users to perform basic arithmetic operations such as addition, subtraction, multiplication, division, and modulus. The calculator also validates user input to ensure it contains only valid characters (numbers and operators), handles consecutive operator errors, and gracefully handles division by zero errors.

### Features

- **Flexible Expression Input:** Users can input arithmetic expressions in any format they prefer, including spaces between numbers and operators.
- **Basic Arithmetic Operations:** Perform addition, subtraction, multiplication, division, and modulus operations.
- **Input Validation:** Validate user input to ensure it contains only valid characters (numbers and operators).
- **Consecutive Operator Handling:** Gracefully handle errors when consecutive operators are encountered in the input expression.
- **User-Friendly Interface:** Provides a clear and intuitive interface with prompts and error messages for ease of use.
- **Error Handling:** Detect and handle division by zero errors, providing informative error messages to the user.
- **Continuation Option:** After calculating the result, gives the user the option to continue using the calculator or exit.

### How to Use

1. Clone the repository to your local machine.
2. Navigate to the project directory in your terminal.
3. Run the `cargo run` command to compile and execute the program.
4. Follow the prompts to input an arithmetic expression and view the result.

### Example

```
**************                             Welcome to R- Calculator!! (Made by RUST)                             **************
Hi, you are now in our monitor

Please input an arithmetic expression:
5+10
Result: 15.0
Do you want to continue? (yes/no)
yes
Please input an arithmetic expression:
-20*4
Result: -80.0
Do you want to continue? (yes/no)
no
||         Thanks for using R-Calculator  :)          ||
```

### Contributions

Contributions are welcome! If you have any suggestions or want to report a bug, please open an issue or submit a pull request.