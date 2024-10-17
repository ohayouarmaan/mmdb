### **Redis Clone in Rust**

**Project Overview**

This project aims to create a Rust-based Redis clone. It currently provides core functionalities such as:

-   **Data Structures:** Support for `SET`, `DELETE`, and `GET` operations.
-   **Replication:** Implements both partial sync replication and full resync replication.
-   **RESP Protocol:** Compatible with the RESP protocol for communication.
-   **RDB Persistence:** Can load data from existing RDB files.

**Future Enhancements**

-   **Transactions:** Introduce support for transactions to ensure data consistency.
-   **Streams:** Implement streams for efficient pub/sub messaging and data processing.

**Getting Started**

1.  **Clone the Repository:**
    
    Bash
    
    ```
    git clone https://github.com/ohayouarmaan/mmdb
    
    ```
    
    **Build and Run:**

Bash

```
cd mmdb
cargo build --release
./target/release/mmdb --port 6379

```

**Usage**

-   **Connect:** Use a Redis client to connect to your server.
-   **Commands:** Execute Redis commands like `SET`, `GET`, `DELETE`, etc.
-   **Replication:** Configure replication settings and connect replica nodes.

**Contributing**

Contributions are welcome! Please follow these guidelines:

-   **Fork the repository.**
-   **Create a branch:**  `git checkout -b feature-your-feature`
-   **Make your changes.**
-   **Commit your changes:**  `git commit -m 'Add some feature'`
-   **Push to the branch:**  `git push origin feature-your-feature`
-   **Create a pull request.**
**License**

This project is licensed under the [MIT License](https://www.google.com/url?sa=E&source=gmail&q=https://opensource.org/licenses/MIT).
