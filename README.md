## Testing the API

To test the API, you can use **Bruno**, **Insomnia**, **Postman**, or any other HTTP IDE to send requests defined in the `/requests` directory.

### 1. Set up your environment

Before launching the stack, create a `.env` file in the project root.  
This file defines the credentials and configuration for your local MinIO instance and the Fastify service user.

#### Environment variables

| Variable                 | Description                                                                                       |
| ------------------------ | ------------------------------------------------------------------------------------------------- |
| `MINIO_ROOT_USER`        | The root username for the MinIO instance (used for initial login and admin tasks).                |
| `MINIO_ROOT_PASSWORD`    | The root password for the MinIO instance.                                                         |
| `MINIO_ENDPOINT`         | The internal URL where the Fastify API connects to the MinIO service (e.g., `http://minio:9000`). |
| `MINIO_FASTIFY_USER`     | The username for the Fastify application to authenticate with MinIO.                              |
| `MINIO_FASTIFY_PASSWORD` | The password associated with the Fastify user account in MinIO.                                   |

### 2. Launch the Docker environment

Run the following command to start all services:

```bash
docker compose up -d
```

This will start the Fastify API server along with a MinIO instance for local object storage.

### 3. Configure MinIO

Once the containers are running:

1. Open the MinIO Console (typically at [http://localhost:9090](http://localhost:9090)).
2. Log in with the **root** credentials defined in your `.env` file.
3. Create a new **Fastify user** using the username and password from your `.env` file.
4. Assign policies or permissions that allow the Fastify user to read and write to the necessary buckets.

### 4. Test the endpoints

After setup, you can send requests to the API endpoints using your chosen HTTP IDE.  
Make sure the API container is running and your `.env` configuration matches your Docker network setup.
