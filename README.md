## Install rust 🦀
``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
This project is meant to illustrate how to use [Rocket](https://rocket.rs/t), [Diesel](https://diesel.rs/) and [rocket_okapi](https://docs.rs/rocket_okapi/latest/rocket_okapi/)

<details>
  <summary> Rocket 🚀</summary>
Is an amazing crate created by ex SpaceX's employee [Sergio Benitez](https://github.com/SergioBenitez), which allows the creation of blazingly fast APIs in Rust. The reasons I fell in love with it are amazing documentation, effortless management of database connections, and the way it can get data from the request automatically using Rust `Traits` ([Example](https://stackoverflow.com/questions/64829301/how-to-retrieve-http-headers-from-a-request-in-rocket/64838724#64838724)). 

</details>

<details>
  <summary> Diesel ⛽</summary>
Diesel is the most productive way to interact with databases in Rust because of its safe and composable abstractions over queries. It eliminates the need to worry about errors by having Diesel eliminate the possibility of incorrect database interactions at compile time.
It can manage migration to different databases like `Postgres`, `MySql`, `SQLite`, and autogenerate rust `Structs` from the schema of tables. 

</details>

### **rocket_okapi** 📚
Automated OpenAPI (AKA [Swagger](https://swagger.io/tools/swagger-ui/)) document generation for Rust/Rocket projects. Never have outdated documentation again. Okapi will generate documentation for you while setting up the server.

# How to get things up and running: 
- `docker-compose up --build -d`  it requires [**Docker**](https://docs.docker.com/engine/install/ubuntu/) and [**Docker Compose**](https://docs.docker.com/compose/install/) to be installed
