terraform {
  backend "gcs" {
  }
}

resource "google_storage_bucket" "tfstate" {
  name          = "${var.project}-tfstate"
  location      = var.location
  force_destroy = true

  versioning {
    enabled = true
  }

  lifecycle {
    prevent_destroy = true
  }
}