## GKE Autopilot

```
$ gcloud container clusters get-credentials my-gke-cluster --region asia-northeast1 --project $(gcloud config get-value project)
$ kubectl config current-context
```

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
# kubectl apply -n argo -f https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/quick-start-postgres.yaml
kubectl apply -n argo -f quick-start-postgres.yaml
# kubectl apply -n argo -f quick-start-postgres.gke-spot.yaml
kubectl get all -n argo
```

```
kubectl -n argo create secret generic my-secret --from-file=key.json=$(echo $GOOGLE_APPLICATION_CREDENTIALS)
kubectl -n argo create configmap users --from-file=../configmap/users.yaml
kubectl -n argo describe configmaps users
```

```
kubectl -n argo port-forward deployment/argo-server 2746:2746
```

```
argo submit -n argo --parameter-file env.yaml --watch tiktok-platform-workflow.gke-spot.yaml
argo submit -n argo --parameter-file env.yaml --watch tiktok-platform-workflow.yaml
```

```
argo cron create -n argo --parameter-file env.yaml tiktok-platform-workflow.yaml
```