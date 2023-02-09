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

This repository aims to grow over time to include more features and more technologies that are commonly used in web stacks and Rust projects.

The project consists of a actix-hosted production environment while still maintaining the Vite development server for better development experience.
See the 'How to run' and 'Developement' sections below


## The Stack
### The Backend
 - Diesel will be used as an ORM to interact with the database.
    - Argon2 will be used to hash passwords.
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

### Docker
 - Install Docker
 - Run `docker build -t rust-actix-auth-flow-example .` to build the image
 - run `docker run -p 8000:8000 rust-actix-auth-flow-example:latest` to run the image and expose the port
 - The backend will be running on `localhost:8000` and will statically serve the frontend on the same port
 - head to `localhost:8000` to see the frontend and interact with the backend

### Alternatively you can run the backend and frontend separately and set up the database manually
## Database
 - Install PostgreSQL
 - Create a database named `rust-auth-example`
 - Create a superuser(to avoid permission issues) named `example` with (optionally) no password.
 -  update the env file with the correct database credentials in the case where you want a custom username and password and database name
 
 ## Frontend
 - Install Node.js
 - Run `npm install` to install the dependencies
 - Run `npm build` to build the frontend
 - The frontend will be statically hosted by the backend so they have the same domain

## Backend
 - Install Rust and Cargo
 - Install Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)(make sure you have the postgres-libs installed)
 - Cookie same site is set to strict when running in production mode.
 - Run `ENV=prod cargo run --release` to run the backend, the migrations will automatically apply.
 - The backend will be running on `localhost:8000`
 - head to 'localhost:8000' to interact with the frontend



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



## Development
There are some weird issues with vite and firefox, so I recommend using chrome for development. I will try to fix this in the future.
### Database
 - Install PostgreSQL
 - Create a database named `rust-auth-example`
 - Create a superuser(to avoid permission issues) named `example` with (optionally) no password.
 -  update the env file with the correct database credentials in the case where you want a custom username and password and database name

### Backend 
 - Install Rust and Cargo
 - Install Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)(make sure you have the postgres-libs installed)
 - set up the `DATABASE_URL` environment variable in the .env file to point to the database
 - Run `diesel migration run` to run the migrations
 - Run `cargo run` to run the backend
 - The backend will be running on `localhost:8000`
 - Cookie security is set to lax different when running in development mode

### Frontend
 - Install Node.js
 - Run `npm install` to install the dependencies
 - Run `npm run dev` to run vite and start a development server
 - it will be running on `localhost:5173`
 - Requests will automatically be routed to the backend on `localhost:8000` 
 - The frontend will be statically hosted by the backend so they have the same domain
