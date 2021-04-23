# What is this
Simple demo app for argocd setup

# Turn on logs
RUST_LOG=debug cargo run

# Debug in vscode
Install plugin: codellDB


# Folder structure
1. k8s -> Kubernetes manifests for deploying application stack to k8s
2. locust -> Python benchmarking tool
3. src -> Rust code for the main application


# Spin up local environment
kind create cluster --name argo --config kind-config.yaml

## Tagging

x.x.x for versioning releases.