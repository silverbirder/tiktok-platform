resource "google_cloud_run_service" "tiktok-scraper-api" {
  name     = "tiktok-scraper-api"
  project  = var.project
  location = var.location

  template {
    spec {
      containers {
        image = "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/cloud-run/tiktok-scraper-api:latest"
      }
    }
  }
}

data "google_iam_policy" "noauth" {
  binding {
    role = "roles/run.invoker"
    members = [
      "allUsers",
    ]
  }
}

resource "google_cloud_run_service_iam_policy" "noauth" {
  location = google_cloud_run_service.tiktok-scraper-api.location
  project  = google_cloud_run_service.tiktok-scraper-api.project
  service  = google_cloud_run_service.tiktok-scraper-api.name

  policy_data = data.google_iam_policy.noauth.policy_data
}