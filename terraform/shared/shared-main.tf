module "service" {
  source   = "../modules/service"
  project  = var.project
}

module "kubernetes-engine" {
  source   = "../modules/kubernetes-engine"
  project  = var.project
  location = var.location
}