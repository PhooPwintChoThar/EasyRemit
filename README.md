# Easy Remit
GitHub Link: https://github.com/PhooPwintChoThar/Easy-Remit

Easy-Remit is a Rust-based application featuring a multi-page GUI built with the Iced framework. It provides secure user management, account operations, and transaction processing, with SQLite for persistent data storage. The project employs AES encryption, hashing, and retry mechanisms, making it ideal for system-level applications where data security and performance are essential.

## Team
1. Phoo Pwint Cho Thar<67011755@kmitl.ac.th>
2. Thiri Thaw <67011731@kmitl.ac.th>

## Contribution
#### Phoo Pwint Cho Thar
Designed the GUI for home, login, and sign-up pages, implemented encryption, decryption, and hashing for security, and collaborated on SQLite database integration.
#### Thiri Thaw
Developed function, transfer, and success pages with backend transaction features and success notifications, and worked on secure integration with the SQLite database.

   
### db.rs
Manages SQLite database connections, CRUD operations, and error handling with retry logic.
### login.rs
Handles user login functionality, including password validation and hashing.
### signup.rs
Manages user signup process with Argon2 hashing for password security.
### home.rs
Main dashboard that provides access to account details, transaction history, and settings.
### transfer.rs
Manages encrypted transfers between accounts with AES encryption for secure data handling.
### success.rs
Displays success notifications for completed transactions.

## Key System-Level Concepts
### Encryption (AES)
Securely encrypts sensitive data before storing it in the SQLite database.
### Hashing (Argon2)
Passwords are hashed using Argon2 to ensure they are securely stored and cannot be easily reversed.
### Concurrency Handling
Uses Arc<Mutex<Connection>> to provide thread-safe database access, allowing multiple users to interact with the app concurrently.
### Retry Mechanism
Implements a retry logic to handle DatabaseBusy errors, enhancing reliability during high database load.
## Rust Concepts
## Structs 
Defined for modular data management, such as User, Account, and Transaction, to organize related fields.
## Enums
Used to represent various states within the app, such as user actions (Login, Signup, Transfer) and account statuses.
## Pass by Reference (&)
Utilized throughout the codebase for efficient memory usage, particularly in handling large structures or data-sensitive operations.
## Pattern Matching
Employed to streamline logic and ensure safe handling of various states and user inputs.
### Security Features
## AES Encryption
Secures sensitive data in storage and during transfers.
## Argon2 Hashing
Ensures user passwords are stored in a way that is resistant to brute-force attacks.
## Database Concurrency Control
SQLite database access is synchronized to prevent conflicts during concurrent transactions.
## Retry Logic for Database Access
Automatic retry for database interactions during high load or contention, ensuring stability.

### Usage
## User Authentication
Secure login and signup with password hashing and validation.
## Account Operations
Accessible from the main dashboard, enabling deposits, withdrawals, and transfers.
## Transaction Processing
Allows secure and encrypted data transfers between accounts.
