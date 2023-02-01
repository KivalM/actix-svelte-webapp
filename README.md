# Rust-auth-example
This repository aims to represent a simple authentication flow using Rust and Actix-web, with a PostgreSQL database and a sveltekit frontend. 


## The Goal
The goal of this repository is to provide a simple example of how to implement session-based authentication in Rust & Actix. This repository will be used as a reference for future projects. This repository will also be used as a reference for future blog posts.

This repository also aims to provide examples of how to use the following technologies:
 - Diesel
 - Actix
 - Juniper
 - Sveltekit
 - Tailwind
 - DaisyUI

As well as provide examples of idiomatic Rust practices.

This repository aims to grow over time to include more features and more technologies that are commonly used in Rust projects.


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
 - Tailwind will be used as a CSS framework.
 - DaisyUI will be used for additional tailwind components.
 - We will use normal fetch requests to interact with the backend.


## How to run
### Database
 - Install PostgreSQL
 - Create a database named `rust-auth-example`
 - Create a user named `example` with no password
 - optionally update the env file with the correct database credentials in the case where you want a custom username and password

 - if you have permission issues connecting to the database
   - Grant all privileges to the user on the database (or just run `GRANT ALL PRIVILEGES ON DATABASE "rust-auth-example" TO example;`)

### Backend
 - Install Rust and Cargo
 - Install Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)
 - Run `diesel migration run` to run the migrations
 - Run `cargo run` to run the backend
 - The backend will be running on `localhost:8000`

### Frontend
 - Install Node.js
 - Run `npm install` to install the dependencies
 - Run `npm run dev` to run the frontend
 - The frontend will be running on `localhost:5173`

## How to use
### Frontend
   - The frontend is a simple login and register page.
   - The frontend will send a request to the backend to register a user.
   - The frontend will send a request to the backend to login a user.
   - The frontend will send a request to the backend to get the current user.
   - The frontend will send a request to the backend to logout a user.

### Backend
   - The backend will handle the requests from the frontend.
   - The backend will register a user.
   - The backend will login a user.
   - The backend will get the current user.
   - The backend will logout a user.
