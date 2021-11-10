provider "google-beta" {
  region = var.region
  zone   = var.zone
}

resource "google_compute_network" "private_network" {
  provider = google-beta
  project  = var.project
  name     = "private-network"
}

resource "google_compute_global_address" "private_ip_address" {
  provider      = google-beta
  project       = var.project
  name          = "private-ip-address"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.private_network.id
}

resource "google_service_networking_connection" "private_vpc_connection" {
  provider = google-beta

  network                 = google_compute_network.private_network.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_ip_address.name]
}

resource "google_sql_database" "tiktok" {
  name     = "tiktok"
  project  = var.project
  instance = google_sql_database_instance.tiktoker.name
}

resource "google_sql_database_instance" "tiktoker" {
  provider            = google-beta
  database_version    = "MYSQL_5_7"
  deletion_protection = false
  name                = "tiktoker-instance"
  project             = var.project
  region              = var.region
  depends_on          = [google_service_networking_connection.private_vpc_connection]
  settings {
    availability_type = "ZONAL"
    disk_type         = "PD_HDD"
    pricing_plan      = "PER_USE"
    tier              = "db-f1-micro"
    backup_configuration {
      enabled            = false
      binary_log_enabled = false
    }
    database_flags {
      name  = "default_time_zone"
      value = "+09:00"
    }
    ip_configuration {
      ipv4_enabled    = false
      private_network = google_compute_network.private_network.id
    }
  }
}

resource "google_sql_user" "root" {
  name     = var.db_user_name
  project  = var.project
  instance = google_sql_database_instance.tiktoker.name
  password = var.db_user_password
}