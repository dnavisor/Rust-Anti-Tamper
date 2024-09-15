 rust-antitamper

`Rust-Anti-Tamper` is a Rust program designed to enhance the security of applications by checking if a DLL loaded into the program is signed. If the DLL is not properly signed, the program will terminate. Otherwise, it will continue executing.

## Features

- **DLL Signature Verification**: Ensures that the loaded DLL is properly signed to prevent unauthorized modifications.
- **Automatic Termination**: Terminates the program if the DLL does not meet the signature requirements.
- **Continued Execution**: Allows the program to proceed if the DLL is verified as signed.

## Requirements

- Rust 1.60 or higher
- Windows operating system (DLL signature verification is specific to Windows)