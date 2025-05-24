# Password Hasher Tool - LDAP Compatible

A modern, secure, and user-friendly web application for password hashing and comparison, built with Rust and Actix-web. This tool provides a robust solution for generating and verifying password hashes using various LDAP-compatible algorithms.

## Features

- **Multiple Hash Algorithms**: Support for MD5, SHA1, SHA256, and SHA512 hashing algorithms
- **Interactive Web Interface**: Clean, responsive UI built with Tailwind CSS
- **Real-time Password Comparison**: Instantly compare hashed passwords with plain text
- **Password Visibility Toggle**: Secure password input with optional visibility
- **History Management**: 
  - Automatic saving of hash operations
  - Downloadable history in text format
  - Clear history functionality
  - Last 10 operations stored locally
- **Security Features**:
  - Client-side password handling
  - Secure hash generation
  - Salt support for enhanced security
  - LDAP-compatible output formats

## Technical Stack

- **Backend**: 
  - Rust programming language
  - Actix-web framework
  - Rust-crypto for secure hashing
- **Frontend**:
  - HTML5/CSS3
  - Tailwind CSS for styling
  - Vanilla JavaScript
  - Font Awesome icons
- **Storage**:
  - Browser localStorage for history
  - No server-side password storage

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Web browser with JavaScript enabled

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/password-hasher.git
cd password-hasher
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run --release
```

4. Access the application at `http://localhost:8080`

## Usage

1. Enter your password in the input field
2. (Optional) Add a salt for additional security
3. Select your preferred hashing algorithm
4. Click "Hash It" to generate the hash
5. Use "Compare" to verify against existing hashes
6. Download or clear history as needed

## Security Considerations

- Passwords are never stored on the server
- All hashing operations are performed server-side
- History is stored locally in the browser
- No sensitive data is transmitted over the network
- Salt support for enhanced security

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Actix-web team for the excellent web framework
- Tailwind CSS for the styling framework
- Font Awesome for the icon set 