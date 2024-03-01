# Rust Microservice Example with Axum

## Prerequisities

- Rust
- Docker
- Minikube (for local Kubernetes deployment)
- kubectl (for interacting with Kubernets clusters)

## Installation
1. Clone the repository:
```bash
git clone https://github.com/entangle2giraffe/rust-microservice.git
```

2. Navigate to the project directory:
```bash
cd rust-microservice
```

3. Build the Docker image:
```bash
docker build -t rust-microservice .
```

4. (Optional) Load the Docker image into Minikube:
```bash
minikube image load rust-microservice
```

## Deployment

To deploy the application to Minikube:

1. Apply the Kubernetes manifests:
```bash
kubectl apply -f k8s/
```

2. Access the service:
```bash
minikube service -n sandbox-ns rust-microservice-service
```

3. Visit /health
```json
{"status":"healthy"}
```