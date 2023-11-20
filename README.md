# authentication-app
## Back-end
Runs on [Actix Web](https://actix.rs/) framework. Compile by first set up the database connection via **diesel setup** run the service with **cargo run**.
## Storage
Cloud based MySql server. For back-end to work, docker image should be running. ORM is managed with Diesel.
## Front-end
Built with [Yew](https://yew.rs/).
Compile with **trunk serve --port 3000** Navigate to http://localhost:3000/. The application will automatically reload if you change any of the source files.
## Proxy
Front-end and back-end share a common proxy route.