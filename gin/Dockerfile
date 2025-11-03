FROM golang:1.25.3-alpine

# Install git for module fetching (optional if you don't fetch modules inside the container)
RUN apk add --no-cache git

WORKDIR /app

# Copy go.mod and go.sum first (to cache dependencies)
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

COPY . .

EXPOSE 3002

CMD ["go", "run", "."]
