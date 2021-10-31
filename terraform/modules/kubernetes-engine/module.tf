resource "google_container_cluster" "primary" {
  name             = "my-gke-cluster"
  location         = var.location
  project          = var.project
  enable_autopilot = true
  vertical_pod_autoscaling {
    enabled = true
  }
}