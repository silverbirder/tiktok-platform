module "service" {
  source   = "../modules/service"
  project  = var.project
}

module "kubernetes-engine" {
  source   = "../modules/kubernetes-engine"
  project  = var.project
  location = var.location
}

module "cloud-build" {
  source   = "../modules/cloud-build"
  project  = var.project
}

module "cloud-run" {
  source   = "../modules/cloud-run"
  project  = var.project
  location = var.location
}