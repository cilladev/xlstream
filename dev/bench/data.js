window.BENCHMARK_DATA = {
  "lastUpdate": 1777674912541,
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
          "id": "5d1b54429728a7f4f5456dd68f06f032405eca29",
          "message": "docs: restructure roadmap, archive phases, clean stale links\n\n- archive v0.1 phase docs to docs/roadmap/archive/v0.1/\n- add docs/roadmap/README.md with big picture and version plan\n- add docs/roadmap/v0.3/ placeholder\n- delete docs/backlog/v0.2.md (replaced by roadmap)\n- delete docs/research/benchmarks.md (replaced by benchmarks/reports/)\n- update README with expanded description and function table\n- fix all stale links to deleted files",
          "timestamp": "2026-04-22T01:31:23+01:00",
          "tree_id": "1c4e46010232504f6838fcdf65a043058c46e30c",
          "url": "https://github.com/cilladev/xlstream/commit/5d1b54429728a7f4f5456dd68f06f032405eca29"
        },
        "date": 1776818094525,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 33,
            "range": "± 1",
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
            "value": 33,
            "range": "± 2",
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
            "value": 211,
            "range": "± 0",
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
            "value": 21257,
            "range": "± 811",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "69baf66dee146b0f79b30f0607039a42d6ef36ef",
          "message": "build(deps): bump phf from 0.11.3 to 0.13.1\n\nBumps [phf](https://github.com/rust-phf/rust-phf) from 0.11.3 to 0.13.1.\n- [Release notes](https://github.com/rust-phf/rust-phf/releases)\n- [Changelog](https://github.com/rust-phf/rust-phf/blob/main/RELEASE_PROCESS.md)\n- [Commits](https://github.com/rust-phf/rust-phf/compare/phf-v0.11.3...v0.13.1)\n\n---\nupdated-dependencies:\n- dependency-name: phf\n  dependency-version: 0.13.1\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2026-04-22T01:53:28+01:00",
          "tree_id": "d3348692def7a29f635016d9a6317779c12fe9c3",
          "url": "https://github.com/cilladev/xlstream/commit/69baf66dee146b0f79b30f0607039a42d6ef36ef"
        },
        "date": 1776819435946,
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
            "value": 34,
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
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 46,
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
            "value": 214,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 72,
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
            "value": 196,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 196,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20452,
            "range": "± 123",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "67448c84c8e6f51997e71993fcc90fb7f3ca8a01",
          "message": "docs: add changelog entry for issue #42",
          "timestamp": "2026-04-22T22:29:32+01:00",
          "tree_id": "81359c44a482bb53c4c878be5a7f3e46cd43cccc",
          "url": "https://github.com/cilladev/xlstream/commit/67448c84c8e6f51997e71993fcc90fb7f3ca8a01"
        },
        "date": 1776893581285,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 35,
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
            "value": 36,
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
            "value": 215,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 200,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 199,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20969,
            "range": "± 509",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "3a22136d80fa22b587a130062bdc86ec135d98f6",
          "message": "docs: fix stale refs, update crate readmes, clean testing standards",
          "timestamp": "2026-04-23T02:26:59+01:00",
          "tree_id": "526793dff193ca66d11eb4de72464fdca696812f",
          "url": "https://github.com/cilladev/xlstream/commit/3a22136d80fa22b587a130062bdc86ec135d98f6"
        },
        "date": 1776907834963,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 37,
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
            "value": 49,
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
            "value": 230,
            "range": "± 1",
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
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 243,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 241,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20298,
            "range": "± 55",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "684a2606c19b33574a535e334aa670e4e0308021",
          "message": "docs: update pr template, remove stale regression pattern, add contributing prereq",
          "timestamp": "2026-04-23T03:07:25+01:00",
          "tree_id": "2d5ca360837efa063883aa6b0d6be61c1dd01fa7",
          "url": "https://github.com/cilladev/xlstream/commit/684a2606c19b33574a535e334aa670e4e0308021"
        },
        "date": 1776910268988,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 2",
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
            "value": 20,
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
            "value": 216,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 198,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 199,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20984,
            "range": "± 61",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "dc2ab7c8ee97667581dcd60fc8c53253f9ecb3f4",
          "message": "docs: remove benchmark automation item from v0.2 roadmap",
          "timestamp": "2026-04-23T13:43:20+01:00",
          "tree_id": "6ca67378bb277aff81cf2a9b1c8a954f166ab4a2",
          "url": "https://github.com/cilladev/xlstream/commit/dc2ab7c8ee97667581dcd60fc8c53253f9ecb3f4"
        },
        "date": 1776948818167,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 32,
            "range": "± 1",
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
            "value": 46,
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
            "value": 216,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 122,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 197,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 176,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 203,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 240,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 432,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 311,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 472,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 164,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 456,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1378,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 74,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 197,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 198,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 291,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 117,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 94,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 128,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20752,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 142,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 203,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 758,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 254,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 263,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 281,
            "range": "± 1",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "fbf9271e31c95b2e6dab0811574e206991101abc",
          "message": "docs: check completed items in roadmap v0.2",
          "timestamp": "2026-04-24T23:57:46+01:00",
          "tree_id": "d6f816813474500fbb588b71c5585a97a361d046",
          "url": "https://github.com/cilladev/xlstream/commit/fbf9271e31c95b2e6dab0811574e206991101abc"
        },
        "date": 1777072091881,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 49,
            "range": "± 1",
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
            "value": 230,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 72,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 216,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 185,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 91,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 166,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 216,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 252,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 448,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 338,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 494,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 163,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 170,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2357,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 470,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1396,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 87,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 220,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 218,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 300,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 117,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 94,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 131,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20433,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 158,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 235,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 740,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 247,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 293,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 283,
            "range": "± 5",
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
          "id": "41ccfe7abbb4488b457d96d359863a74bdf0d5f4",
          "message": "xlstream-eval: add empty builtin scaffolds for v0.3+ categories",
          "timestamp": "2026-04-25T00:14:48+01:00",
          "tree_id": "2c1ed0d2cd43d5762f342fdb5d7f5f5eaa3d4639",
          "url": "https://github.com/cilladev/xlstream/commit/41ccfe7abbb4488b457d96d359863a74bdf0d5f4"
        },
        "date": 1777073122813,
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
            "value": 34,
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
            "value": 34,
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
            "value": 216,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 199,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 175,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 84,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 207,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 241,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 429,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 312,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 472,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 159,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 466,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1377,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 198,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 198,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 290,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 118,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 96,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 129,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20560,
            "range": "± 469",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 140,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 206,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 764,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 257,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 264,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 274,
            "range": "± 1",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "14d2b66594b04876bc225cbae699a84bfea881f2",
          "message": "xlstream-eval: add MINIFS and MAXIFS conditional aggregates",
          "timestamp": "2026-04-25T12:38:46+01:00",
          "tree_id": "cb7e8105c52fcba8fd22ecfde379e1e07ec3fcfc",
          "url": "https://github.com/cilladev/xlstream/commit/14d2b66594b04876bc225cbae699a84bfea881f2"
        },
        "date": 1777117746794,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 33,
            "range": "± 1",
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
            "value": 34,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 75,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 75,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 120,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 197,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 174,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 152,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 205,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 240,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 415,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 308,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 471,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 169,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 454,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1372,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 73,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 196,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 195,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 301,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 114,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 94,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 94,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 128,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20912,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 139,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 203,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 769,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 253,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 266,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 266,
            "range": "± 7",
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
          "id": "3de104aef38a9c9d870551de75bbb226dd3db436",
          "message": "ci: raise bench-gate threshold to 20%",
          "timestamp": "2026-04-25T13:08:36+01:00",
          "tree_id": "afba7077401740500cf8f9a828b93d532f6be265",
          "url": "https://github.com/cilladev/xlstream/commit/3de104aef38a9c9d870551de75bbb226dd3db436"
        },
        "date": 1777119541481,
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
            "value": 35,
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
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 20,
            "range": "± 1",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 72,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 140,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 224,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 176,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 106,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 180,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 226,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 240,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 415,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 308,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 471,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 158,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 452,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1374,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 73,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 93,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 81,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 195,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 201,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 303,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 115,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 95,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21250,
            "range": "± 269",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 139,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 208,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 779,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 242,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 268,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 276,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "f4597874462cdb5d2c98f9e72a8cf46d6ff0bbcb",
          "message": "build(deps): bump criterion from 0.5.1 to 0.8.2\n\nBumps [criterion](https://github.com/criterion-rs/criterion.rs) from 0.5.1 to 0.8.2.\n- [Release notes](https://github.com/criterion-rs/criterion.rs/releases)\n- [Changelog](https://github.com/criterion-rs/criterion.rs/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/criterion-rs/criterion.rs/compare/0.5.1...criterion-v0.8.2)\n\n---\nupdated-dependencies:\n- dependency-name: criterion\n  dependency-version: 0.8.2\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2026-04-28T00:41:20+01:00",
          "tree_id": "bd9a08c5b1a19819e2033130c47541fff0078959",
          "url": "https://github.com/cilladev/xlstream/commit/f4597874462cdb5d2c98f9e72a8cf46d6ff0bbcb"
        },
        "date": 1777333938001,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 35,
            "range": "± 1",
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
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 56,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 226,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 77,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 77,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 193,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 174,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 80,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 150,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 207,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 209,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 384,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 308,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 412,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 159,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 451,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1376,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 80,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 89,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 190,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 190,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 295,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 110,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 112,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 123,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21659,
            "range": "± 624",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 209,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 797,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 236,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 264,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 277,
            "range": "± 5",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "priscillaemasoga@gmail.com",
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "a0982ab73e823a3f5f91ae606663bb6c62e5ad64",
          "message": "build(deps): bump softprops/action-gh-release from 2 to 3\n\nBumps [softprops/action-gh-release](https://github.com/softprops/action-gh-release) from 2 to 3.\n- [Release notes](https://github.com/softprops/action-gh-release/releases)\n- [Changelog](https://github.com/softprops/action-gh-release/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/softprops/action-gh-release/compare/v2...v3)\n\n---\nupdated-dependencies:\n- dependency-name: softprops/action-gh-release\n  dependency-version: '3'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2026-04-28T20:54:36+01:00",
          "tree_id": "318c464d7569ed07bf65115edb3448be0a397c0f",
          "url": "https://github.com/cilladev/xlstream/commit/a0982ab73e823a3f5f91ae606663bb6c62e5ad64"
        },
        "date": 1777406703324,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 227,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 122,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 200,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 183,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 164,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 209,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 211,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 415,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 331,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 426,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 161,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2374,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 470,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1392,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 73,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 111,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 237,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 237,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 296,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 116,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 19621,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 139,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 230,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 736,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 243,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 265,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 305,
            "range": "± 3",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "8117dd9709176c8a4971d48b29f4c2426e6726fb",
          "message": "test: improve operators, logical, math, text, lookup conformance fixtures",
          "timestamp": "2026-04-30T23:36:19+01:00",
          "tree_id": "3bc3186fe75206227511d479a3e0a2239c2a234f",
          "url": "https://github.com/cilladev/xlstream/commit/8117dd9709176c8a4971d48b29f4c2426e6726fb"
        },
        "date": 1777589193693,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 39,
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
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 190,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 180,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 67,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 191,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 199,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 365,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 413,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2319,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 403,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1415,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 162,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 246,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 70,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 103,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 18036,
            "range": "± 255",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 117,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 717,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 299,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 251,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 223,
            "range": "± 2",
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
          "id": "073c9aeb32e2b6f56d167e917b884918053914dd",
          "message": "ci: only run bench-gate on PRs, not main pushes",
          "timestamp": "2026-05-01T01:32:35+01:00",
          "tree_id": "8d1a25a279ed88c9a035d9c93032083985556b5a",
          "url": "https://github.com/cilladev/xlstream/commit/073c9aeb32e2b6f56d167e917b884918053914dd"
        },
        "date": 1777596188869,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 227,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 122,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 200,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 179,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 209,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 209,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 410,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 331,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 419,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2376,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 476,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1395,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 113,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 211,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 209,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 295,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 116,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 87,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 88,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 129,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20046,
            "range": "± 420",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 133,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 229,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 762,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 238,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 264,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 281,
            "range": "± 16",
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
          "id": "5d7f1e648113a155711e054cb54ef83c7c7a3137",
          "message": "docs: fix topo sort description — self-edges filtered before sort, cross-column cycles caught by topo sort",
          "timestamp": "2026-05-01T15:34:47+01:00",
          "tree_id": "32b957c91c687dc2d9a3770b793ab426225849bb",
          "url": "https://github.com/cilladev/xlstream/commit/5d7f1e648113a155711e054cb54ef83c7c7a3137"
        },
        "date": 1777646737789,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 227,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 124,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 203,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 179,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 166,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 211,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 213,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 413,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 333,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 424,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 165,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2380,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 481,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1397,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 77,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 112,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 82,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 209,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 210,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 296,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 87,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 131,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20095,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 228,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 760,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 240,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 269,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 285,
            "range": "± 2",
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "080945b2f621e08e3dcb7dc1236f49270408c843",
          "message": "docs: add ThreeDimensionalRef to evaluator dispatch comment",
          "timestamp": "2026-05-01T22:04:17+01:00",
          "tree_id": "d925b7c24a1725745152a634379208ca20bddd66",
          "url": "https://github.com/cilladev/xlstream/commit/080945b2f621e08e3dcb7dc1236f49270408c843"
        },
        "date": 1777670098089,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 57,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 239,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 125,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 208,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 183,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 90,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 163,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 215,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 211,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 410,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 334,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 419,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 152,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2354,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 465,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1394,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 82,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 77,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 112,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 84,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 233,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 232,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 295,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 116,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 90,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21585,
            "range": "± 684",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 145,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 230,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 758,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 238,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 264,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 286,
            "range": "± 14",
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
          "id": "d9aef65c6839a57aea8d8afe778a7ac927943aff",
          "message": "docs: add feature spec files",
          "timestamp": "2026-05-01T23:24:25+01:00",
          "tree_id": "6fc60f572a6071f3acf72657a14eb5c46e89875c",
          "url": "https://github.com/cilladev/xlstream/commit/d9aef65c6839a57aea8d8afe778a7ac927943aff"
        },
        "date": 1777674912333,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 215,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 118,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 207,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 172,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 81,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 158,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 213,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 208,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 387,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 306,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 406,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 162,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 450,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1390,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 80,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 217,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 217,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 308,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 93,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 83,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 125,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22329,
            "range": "± 219",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 135,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 205,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 763,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 233,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 258,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 269,
            "range": "± 3",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}