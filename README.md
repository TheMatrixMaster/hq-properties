# hq-properties
Professional real estate agent website

## Description
1. Frontend web-app written in Svelte
2. Backend REST API written in Rust using Rocket web framework and Diesel ORM
3. Multi-threaded filewatcher written in Rust using Notify crate

## Development
### Setup API
1. `cd backend`
2. Create an `env.sh` file using the `env.example.sh` template
3. Source the `env.sh` file
4. Spin up a postgres docker container using the configured env variables
   ```bash
   make setup
   make db
   ``` 
6. Compile and start the REST API service: `make run_api`

### Setup frontend
1. `cd frontend`
2. Install dependencies: `pnpm i`
3. Start the development server: `pnpm dev`

### Setup filewatcher (after API setup)
1. `cd backend`
2. Source the `env.sh` file
3. Create a watch directory for the filewatcher to inspect (or use an existing directory)
   ```bash
   mkdir .watchdir
   ```
4. Compile and start the filewatcher service: `make run_fw`
5. Create a Centris `.zip` archive file in the `.watchdir`

## Deployment
### CI/CD Pipeline
I've setup github actions that build and push docker images to the public docker registry at namespace 
[1930414](https://hub.docker.com/u/1930414)
- On push to branch `develop`, I run tests on both the backend and frontend services
- On push to branch `main`, I build and push docker images for:
  - [REST api](https://hub.docker.com/repository/docker/1930414/hq-api/general)
  - [Filewatcher](https://hub.docker.com/repository/docker/1930414/hq-fw)
  - [Web app](https://hub.docker.com/repository/docker/1930414/hq-webapp)
  - [Nginx RP](https://hub.docker.com/repository/docker/1930414/hq-nginx)

 ### Deploying to production
I use the following [setup](https://github.com/woollysammoth/sveltekit-docker-nginx) by 
@woollysammoth to handle ssl certificates and docker images on my production server. I also setup a postgres instance on
the host machine instead of using docker for security and maintenance advantages.
