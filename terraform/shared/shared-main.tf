module "service" {
  source  = "../modules/service"
  project = var.project
}

module "kubernetes-engine" {
  source   = "../modules/kubernetes-engine"
  project  = var.project
  location = var.location
}

module "cloud-build" {
  source  = "../modules/cloud-build"
  project = var.project
}

# module "cloud-sql" {
#   source           = "../modules/cloud-sql"
#   project          = var.project
#   region           = var.region
#   zone             = var.location
#   db_user_name     = var.db_user_name
#   db_user_password = var.db_user_password
# }

module "firestore" {
  source  = "../modules/firestore"
  project = var.project
}