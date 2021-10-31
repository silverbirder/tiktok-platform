resource "google_project_service" "container-service" {
  project = var.project
  service = "container.googleapis.com"

  disable_dependent_services = true
}