resource "google_cloudbuild_trigger" "tiktok-platform-kubernetes-cronjob-storer" {
  project     = var.project
  name        = "tiktok-platform-kubernetes-cronjob-storer"
  description = "Cloud Build to trigger kubernetes/resources/cronjob/storer source code changes."
  build {
    images = [
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/storer:$COMMIT_SHA",
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/storer:latest",
    ]
    step {
      dir  = "kubernetes/resources/cronjob/storer/"
      name = "gcr.io/cloud-builders/docker"
      args = [
        "build",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/storer:$COMMIT_SHA",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/storer:latest",
        ".",
      ]
    }
    timeout = "600s"
  }
  included_files = [
    "kubernetes/resources/cronjob/storer/**",
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

resource "google_cloudbuild_trigger" "tiktok-platform-kubernetes-cronjob-transfer" {
  project     = var.project
  name        = "tiktok-platform-kubernetes-cronjob-transfer"
  description = "Cloud Build to trigger kubernetes/resources/cronjob/transfer source code changes."
  build {
    images = [
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/transfer:$COMMIT_SHA",
      "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/transfer:latest",
    ]
    step {
      dir  = "kubernetes/resources/cronjob/transfer/"
      name = "gcr.io/cloud-builders/docker"
      args = [
        "build",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/transfer:$COMMIT_SHA",
        "-t",
        "gcr.io/${var.project}/github.com/silver-birder/tiktok-platform/kubernetes/resources/cronjob/transfer:latest",
        ".",
      ]
    }
    timeout = "600s"
  }
  included_files = [
    "kubernetes/resources/cronjob/transfer/**",
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