window.BENCHMARK_DATA = {
  "lastUpdate": 1776796842384,
  "repoUrl": "https://github.com/cilladev/xlstream",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "8a56bfc7f8ba37a8da705e489c98e052af5e881c",
          "message": "chore: update regresion jobs name",
          "timestamp": "2026-04-21T14:00:56+01:00",
          "tree_id": "832b74768cd2a4e3c1e8314d891f75d7e67ec4c5",
          "url": "https://github.com/cilladev/xlstream/commit/8a56bfc7f8ba37a8da705e489c98e052af5e881c"
        },
        "date": 1776788746917,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 215,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 225,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 223,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20918,
            "range": "± 110",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "591bb924c72da8cad3596e0a6b46bbf263504ec6",
          "message": "ci: split into parallel jobs — unit-tests, end-to-end, regression\n\n- unit-tests: cross-platform cargo test + doctests\n- end-to-end: per-feature integration tests (ubuntu only)\n- regression: golden-file vs Excel (ubuntu only)\n- remove separate accuracy.yml (merged into ci.yml)\n- all jobs run in parallel",
          "timestamp": "2026-04-21T18:33:03+01:00",
          "tree_id": "1fd38014baab72aaae3a219d2119c3d365ed8855",
          "url": "https://github.com/cilladev/xlstream/commit/591bb924c72da8cad3596e0a6b46bbf263504ec6"
        },
        "date": 1776792997097,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 216,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 71,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 201,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 204,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20993,
            "range": "± 119",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "53d39b6bf2cf077f0e41cdd6aadcab417ad1be4d",
          "message": "chore: update contributing.md",
          "timestamp": "2026-04-21T18:40:07+01:00",
          "tree_id": "eecf0db91137f99b21ff1bede3b354cfd0fab9d2",
          "url": "https://github.com/cilladev/xlstream/commit/53d39b6bf2cf077f0e41cdd6aadcab417ad1be4d"
        },
        "date": 1776793427783,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 229,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 222,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 222,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 19591,
            "range": "± 44",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "3ee78e62963fd7f5483d2ecd5b9bec19f8c95951",
          "message": "chore: del benchmark file",
          "timestamp": "2026-04-21T19:06:21+01:00",
          "tree_id": "a27166139904f6cb73fc9accc5d98bbcd7b86722",
          "url": "https://github.com/cilladev/xlstream/commit/3ee78e62963fd7f5483d2ecd5b9bec19f8c95951"
        },
        "date": 1776794993864,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 212,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 71,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 198,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 198,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21070,
            "range": "± 103",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "cc6b4a7a76bffe1ce0557a3c0dca1867bc612e45",
          "message": "chore: update supported funcs",
          "timestamp": "2026-04-21T19:20:49+01:00",
          "tree_id": "ee91e08834dbf66925e77fa9279c7db410db182c",
          "url": "https://github.com/cilladev/xlstream/commit/cc6b4a7a76bffe1ce0557a3c0dca1867bc612e45"
        },
        "date": 1776795869995,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 216,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 71,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 230,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 229,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20668,
            "range": "± 103",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "b237fc83e0771272b5e3d5aca98d0406eb4d5499",
          "message": "release: bump to 0.2.0",
          "timestamp": "2026-04-21T19:29:00+01:00",
          "tree_id": "1255886a4f32a839c45d9c9fd077fd43afd9bba1",
          "url": "https://github.com/cilladev/xlstream/commit/b237fc83e0771272b5e3d5aca98d0406eb4d5499"
        },
        "date": 1776796361932,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 28,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 28,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 28,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 28,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 195,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 67,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 67,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 173,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 173,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 18540,
            "range": "± 48",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "b8304c4bc5ecaec7be7b53999dd415f8565bd568",
          "message": "fix: point pypi readme to root README.md",
          "timestamp": "2026-04-21T19:37:10+01:00",
          "tree_id": "070c2d6cc58d51bc55dec982de2ae3f00b6ad101",
          "url": "https://github.com/cilladev/xlstream/commit/b8304c4bc5ecaec7be7b53999dd415f8565bd568"
        },
        "date": 1776796842157,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 33,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 213,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 206,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 205,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20791,
            "range": "± 213",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}