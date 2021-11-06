## Minikube

```
minikube start
```

### Local Docker Image

```
eval $(minikube -p minikube docker-env)
```

## Argo Workflow
### Install

```
kubectl apply -f ../namespace/argo.yaml
kubectl apply -n argo -f https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/quick-start-postgres.yaml
kubectl get all -n argo
```

```
kubectl -n argo port-forward deployment/argo-server 2746:2746
```

```
argo submit -n argo --watch yaml
```