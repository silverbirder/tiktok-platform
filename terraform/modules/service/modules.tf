resource "google_project_service" "container-service" {
  project = var.project
  service = "container.googleapis.com"

  disable_dependent_services = true
}

resource "google_project_service" "cloudbuild-service" {
  project = var.project
  service = "cloudbuild.googleapis.com"

  disable_dependent_services = true
}

resource "google_project_service" "cloudrun-service" {
  project = var.project
  service = "run.googleapis.com"

  disable_dependent_services = true
}