# resource "google_sql_database" "tiktok" {
# }

resource "google_sql_database_instance" "tiktoker" {
  database_version    = "MYSQL_5_7"
  deletion_protection = true
  name                = "tiktoker"
  project             = var.project
  region              = "us-central1"
  # region = var.region
  settings {
    activation_policy     = "NEVER"
    availability_type     = "ZONAL"
    disk_autoresize       = true
    disk_autoresize_limit = 0
    disk_size             = 10
    disk_type             = "PD_HDD"
    pricing_plan          = "PER_USE"
    tier                  = "db-f1-micro"
    database_flags {
      name  = "default_time_zone"
      value = "+09:00"
    }
    maintenance_window {
      hour         = 0
      update_track = ""
    }
    location_preference {
      follow_gae_application = ""
      zone                   = "us-central1-f"
      # zone = var.zone
    }
  }
}
