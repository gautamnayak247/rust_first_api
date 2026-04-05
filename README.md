# Rust API

This project is a simple Rust-based API built using the Axum framework. It provides endpoints to manage `Employee` resources.

## Endpoints

### Health Check
- **GET /**
  - Returns: `"healthy!"`

### Employee Endpoints

- **POST /employees**
  - Description: Create a new employee.
  - Request Body:
    ```json
    {
      "id": 1,
      "name": "John Doe",
      "role": "Developer"
    }
    ```
  - Response:
    - Status: `201 Created`
    - Body:
      ```json
      {
        "id": 1,
        "name": "John Doe",
        "role": "Developer"
      }
      ```

- **PUT /employees**
  - Description: Update an existing employee.
  - Request Body: Same as POST.
  - Response:
    - Status: `200 OK`
    - Body: Same as POST.

- **GET /employees**
  - Description: Retrieve an example employee.
  - Response:
    - Status: `200 OK`
    - Body:
      ```json
      {
        "id": 1,
        "name": "Gautam",
        "role": "Permanent"
      }
      ```

## Running the Project

1. Install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone the repository.
3. Run the server:
   ```bash
   cargo run
   ```
4. Access the API at `http://127.0.0.1:3000`.

## Dependencies

- **Axum**: Web framework for building APIs.
- **Serde**: Serialization and deserialization library.
- **Tokio**: Asynchronous runtime for Rust.

## License

This project is licensed under the MIT License.