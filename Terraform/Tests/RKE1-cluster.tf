# Create a new rancher2 RKE Cluster
resource "rancher2_cluster" "test-rke" {
  name        = "balmeida-test"
  description = "Foo rancher2 custom cluster"
  rke_config {
    network {
      plugin = "canal"
    }
  }
}
