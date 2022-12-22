# Create a new rancher2 RKE Cluster
resource "rancher2_cluster" "foo-custom" {
  name                                    = "foo-custom"
  description                             = "Foo rancher2 custom cluster"
  default_pod_security_policy_template_id = "restricted"
  rke_config {
    network {
      plugin = "canal"
    }
    services {
      kube_api {
        audit_log {
          enabled = false
        }
      }
    }
  }
}
