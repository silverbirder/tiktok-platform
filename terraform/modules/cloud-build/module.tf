resource "google_cloudbuild_trigger" "tiktok-platform-kubernetes-cronjob-scraper" {
  project     = var.project
  name        = "tiktok-platform-kubernetes-cronjob-scraper"
  description = "Cloud Build to trigger kubernetes/resources/cronjob/scraper source code changes."
  build {
    images = [
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/scraper:$COMMIT_SHA",
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/scraper:latest",
    ]
    step {
      dir  = "kubernetes/resources/cronjob/scraper/"
      name = "gcr.io/cloud-builders/docker"
      args = [
        "build",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/scraper:$COMMIT_SHA",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/scraper:latest",
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

resource "google_cloudbuild_trigger" "tiktok-platform-cloud-run-tiktok-scraper-api" {
  project     = var.project
  name        = "tiktok-platform-cloud-run-tiktok-scraper-api"
  description = "Cloud Build to trigger cloud-run/tiktok-scraper-api source code changes."
  build {
    images = [
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/cloud-run/tiktok-scraper-api:$COMMIT_SHA",
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/cloud-run/tiktok-scraper-api:latest",
    ]
    step {
      dir  = "cloud-run/tiktok-scraper-api/"
      name = "gcr.io/cloud-builders/docker"
      args = [
        "build",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/cloud-run/tiktok-scraper-api:$COMMIT_SHA",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/cloud-run/tiktok-scraper-api:latest",
        ".",
      ]
    }
    timeout = "600s"
  }
  included_files = [
    "cloud-run/tiktok-scraper-api/**",
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