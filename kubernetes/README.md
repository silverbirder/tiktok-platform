## Argo Workflow
### Install

```
kubectl apply namespace/argo.yaml
kubectl apply -f https://raw.githubusercontent.com/argoproj/argo/v2.2.1/manifests/install.yaml -n argo
```

```
kubectl -n argo port-forward deployment/argo-server 2746:2746
```