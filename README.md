# rust-webapp-template

This is my personal "batteries included" template to quickly create full stack web applications with a database, authentication and a REST API.

> [!WARNing]  
> This template is created to purpose-fit my exact needs and thus is very opinionated and inflexible. If you still want to use it, feel free to do so though.

## Stack

The following technology stack has been chosen for this template:

### Backend

The backend is written in [Rust](https://www.rust-lang.org/). [Rocket](https://rocket.rs/) is used for the HTTP server and REST API handling. Authentication is handled via [OpenID Connect](https://openid.net/developers/how-connect-works/), which is an extension of OAuth2. Database bindings and migrations are done with [sqlx](https://docs.rs/sqlx/latest/sqlx/) and are implemented with PostgreSQL as Database. Configuration is managed via [figment](https://docs.rs/figment/latest/figment/), based on environment variables. But other configuration sources can easily be added, if desired.

### Frontend

The frontend is a [React](https://react.dev/) SPA; bundled with [Vite](https://vitejs.dev/) and [SWC](https://swc.rs/). As JS runtime, [bun](https://bun.sh/) is used. Styling is done with [styled-components ](https://styled-components.com/) and state management is handled with [zustand](https://docs.pmnd.rs/zustand/getting-started/introduction).