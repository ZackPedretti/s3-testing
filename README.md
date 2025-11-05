## Testing the API

To test the API, you can use **Bruno**, **Insomnia**, **Postman**, or any other HTTP IDE to send requests defined in the `/requests` directory.

### 1. Set up your environment

Before launching the stack, create a `.env` file in the project root.  
This file defines the credentials and configuration for your local MinIO instance and the API service user.

#### Environment variables

| Variable              | Description                                                                        |
| --------------------- | ---------------------------------------------------------------------------------- |
| `MINIO_ROOT_USER`     | The root username for the MinIO instance (used for initial login and admin tasks). |
| `MINIO_ROOT_PASSWORD` | The root password for the MinIO instance.                                          |
| `MINIO_DEV_USER`      | The username for the API applications to authenticate with MinIO.                  |
| `MINIO_DEV_PASSWORD`  | The password associated with the API user account in MinIO.                        |

### 2. Launch the Docker environment

Run the following command to start all services:

```bash
docker compose up -d
```

This will start the Fastify and Axum API servers along with a MinIO instance for local object storage.

To run only one API server, you can also run the respective Docker compose file to the service.
E.g: To run the Fastify API:

```bash
docker compose -f compose-fastify.yaml up -d
```

#### Launch the Axum API for development

In the `/axum` directory, run the following command to build the executable in dev mode:
```bash
cargo build
```
At the first build, cargo (the rust compiler) will compile all the code and the dependencies. Afterwards, it will only compile the code outside of the dependencies.
Then, to start the Docker containers, run this command:

```bash
docker compose -f compose-axum.yaml up -d
```
The development Docker container is organized this way to avoid recompiling all the dependencies every time we need to test the Rust code. In consequence, the build time is much faster.
### 3. Configure MinIO

#### Creating the bucket
The APIs need a bucket called "Jukebox", which you can create by going to [the MinIO console page](http://localhost:9000).

#### Creating the dev user
Once the containers are running:

1. Install the MinIO CLI Tool (mc) using this command:
```bash
wget https://dl.min.io/client/mc/release/linux-amd64/mc
```
2. Assign the executable right to mc by running
```bash
chmod +x mc
```
3. Create a new **Dev user** using the username and password from your `.env` file in this command:
```bash
./mc admin user add minio <username> <password>
```
4. Assign policies or permissions that allow the Fastify user to read and write by running this command:
```bash
./mc admin policy attach minio readwrite --user <username>
```

### 4. Test the endpoints

After setup, you can send requests to the API endpoints using your chosen HTTP IDE.  
Make sure the API container is running and your `.env` configuration matches your Docker network setup.
