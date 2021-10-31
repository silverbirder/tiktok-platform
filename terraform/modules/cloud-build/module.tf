resource "google_cloudbuild_trigger" "tiktok-platform-scraper" {
  project     = var.project
  name        = "tiktok-platform-scraper"
  description = "Cloud Build to trigger kubernetes/resources/cronjob/scraper source code changes."
  build {
    images = [
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/scraper:$COMMIT_SHA",
    ]
    step {
      dir  = "kubernetes/resources/cronjob/scraper/"
      name = "gcr.io/cloud-builders/docker"
      args = [
        "build",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/scraper:$COMMIT_SHA",
        ".",
      ]
    }
    timeout = "600s"
  }
  included_files = [
    "kubernetes/resources/cronjob/scraper/**",
  ]
  github {
    name  = "tiktok-platform"
    owner = "Silver-birder"
    push {
      branch       = "^main$"
      invert_regex = false
    }
  }
}