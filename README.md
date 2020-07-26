# MonQ

## What is MonQ?

MonQ is a quiz editor.

## build

### dashboard

You have to install npm packages to build dashboard SPA.

```zsh
cd ../monq_dashboard/tailwind
npm install
cd ../../
cargo make build_dashboard
```

### backend

In an another terminal window, run this command.

```
cargo make start
```

The command starts a backend server in watch mode. After finishing the build process, access any one of following urls.

* [http://127.0.0.1:8080/](http://127.0.0.1:8080/)
* [http://127.0.0.1:8080/dashboard](http://127.0.0.1:8080/dashboard)

## Reference

### Project Structure

* [examples/juniper/](https://github.com/actix/examples/tree/master/juniper)
* [examples/juniper-advanced/](https://github.com/actix/examples/tree/master/juniper-advanced)
* [rust-graphql-juniper-actix-diesel-postgres](https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/tree/015cf2e116124f8553ee31263ff29ecc8a1bfa3f)
* [actix-with-elasticsearch](https://github.com/kikei/actix-with-elasticsearch)