resource "google_firestore_document" "document" {
  project     = var.project
  collection  = "collection"
  document_id = "document"
  fields      = "{\"key\":{\"stringValue\":\"value\"}}"
}