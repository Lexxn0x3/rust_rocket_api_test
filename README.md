# Rocket API Example Template

This repository is a simple template for building a versioned REST API using the Rocket framework in Rust. It serves as a basic example to help you get started with Rocket, demonstrate API versioning, and implement CORS (Cross-Origin Resource Sharing) support.

## Project Structure

The project is organized as follows:

```bash
src/
├── api/
│ ├── mod.rs # API module entry point
│ ├── v1/ # Version 1 of the API
│ │ ├── mod.rs # API v1 module entry point
│ │ ├── pets.rs # API v1 routes for pets
│ └── v2/ # Version 2 of the API
│ ├── mod.rs # API v2 module entry point
│ ├── pets.rs # API v2 routes for pets
├── main.rs # Main application entry point
```

## Features

- **Versioned API**: Demonstrates how to create and manage multiple versions of your API (e.g., `/api/v1/`, `/api/v2/`).
- **CORS Support**: Configured to handle cross-origin requests, allowing you to test the API with tools like Swagger UI or Postman.
- **Basic CRUD Operations**: Simple endpoints for creating and retrieving "pet" records, structured differently across API versions.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Ensure you have the latest stable version installed)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rusts package manager)

### Installation

1. **Clone the repository**:

    ```bash
    git clone https://github.com/your-username/rocket-api-template.git
    cd rocket-api-template
    ```

2. **Install dependencies**:

    Cargo will automatically handle dependencies when you build the project.

    ```bash
    cargo build
    ```

3. **Run the application**:

    Start the Rocket server locally:

    ```bash
    cargo run
    ```

    The server will start on `http://localhost:8000`.

### API Endpoints

#### Version 1 (v1)

- **Create Pet**: `POST /api/v1/pets`
- **Get Pet by ID**: `GET /api/v1/pet/<id>`

Example JSON payload for creating a pet:

```json
{
  "id": 1,
  "name": "Buddy",
  "species": "Dog"
}
```

#### Version 1 (v1)

- **Create Pet**: `POST /api/v2/pets`
- **Get Pet by ID**: `GET /api/v2/pet/<id>`

Example JSON payload for creating a pet:

```json
{
  "id": 1,
  "name": "Susi",
  "species": "Cat"
}
```

## CORS
The API is configured to allow CORS requests from any origin. This makes it easy to test the API using web-based tools like Swagger UI. CORS settings are managed in the main.rs file using the rocket_cors crate.

## Project Goals
Learning: This template is designed to help you understand the basics of Rocket, including how to set up versioned APIs and configure CORS.
Template: You can use this as a starting point for your own projects, expanding on the basic structure and functionality provided.
## Contributing
Feel free to fork this repository, make improvements, and submit pull requests. Contributions are always welcome!
