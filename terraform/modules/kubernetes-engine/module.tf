resource "google_container_cluster" "primary" {
  name             = "my-gke-cluster"
  location         = var.location
  project          = var.project
  enable_autopilot = true
  vertical_pod_autoscaling {
    enabled = true
  }
  lifecycle {
    ignore_changes = [
      master_version
    ]
  }
  release_channel {
    channel = "RAPID"
    # for gke-spot
    # @see: https://cloud.google.com/blog/ja/products/containers-kubernetes/announcing-spot-pods-for-gke-autopilot
  }
}