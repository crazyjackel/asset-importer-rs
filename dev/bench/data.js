window.BENCHMARK_DATA = {
  "lastUpdate": 1749405856380,
  "repoUrl": "https://github.com/crazyjackel/asset-importer-rs",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "jackel1020@gmail.com",
            "name": "J L",
            "username": "crazyjackel"
          },
          "committer": {
            "email": "jackel1020@gmail.com",
            "name": "J L",
            "username": "crazyjackel"
          },
          "distinct": true,
          "id": "dc95b590c78ff6e7f2aa9956378dbac9a3a80fc0",
          "message": "Hotfix #2\n\nBenchmarking did not work due criterion and lib bench... ideally this fixes it when run locally",
          "timestamp": "2025-02-12T11:12:54-05:00",
          "tree_id": "92627ca61d616ccc7af770ce55be475ba6140977",
          "url": "https://github.com/crazyjackel/asset-importer-rs/commit/dc95b590c78ff6e7f2aa9956378dbac9a3a80fc0"
        },
        "date": 1739376889339,
        "tool": "cargo",
        "benches": [
          {
            "name": "Import Group/import avocado",
            "value": 96121952,
            "range": "± 303443",
            "unit": "ns/iter"
          },
          {
            "name": "Export Group/export avocado",
            "value": 196740673,
            "range": "± 431941",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "47047592+crazyjackel@users.noreply.github.com",
            "name": "Jackson Levitt",
            "username": "crazyjackel"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ab438bb35eaefb8197ec721d0345a4b18bd97c8f",
          "message": "Merge pull request #10 from crazyjackel/jlevitt/gltf-v1-export\n\nGLTF 1.0 Export",
          "timestamp": "2025-06-08T13:01:57-05:00",
          "tree_id": "a971e7249c3695bc93bce585629b91cb83743053",
          "url": "https://github.com/crazyjackel/asset-importer-rs/commit/ab438bb35eaefb8197ec721d0345a4b18bd97c8f"
        },
        "date": 1749405856015,
        "tool": "cargo",
        "benches": [
          {
            "name": "Import Group/import avocado (gltf2)",
            "value": 96850102,
            "range": "± 462016",
            "unit": "ns/iter"
          },
          {
            "name": "Import Group/import avocado (gltf)",
            "value": 47050017,
            "range": "± 275869",
            "unit": "ns/iter"
          },
          {
            "name": "Export Group/export avocado (gltf2)",
            "value": 199534600,
            "range": "± 654862",
            "unit": "ns/iter"
          },
          {
            "name": "Export Group/export avocado (gltf)",
            "value": 47228872,
            "range": "± 170715",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}