# Rust-auth-example
This repository aims to represent a simple authentication flow using Rust and Actix-web, with a PostgreSQL database and a sveltekit frontend. 


## The Stack
### The Backend
 - Diesel will be used as an ORM to interact with the database.
    - Bcrypt will be used to hash passwords.
 - Actix web will be used as a web framework.
    - Actix-session will be used to manage sessions.
    - Actix-cors will be used to manage CORS.
 - Juniper will be used as a GraphQL framework.
 

### The Frontend
 - Sveltekit will be used as a frontend framework.
 - Tailwind will be used as a CSS framework, with flowbite.js for responsive components.
 - We will use normal fetch requests to interact with the backend.


## How to run
### Backend
 - Install Rust and Cargo
 - Install PostgreSQL
 - Install Diesel CLI
 - Run `diesel setup` to create the database
 - Run `diesel migration run` to run the migrations
 - Run `cargo run` to run the backend
 - The backend will be running on `localhost:8080`

### Frontend
 - Install Node.js
 - Run `npm install` to install the dependencies
 - Run `npm run dev` to run the frontend
 - The frontend will be running on `localhost:3000`

## How to use
