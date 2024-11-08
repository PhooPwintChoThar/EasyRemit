# Easy Remit
GitHub Link: https://github.com/PhooPwintChoThar/Easy-Remit

Easy-Remit is a Rust-based application featuring a multi-page GUI built with the Iced framework. It provides secure user management, account operations, and transaction processing, with SQLite for persistent data storage. The project employs AES encryption, hashing, and retry mechanisms, making it ideal for system-level applications where data security and performance are essential.

## Team
1. Phoo Pwint Cho Thar<67011755@kmitl.ac.th>
2. Thiri Thaw <67011731@kmitl.ac.th>

## Contribution
#### Phoo Pwint Cho Thar - Designed the GUI for main, home, login, and sign-up pages, implemented encryption, decryption, and hashing for security, and collaborated on SQLite database integration.
#### Thiri Thaw - Developed db, function, transfer, and success pages with backend transaction features and success notifications, and worked on secure integration with the SQLite database.
### main.rs
Purpose: Entry point of the application, initializing and running the GUI.
Key Features:
Initializes the Iced GUI and sets up the main application structure.
Coordinates transitions between pages (Login, Home, Transfer, Success).
Handles global error management and state initialization.
### home.rs
Purpose: Main dashboard for displaying user account details.
Key Features:
Shows account balance, recent transactions, and basic user information.
Provides access to other actions such as fund transfers and account settings.
Serves as the main hub after a successful login.
### signup.rs
Purpose: Manages the user registration process.
Key Features:
Collects new user details, including username and password.
Hashes passwords with Argon2 for secure storage.
### login.rs
Purpose: Handles user login functionality, including password validation.
Key Features:
Collects user login credentials (username and password).
Verifies passwords by comparing hashed input with stored hashes in the database.
Provides feedback on login success or failure, allowing secure user access.
Adds new users to the database, ensuring unique usernames and secure password storage.
### db.rs
Purpose: Manages SQLite database connections and CRUD operations.
Key Features:
Establishes and maintains a connection to the SQLite database.
Performs Create, Read, Update, Delete (CRUD) operations for users and transactions.
Implements error handling with retry logic to handle database access issues (e.g., DatabaseBusy).
### function.rs
Purpose: Core utility functions for authentication, encryption, and transactions.
Key Features:
Hashes passwords with Argon2 for secure user authentication.
Encrypts and decrypts sensitive data with AES encryption.
Provides utility functions for reusable operations across modules.
### transfer.rs
Purpose: Manages encrypted transfers between accounts.
Key Features:
Collects transfer details like recipient, amount, and any notes.
Encrypts sensitive data using AES encryption before storing it in the database.
Processes and records transactions securely, ensuring safe data handling.
### success.rs
Purpose: Displays success notifications for completed transactions.
Key Features:
Provides visual and auditory confirmation of successful transactions.
Plays a notification sound to alert users.
Offers options to return to the dashboard or initiate a new transaction.
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
### Structs 
Defined for modular data management, such as User, Account, and Transaction, to organize related fields.
### Enums
Used to represent various states within the app, such as user actions (Login, Signup, Transfer) and account statuses.
### Pass by Reference (&)
Utilized throughout the codebase for efficient memory usage, particularly in handling large structures or data-sensitive operations.
### Pattern Matching
Employed to streamline logic and ensure safe handling of various states and user inputs.
## Security Features
### AES Encryption
Secures sensitive data in storage and during transfers.
### Argon2 Hashing
Ensures user passwords are stored in a way that is resistant to brute-force attacks.
## Database Concurrency Control
SQLite database access is synchronized to prevent conflicts during concurrent transactions.
### Retry Logic for Database Access
Automatic retry for database interactions during high load or contention, ensuring stability.
## Usage
### User Authentication
Secure login and signup with password hashing and validation.
### Account Operations
Accessible from the main dashboard, enabling deposits, withdrawals, and transfers.
### Transaction Processing
Allows secure and encrypted data transfers between accounts.
