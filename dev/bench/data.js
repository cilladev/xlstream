window.BENCHMARK_DATA = {
  "lastUpdate": 1779116505741,
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
          "id": "373b07abe94ba4da8f3c5f65bd98a3108276b706",
          "message": "fix: update benchmarks and docs",
          "timestamp": "2026-05-11T10:00:39+01:00",
          "tree_id": "b2e2abedb3e3a3d9aef25172e100fc80556517a6",
          "url": "https://github.com/cilladev/xlstream/commit/373b07abe94ba4da8f3c5f65bd98a3108276b706"
        },
        "date": 1778490708499,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 211,
            "range": "± 1",
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
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 123,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 200,
            "range": "± 1",
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
            "value": 84,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 157,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 214,
            "range": "± 1",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 307,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 406,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 159,
            "range": "± 4",
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
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 448,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1375,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 84,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 93,
            "range": "± 1",
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
            "value": 198,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 196,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 276,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 109,
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
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22237,
            "range": "± 212",
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
            "value": 208,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 749,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 243,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 257,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 271,
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
          "id": "cff9b52de207648eb4f07379cdf70e61ed42d77d",
          "message": "xlstream-core: replace values_only bool with OutputMode enum\n\ncode-style says \"prefer enums over type-flag booleans.\"\nCLI --values-only flag and Python values_only kwarg unchanged\n(bool-to-enum conversion at the boundary).",
          "timestamp": "2026-05-11T14:47:55+01:00",
          "tree_id": "c4b50959aee887839bbf6801f31a91e4446fd688",
          "url": "https://github.com/cilladev/xlstream/commit/cff9b52de207648eb4f07379cdf70e61ed42d77d"
        },
        "date": 1778507906672,
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
            "value": 228,
            "range": "± 0",
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
            "value": 121,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 199,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 179,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 89,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 209,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 210,
            "range": "± 1",
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
            "value": 332,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 422,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 164,
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
            "value": 2356,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 469,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1395,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 83,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 111,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 96,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 210,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 212,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 297,
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 90,
            "range": "± 0",
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
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22012,
            "range": "± 518",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 149,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 225,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 750,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 237,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 261,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 283,
            "range": "± 1",
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
          "id": "78b3d4f02b9290aa6710895cb5061d589ceb2982",
          "message": "build(deps): bump rust_xlsxwriter from 0.94.0 to 0.95.0\n\nBumps [rust_xlsxwriter](https://github.com/jmcnamara/rust_xlsxwriter) from 0.94.0 to 0.95.0.\n- [Changelog](https://github.com/jmcnamara/rust_xlsxwriter/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/jmcnamara/rust_xlsxwriter/compare/v0.94.0...v0.95.0)\n\n---\nupdated-dependencies:\n- dependency-name: rust_xlsxwriter\n  dependency-version: 0.95.0\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2026-05-11T15:00:23+01:00",
          "tree_id": "46c2803e5c19418df5e0f5b785356a0ff883fdde",
          "url": "https://github.com/cilladev/xlstream/commit/78b3d4f02b9290aa6710895cb5061d589ceb2982"
        },
        "date": 1778508720139,
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
            "value": 78,
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
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 204,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 179,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 214,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 210,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 408,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 333,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 420,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 154,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2353,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 467,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1400,
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
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 75,
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 211,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 306,
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
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 89,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21164,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 140,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 224,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 758,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 237,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 268,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 292,
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
          "id": "eba1e007606d62f2516c3adebaedcfa9212e4b3c",
          "message": "xlstream-eval: bundle Arc-wrapped fields into SharedPlan struct",
          "timestamp": "2026-05-11T22:41:47+01:00",
          "tree_id": "2b5a55dbbc59427980a605a4e531f339ee471327",
          "url": "https://github.com/cilladev/xlstream/commit/eba1e007606d62f2516c3adebaedcfa9212e4b3c"
        },
        "date": 1778536336631,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 34,
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
            "range": "± 1",
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
            "value": 214,
            "range": "± 1",
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
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 118,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 195,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 176,
            "range": "± 3",
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
            "value": 153,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 210,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 210,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 392,
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
            "value": 410,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 451,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1376,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 94,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 197,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 196,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 304,
            "range": "± 9",
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
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 0",
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
            "value": 22325,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 201,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 768,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 260,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 269,
            "range": "± 4",
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
          "id": "79bffa19b6df575a2902f34f33f281f97992f847",
          "message": "docs: remove keep formulas docs",
          "timestamp": "2026-05-11T23:11:10+01:00",
          "tree_id": "724c4c1a4c10a015f50576dcfb792e07260c774f",
          "url": "https://github.com/cilladev/xlstream/commit/79bffa19b6df575a2902f34f33f281f97992f847"
        },
        "date": 1778538128733,
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
            "range": "± 0",
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
            "value": 78,
            "range": "± 0",
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
            "value": 207,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 180,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 167,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 216,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 210,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 408,
            "range": "± 1",
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
            "range": "± 8",
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
            "value": 158,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2352,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 463,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1399,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 81,
            "range": "± 2",
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
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 113,
            "range": "± 8",
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
            "value": 217,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 218,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 86,
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
            "value": 86,
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
            "value": 21344,
            "range": "± 286",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 226,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 780,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 243,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 269,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 284,
            "range": "± 8",
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
          "id": "0be96257d001695e9d43529f246b675e1ad5487f",
          "message": "docs: add row column docs",
          "timestamp": "2026-05-11T23:23:23+01:00",
          "tree_id": "26e59c5f14a2a60158fcbc1ab2c159708ec004d7",
          "url": "https://github.com/cilladev/xlstream/commit/0be96257d001695e9d43529f246b675e1ad5487f"
        },
        "date": 1778538828324,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 35,
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
            "value": 212,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 124,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 202,
            "range": "± 2",
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
            "value": 81,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 214,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 209,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 391,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 306,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 447,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 167,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 458,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1382,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 103,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 94,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 107,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 125,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 102,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 197,
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
            "name": "math/round",
            "value": 280,
            "range": "± 6",
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
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22936,
            "range": "± 385",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 135,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 203,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 1020,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 236,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 261,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 269,
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
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "1f7fa065250eddd4b0435f7a495a17879784815e",
          "message": "ci: reset Cargo.lock before bench-update gh-pages switch",
          "timestamp": "2026-05-12T00:21:04+01:00",
          "tree_id": "f43a5c6c8ac5f13d83c4f21a206ff47b8bf20743",
          "url": "https://github.com/cilladev/xlstream/commit/1f7fa065250eddd4b0435f7a495a17879784815e"
        },
        "date": 1778542288701,
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
            "value": 228,
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
            "value": 126,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 211,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 180,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 167,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 213,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 186,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 381,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 330,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 364,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 162,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 157,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2352,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 466,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1393,
            "range": "± 3",
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
            "value": 70,
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
            "value": 112,
            "range": "± 1",
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
            "value": 214,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 224,
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
            "value": 125,
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
            "value": 92,
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
            "value": 132,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 20945,
            "range": "± 596",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 142,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 242,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 742,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 237,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 276,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 290,
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
          "id": "660dcd0cfb6c697e10944d4d7c39425c68868677",
          "message": "docs: fix stale counts in roadmap files",
          "timestamp": "2026-05-12T23:25:12+01:00",
          "tree_id": "667a692094e38ab809e01532bfe46d5c6004daad",
          "url": "https://github.com/cilladev/xlstream/commit/660dcd0cfb6c697e10944d4d7c39425c68868677"
        },
        "date": 1778625334961,
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
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 58,
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
            "value": 212,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
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
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 191,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 173,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 153,
            "range": "± 2",
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
            "value": 196,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 377,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 309,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 386,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 462,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1371,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 1",
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
            "value": 87,
            "range": "± 0",
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
            "value": 191,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 192,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 282,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
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
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 81,
            "range": "± 4",
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
            "value": 22191,
            "range": "± 216",
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
            "value": 201,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 760,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 264,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 264,
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
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "2d4b66abff449c9cba9432ad75746f7dcb5b3450",
          "message": "docs: fix stale docs",
          "timestamp": "2026-05-13T00:32:59+01:00",
          "tree_id": "aa04cbb9cf8bc0ae8c25ba868c43a3ecd9601ea4",
          "url": "https://github.com/cilladev/xlstream/commit/2d4b66abff449c9cba9432ad75746f7dcb5b3450"
        },
        "date": 1778629460202,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 1",
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
            "value": 210,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 192,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 153,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 197,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 377,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 309,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 385,
            "range": "± 2",
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
            "value": 156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2105,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 447,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1367,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
            "range": "± 0",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 88,
            "range": "± 0",
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
            "value": 194,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 194,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 281,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 113,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 91,
            "range": "± 1",
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
            "value": 82,
            "range": "± 1",
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
            "value": 22226,
            "range": "± 278",
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
            "value": 201,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 767,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 264,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 272,
            "range": "± 4",
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
          "id": "5844426b8f877ce4694fbab0eb14706902ce889f",
          "message": "docs: make design goal concrete, update remaining 700k references\n\n- CLAUDE.md: \"evaluate 100k x 50 in <15s at <250 MB\" (was vague)\n- v0.2/v0.3 roadmap: memory target updated to 100k baseline (643 MB)",
          "timestamp": "2026-05-13T10:55:47+01:00",
          "tree_id": "222b31b71433cf37b0702be2f47b854eb298553c",
          "url": "https://github.com/cilladev/xlstream/commit/5844426b8f877ce4694fbab0eb14706902ce889f"
        },
        "date": 1778666781077,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 210,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
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
            "value": 117,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 194,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 171,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 1",
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
            "value": 210,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 196,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 376,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 309,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 385,
            "range": "± 2",
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
            "value": 158,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2100,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 450,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1373,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 2",
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
            "value": 88,
            "range": "± 1",
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
            "value": 250,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 252,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 281,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 112,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 91,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 126,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22023,
            "range": "± 254",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 202,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 764,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 238,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 264,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 267,
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
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "a134c38364c89fa64cb03c63000f86565f63f15c",
          "message": "docs: add multi-format I/O to v0.4 roadmap\n\nInput: .xlsm, .xltx, .xltm, .xlam (free), .xlsb (streaming)\nOutput: .csv (data extraction), .xlsm (macro passthrough)",
          "timestamp": "2026-05-13T11:09:58+01:00",
          "tree_id": "4400084fdc5e7d98565143b623b8d8fecfde6d75",
          "url": "https://github.com/cilladev/xlstream/commit/a134c38364c89fa64cb03c63000f86565f63f15c"
        },
        "date": 1778667636936,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 30,
            "range": "± 1",
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
            "value": 192,
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
            "value": 178,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 160,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 65,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 159,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 189,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 166,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 342,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 349,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 136,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 132,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2314,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 403,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1407,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 50,
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
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 65,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 162,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 161,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 252,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 69,
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
            "value": 64,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 19764,
            "range": "± 489",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 116,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 734,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 308,
            "range": "± 6",
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
            "value": 219,
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
          "id": "255fae6f93ecd3a9a162708d89a75e6bdd3892dc",
          "message": "docs: update headline to LibreOffice comparison, fix Excel row format",
          "timestamp": "2026-05-13T11:20:52+01:00",
          "tree_id": "aa22adb153119ab94dced8c845adf04400d6eda5",
          "url": "https://github.com/cilladev/xlstream/commit/255fae6f93ecd3a9a162708d89a75e6bdd3892dc"
        },
        "date": 1778668286645,
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
            "range": "± 3",
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
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 202,
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
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 212,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 185,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 382,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 330,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 364,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 163,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 162,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2350,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 470,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1413,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 80,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 75,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 110,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 86,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 214,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 215,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 302,
            "range": "± 2",
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
            "value": 88,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 90,
            "range": "± 6",
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
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21263,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 136,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 228,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 744,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 236,
            "range": "± 3",
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
            "value": 285,
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
          "id": "46f97247da833b59fd7aa3c1a54328b356390e91",
          "message": "docs: archive v0.2, start v0.3 development",
          "timestamp": "2026-05-13T12:14:31+01:00",
          "tree_id": "3a5749aeec803237f6352eadc26232873b42a484",
          "url": "https://github.com/cilladev/xlstream/commit/46f97247da833b59fd7aa3c1a54328b356390e91"
        },
        "date": 1778671500937,
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
            "value": 34,
            "range": "± 2",
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
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 210,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 119,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 195,
            "range": "± 1",
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
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 152,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 203,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 382,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 309,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 398,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 448,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1378,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
            "range": "± 0",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 192,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 192,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 281,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 86,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22443,
            "range": "± 364",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 203,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 759,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 265,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 269,
            "range": "± 6",
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
          "id": "0f4fee24e0f2594c6dfd82d7524ae83a0b99de89",
          "message": "docs: add feature specs for descriptive statistics\n\n02-stdev-var: STDEV.S/P + VAR.S/P (27 conformance formulas)\n03-avedev: AVEDEV (17 conformance formulas)\n04-skew-kurt: SKEW/SKEW.P + KURT (25 conformance formulas)",
          "timestamp": "2026-05-13T12:25:25+01:00",
          "tree_id": "c24b4aeca0c5132298af7b3c3b9d0a3887d8fd90",
          "url": "https://github.com/cilladev/xlstream/commit/0f4fee24e0f2594c6dfd82d7524ae83a0b99de89"
        },
        "date": 1778672161851,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 34,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 45,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 47,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/negate",
            "value": 23,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/percent",
            "value": 38,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 211,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 84,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 12",
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
            "value": 194,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 171,
            "range": "± 2",
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
            "value": 153,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 205,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 197,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 379,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 312,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 386,
            "range": "± 3",
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
            "value": 158,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2100,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 447,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1369,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
            "range": "± 0",
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
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 88,
            "range": "± 0",
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
            "value": 199,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 200,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 282,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
            "range": "± 2",
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
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22172,
            "range": "± 2145",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 201,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 807,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 239,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 262,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 271,
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
          "id": "aeae6aced8ae50d5240c2139e66288b85d48846e",
          "message": "xlstream-eval: document #NUM! error path in stdev_s/stdev_p, note NaN passthrough in collect_numerics",
          "timestamp": "2026-05-13T13:17:23+01:00",
          "tree_id": "390d41e72fb5d38f348d8bed475abbf0c2d831a3",
          "url": "https://github.com/cilladev/xlstream/commit/aeae6aced8ae50d5240c2139e66288b85d48846e"
        },
        "date": 1778675397688,
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
            "value": 34,
            "range": "± 0",
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 212,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
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
            "value": 116,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 202,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 156,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 204,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 197,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 385,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 310,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 387,
            "range": "± 9",
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
            "value": 157,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 449,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1378,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 90,
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 201,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 203,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 280,
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
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 94,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 0",
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
            "value": 21805,
            "range": "± 325",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 140,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 201,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 775,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 237,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 260,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 273,
            "range": "± 4",
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
          "id": "543a3aefca938cc62c04a7700b2136e95ecc904a",
          "message": "xlstream-eval: use finite_or_num, add overflow tests for SKEW/KURT\n\n- replace inline NaN/Infinity checks with finite_or_num (consistency\n  with var_s/var_p)\n- add overflow/infinity tests for skew, skew_p, kurt",
          "timestamp": "2026-05-13T13:44:49+01:00",
          "tree_id": "95f9e6b3ea8832c9f050e5962c5321800a966726",
          "url": "https://github.com/cilladev/xlstream/commit/543a3aefca938cc62c04a7700b2136e95ecc904a"
        },
        "date": 1778676989098,
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
            "value": 228,
            "range": "± 1",
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
            "value": 127,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 201,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 179,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 93,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 215,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 187,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 381,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 333,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 369,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 152,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2350,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 462,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1398,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 85,
            "range": "± 0",
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
            "value": 111,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 48,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 64,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 214,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 216,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 303,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 88,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 90,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 87,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 129,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21514,
            "range": "± 409",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 132,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 231,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 737,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 236,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 262,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 286,
            "range": "± 4",
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
          "id": "8a19f7783725532e77c22c3bb2609c240a642634",
          "message": "docs: add feature spec for additional funcs",
          "timestamp": "2026-05-13T14:15:01+01:00",
          "tree_id": "b2485eb61c443fa0dc50ed4697f3331d76e00aa3",
          "url": "https://github.com/cilladev/xlstream/commit/8a19f7783725532e77c22c3bb2609c240a642634"
        },
        "date": 1778678835148,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 210,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
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
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 200,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 173,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 82,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 154,
            "range": "± 1",
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
            "value": 197,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 377,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 313,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 387,
            "range": "± 2",
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
            "value": 158,
            "range": "± 1",
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
            "value": 482,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1406,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
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
            "value": 88,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 55,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 189,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 190,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 280,
            "range": "± 2",
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
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 79,
            "range": "± 0",
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
            "value": 22484,
            "range": "± 340",
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
            "value": 200,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 801,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 239,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 260,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 316,
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "9636669f54393cbb6a29be1f6ffaa882d1630542",
          "message": "xlstream-parse: fix leftover conflict marker in sets.rs test",
          "timestamp": "2026-05-13T15:16:48+01:00",
          "tree_id": "a497cdbb8bf70db4525293a5fdb381b4be932857",
          "url": "https://github.com/cilladev/xlstream/commit/9636669f54393cbb6a29be1f6ffaa882d1630542"
        },
        "date": 1778682547636,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 225,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 119,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 200,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 172,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 165,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 215,
            "range": "± 3",
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
            "value": 386,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 313,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 410,
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
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2099,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 454,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1365,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 71,
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
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 207,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 205,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 283,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
            "range": "± 0",
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
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 125,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22149,
            "range": "± 529",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 138,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 202,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 758,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 262,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 274,
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
          "id": "8835a012ec5aa9824d7b5823786d834393ad936f",
          "message": "xlstream-eval: add rank_avg empty test, update module doc",
          "timestamp": "2026-05-13T15:42:11+01:00",
          "tree_id": "52842c3dd20b19efd7531dab9173ece7867b1c8a",
          "url": "https://github.com/cilladev/xlstream/commit/8835a012ec5aa9824d7b5823786d834393ad936f"
        },
        "date": 1778684048004,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 213,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 3",
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
            "value": 195,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 171,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 208,
            "range": "± 0",
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
            "value": 312,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 409,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2099,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 450,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1369,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 79,
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 61,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 212,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 206,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 292,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 110,
            "range": "± 0",
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
            "value": 87,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 81,
            "range": "± 0",
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
            "value": 22344,
            "range": "± 198",
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
            "value": 201,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 756,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 259,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 278,
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
          "id": "edb19080ac2c77a0c3a959b8941e3d2bf734d812",
          "message": "docs: add feature spec for additonal funcs",
          "timestamp": "2026-05-13T16:03:15+01:00",
          "tree_id": "f9eb3030055ab895c09a3f0b1ab3ad12c9fac07e",
          "url": "https://github.com/cilladev/xlstream/commit/edb19080ac2c77a0c3a959b8941e3d2bf734d812"
        },
        "date": 1778685299341,
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
            "value": 38,
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
            "value": 199,
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
            "value": 105,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 178,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 163,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 63,
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
            "value": 188,
            "range": "± 4",
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
            "value": 364,
            "range": "± 4",
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
            "value": 412,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 132,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2346,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 403,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1409,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 66,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 53,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 63,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 67,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 45,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 44,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 168,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 166,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 248,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 68,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 63,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 19729,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 119,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 162,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 722,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 297,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 262,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 221,
            "range": "± 0",
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
          "id": "76b0970ef6676989d0aa139fe365a9f197728398",
          "message": "xlstream-eval: refactor EXPON.DIST to Result return type, add edge tests",
          "timestamp": "2026-05-13T16:37:47+01:00",
          "tree_id": "051374a82deb1d623323571661cf4ae6bbb7ffa3",
          "url": "https://github.com/cilladev/xlstream/commit/76b0970ef6676989d0aa139fe365a9f197728398"
        },
        "date": 1778687367325,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
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
            "value": 211,
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
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 194,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 156,
            "range": "± 4",
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
            "value": 208,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 387,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 310,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 410,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 160,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2104,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 452,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1382,
            "range": "± 2",
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
            "value": 72,
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
            "value": 90,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 61,
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
            "value": 189,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 284,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
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
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 125,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22287,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 159,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 205,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 777,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 203,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 257,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 272,
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
          "id": "46ae4b969c124c0e0eaa88eee4301e55339b1e9c",
          "message": "docs: add feature spec for additonal funcs",
          "timestamp": "2026-05-13T17:03:56+01:00",
          "tree_id": "f9b0180396c213bae190cf92636eb9431006853f",
          "url": "https://github.com/cilladev/xlstream/commit/46ae4b969c124c0e0eaa88eee4301e55339b1e9c"
        },
        "date": 1778688962856,
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
            "value": 44,
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
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 175,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 156,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 139,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 126,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 165,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 190,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 338,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 277,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 363,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 127,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 121,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 1815,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 366,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1113,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 43,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 38,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 167,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 167,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 234,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 69,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 69,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 65,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 103,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 17059,
            "range": "± 406",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 177,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 582,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 186,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 202,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 223,
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
          "id": "f524a5051f7945bfb0771221a5726aea0fe084cd",
          "message": "xlstream-eval: address PR #114 review feedback\n\n- reject non-finite x/mean in builtin_norm_dist (C1)\n- reject non-finite mean in builtin_norm_inv (C1)\n- add coercion tests: Text(\"0.5\"), Number(1.0) as bool (C2)\n- add type mismatch tests: Text(\"abc\") (C3)\n- fix relative import to use crate::builtins::math (I1)\n- update module doc to list NORM.DIST/NORM.INV (I2)\n- add upper-tail test p=0.999 exercising high branch (I3)\n- update Errors rustdoc for non-finite inputs (I4)\n- add stdev=INFINITY, NaN p, mean=INFINITY tests (I5)\n- fix norm_inv_standard accuracy claim to ~4e-9 (S2)\n- fix bool_arg_ce doc (S4), add SQRT_2PI doc (S1)\n- reorder erf tests before NORM.DIST tests (S5)",
          "timestamp": "2026-05-13T17:34:14+01:00",
          "tree_id": "5562971d72780b79ba36bf2b35f4aabad0b6819c",
          "url": "https://github.com/cilladev/xlstream/commit/f524a5051f7945bfb0771221a5726aea0fe084cd"
        },
        "date": 1778690758965,
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
            "range": "± 1",
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
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 213,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 117,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 194,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 153,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 205,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 213,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 413,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 314,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 411,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2079,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 451,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1378,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 83,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 1",
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
            "value": 87,
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
            "name": "info/row_ref",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 61,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 197,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 196,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 289,
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
            "range": "± 1",
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
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 125,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22004,
            "range": "± 150",
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
            "value": 201,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 788,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 236,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 256,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 265,
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
          "id": "88a69e91cec357f2641a9896e0d23e32e591cb9d",
          "message": "xlstream-eval: add error cases to norm_s_dist conformance fixture\n\nadds NORM.S.INV(0), NORM.S.INV(1), NORM.S.INV(-0.5), NORM.S.INV(1.5)\nerror test cases. needs Excel recalculation before conformance passes.",
          "timestamp": "2026-05-13T18:10:18+01:00",
          "tree_id": "4bc5cdc94db67a9c41fa166d7304578254fd5bb3",
          "url": "https://github.com/cilladev/xlstream/commit/88a69e91cec357f2641a9896e0d23e32e591cb9d"
        },
        "date": 1778692918543,
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 214,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 2",
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
            "value": 118,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 195,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 77,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 156,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 4",
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
            "value": 389,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 314,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 425,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 155,
            "range": "± 0",
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
            "value": 453,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1390,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 78,
            "range": "± 0",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 61,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 194,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 192,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 286,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 92,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 87,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 85,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 126,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22203,
            "range": "± 315",
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
            "value": 204,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 850,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
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
            "value": 270,
            "range": "± 4",
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
          "id": "90dcb2aa60de8d9854cbde0e793ae243b1f04a4d",
          "message": "xlstream-eval: fix correl intermediate overflow, exhaustive match, doc gaps",
          "timestamp": "2026-05-13T19:18:33+01:00",
          "tree_id": "100a2ac1468d4ba172f272fa0243ed08144ca50d",
          "url": "https://github.com/cilladev/xlstream/commit/90dcb2aa60de8d9854cbde0e793ae243b1f04a4d"
        },
        "date": 1778697018878,
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
            "value": 78,
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
            "value": 122,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 201,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 180,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 85,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 165,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 210,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 410,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 332,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 418,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2353,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 480,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1471,
            "range": "± 15",
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
            "value": 70,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 76,
            "range": "± 0",
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
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 41,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 226,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 228,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 303,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 115,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 88,
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
            "value": 84,
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
            "value": 21543,
            "range": "± 222",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 136,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 223,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 723,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 242,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 266,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 288,
            "range": "± 12",
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
          "id": "afb01892fbdd98bf73ebe450982520016c87c3b9",
          "message": "xlstream-eval: address PR #117 review feedback",
          "timestamp": "2026-05-13T19:53:56+01:00",
          "tree_id": "4ed7a7bd900c5f365b447b786ebd2de25a4883c1",
          "url": "https://github.com/cilladev/xlstream/commit/afb01892fbdd98bf73ebe450982520016c87c3b9"
        },
        "date": 1778699138508,
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
            "value": 228,
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 203,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 181,
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
            "value": 165,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 210,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 408,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 332,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 418,
            "range": "± 5",
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
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2348,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 471,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1391,
            "range": "± 3",
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
            "value": 85,
            "range": "± 3",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 56,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 216,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 218,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 303,
            "range": "± 3",
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
            "value": 90,
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
            "range": "± 1",
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
            "value": 21324,
            "range": "± 257",
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
            "value": 235,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 757,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 239,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 274,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 284,
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
          "id": "277a72fa3ec83f4b10a01f09b644f5a1acc29fd6",
          "message": "xlstream-eval: add SLOPE, INTERCEPT, RSQ, FORECAST.LINEAR on top of main",
          "timestamp": "2026-05-13T20:30:52+01:00",
          "tree_id": "b7b77e566ecc3f13e3d6a644ff19da4bba40cfe7",
          "url": "https://github.com/cilladev/xlstream/commit/277a72fa3ec83f4b10a01f09b644f5a1acc29fd6"
        },
        "date": 1778701361532,
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
            "value": 228,
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
            "value": 228,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 181,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 166,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 209,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 210,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 410,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 334,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 420,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 158,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 155,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2349,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 474,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1399,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 83,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 78,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 111,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 84,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 55,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 42,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 218,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 214,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 300,
            "range": "± 4",
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
            "value": 90,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 87,
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
            "value": 128,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21161,
            "range": "± 373",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 131,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 240,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 739,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 243,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 302,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 306,
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
            "name": "Priscillla",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "0ce8024264e3cae0bd02f73b7fe6844f8cdcab5b",
          "message": "xlstream-eval: address PR #120 review feedback\n\n- add to_int helper: guards f64→i64 truncation against NaN, infinity,\n  and out-of-range values (was silently saturating)\n- use checked arithmetic in COMBINA to prevent i64 overflow panic\n- add missing coercion/type-mismatch tests for all 6 functions\n- add negative-k tests for PERMUTATIONA and COMBINA\n- add fractional-truncation tests for PERMUTATIONA and COMBINA\n- remove unnecessary cast_sign_loss allows from FACT/FACTDOUBLE\n- add precondition doc to combin_core\n- fix roadmap: PERMUT / PERMUTA → PERMUT / PERMUTATIONA",
          "timestamp": "2026-05-13T21:36:11+01:00",
          "tree_id": "67fa9bbf4096614c592431b99e4decdabeac4266",
          "url": "https://github.com/cilladev/xlstream/commit/0ce8024264e3cae0bd02f73b7fe6844f8cdcab5b"
        },
        "date": 1778705277777,
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
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 144,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 229,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 216,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 111,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 182,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 222,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 212,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 408,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 334,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 421,
            "range": "± 1",
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
            "value": 158,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2352,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 464,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1391,
            "range": "± 5",
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
            "value": 72,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 78,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 115,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 82,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 55,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 60,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 38,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 43,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 213,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 214,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 301,
            "range": "± 6",
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
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 86,
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
            "value": 127,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21479,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 137,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 232,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 824,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 252,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 273,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 286,
            "range": "± 11",
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
          "id": "e2086d79b72a930396f75270b4ed4c0d90130939",
          "message": "xlstream-eval: address PR #121 review feedback",
          "timestamp": "2026-05-13T22:43:57+01:00",
          "tree_id": "53680f09279efc4caccfca5580d22e47de50ae6e",
          "url": "https://github.com/cilladev/xlstream/commit/e2086d79b72a930396f75270b4ed4c0d90130939"
        },
        "date": 1778709336062,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 35,
            "range": "± 2",
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
            "value": 35,
            "range": "± 1",
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
            "value": 54,
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
            "value": 211,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 4",
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
            "value": 205,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 171,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 159,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 206,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 198,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 377,
            "range": "± 2",
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
            "value": 385,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 163,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2101,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 449,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1366,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 81,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 75,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 95,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 82,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 52,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 62,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 206,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 203,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 284,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 91,
            "range": "± 0",
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
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 126,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21928,
            "range": "± 353",
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
            "value": 202,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 794,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 240,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 262,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 271,
            "range": "± 4",
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
          "id": "604105868c7528f816204f501caef9abbfd9df07",
          "message": "xlstream-eval: address PR #122 review feedback\n\n- add is_finite() guard on COMPLEX real/imag args\n- add is_finite() check to format_number (match coerce version)\n- remove trim() from parse_complex (Excel rejects whitespace)\n- add error propagation test for suffix arg\n- add non-finite input tests for COMPLEX\n- add scientific notation in imaginary part test\n- add comment on find_imag_split i > 0 guard",
          "timestamp": "2026-05-14T00:15:48+01:00",
          "tree_id": "f783cf3ae613a8e9124b03d5fdd1c1b7caad45d0",
          "url": "https://github.com/cilladev/xlstream/commit/604105868c7528f816204f501caef9abbfd9df07"
        },
        "date": 1778714847902,
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
            "value": 42,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 212,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 77,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 77,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 123,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 192,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 172,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 82,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 152,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 211,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 244,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 440,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 309,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 441,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 449,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1375,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 97,
            "range": "± 6",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 93,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 80,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 63,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 218,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 224,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 281,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 111,
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
            "value": 87,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 81,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22558,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 142,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 217,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 819,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 243,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 268,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 271,
            "range": "± 6",
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
          "id": "f7b40ef8f7276e145edbfa28b704f38a18370dca",
          "message": "xlstream-eval: address PR #123 review feedback",
          "timestamp": "2026-05-14T00:28:42+01:00",
          "tree_id": "bf4658c5aa0c127affe190dc6bc97465533145a9",
          "url": "https://github.com/cilladev/xlstream/commit/f7b40ef8f7276e145edbfa28b704f38a18370dca"
        },
        "date": 1778715607332,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/subtract",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 213,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 117,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 200,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 171,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 152,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 212,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 199,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 378,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 311,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 461,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 1",
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
            "value": 2100,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 451,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1369,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 69,
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
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 192,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 192,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 282,
            "range": "± 2",
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
            "value": 86,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 23198,
            "range": "± 607",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 201,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 771,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 238,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 260,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 276,
            "range": "± 4",
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
          "id": "4a60337d2b46361ced8b9055306dbfaecf452901",
          "message": "xlstream-eval: address PR #124 review feedback\n\n- replace unwrap_or_default with explicit error return in BASE\n- add explicit f64 bound check in extract_places before cast\n- add 8 missing error propagation + arity tests for cross-base\n- add 5 numeric coercion tests for cross-base functions\n- add places arg error propagation tests for DEC2BIN/DEC2OCT\n- add BASE error propagation in min_length and min_length=0 test",
          "timestamp": "2026-05-14T00:53:07+01:00",
          "tree_id": "38cf5e65b22ab2157dc5fae823e67319c6f55a07",
          "url": "https://github.com/cilladev/xlstream/commit/4a60337d2b46361ced8b9055306dbfaecf452901"
        },
        "date": 1778717075016,
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
            "range": "± 1",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
            "range": "± 2",
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
            "value": 214,
            "range": "± 1",
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
            "value": 120,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 206,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 174,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 159,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 213,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 212,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 376,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 313,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 385,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2102,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 463,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1373,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
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
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 62,
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
            "value": 192,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 285,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 110,
            "range": "± 2",
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
            "value": 90,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22378,
            "range": "± 491",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 199,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 791,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 239,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 260,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 264,
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
          "id": "c221403cfbbebe49b57a7f19fe971496e34256c2",
          "message": "xlstream-eval: address PR #125 review feedback",
          "timestamp": "2026-05-14T01:09:02+01:00",
          "tree_id": "255a7f021d47ebea525e53c37b1f47223d26b96d",
          "url": "https://github.com/cilladev/xlstream/commit/c221403cfbbebe49b57a7f19fe971496e34256c2"
        },
        "date": 1778718033190,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 230,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 118,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 197,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 0",
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
            "value": 155,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 213,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 201,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 378,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 320,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 390,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 161,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 157,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2105,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 451,
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
            "value": 79,
            "range": "± 0",
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
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 197,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 196,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 281,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 109,
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
            "value": 84,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22066,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 204,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 761,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 237,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 268,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 270,
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
          "id": "12b6fecbf4e641698f9ddc0bf26d1e710270ac6b",
          "message": "docs: fix kbyte test case in CONVERT spec (SI k=1000, not 1024)",
          "timestamp": "2026-05-14T01:48:21+01:00",
          "tree_id": "5a9b39263dcae99b84f67cc908ae13d784854bff",
          "url": "https://github.com/cilladev/xlstream/commit/12b6fecbf4e641698f9ddc0bf26d1e710270ac6b"
        },
        "date": 1778720396469,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/multiply",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/divide",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 53,
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
            "value": 212,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
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
            "value": 117,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 201,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
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
            "value": 211,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 201,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 380,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 319,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 390,
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
            "value": 156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2088,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 451,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1378,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 79,
            "range": "± 0",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 62,
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
            "value": 205,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 283,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 109,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 84,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 23553,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 201,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 770,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 236,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 263,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 278,
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
          "id": "a4f3e925da24927fc4fe93221a2cfbcc3f226517",
          "message": "xlstream-eval: add full CONVERT conformance coverage\n\nInclude all 39 formulas: factor-dependent conversions (stone, ozm,\nBTU, HP, atm->psi), cross-category errors, and unknown unit errors.\nLO agrees with our values within epsilon on all cells.",
          "timestamp": "2026-05-14T21:22:00+01:00",
          "tree_id": "050d4f2e369fa065924103669d9a5e526ba0bd37",
          "url": "https://github.com/cilladev/xlstream/commit/a4f3e925da24927fc4fe93221a2cfbcc3f226517"
        },
        "date": 1778790813982,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/add",
            "value": 30,
            "range": "± 3",
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
            "range": "± 1",
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
            "value": 198,
            "range": "± 4",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 184,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 161,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 157,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 196,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 168,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 335,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 351,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 134,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 132,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2322,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 404,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1413,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 61,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 73,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 160,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 249,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 67,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 19676,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 119,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 765,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 298,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 267,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 222,
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
            "name": "Priscilla Emasoga",
            "username": "cilladev"
          },
          "distinct": true,
          "id": "d92af95f8f78676f9b921ef19bd081b4ad0aa8f1",
          "message": "docs: add feature specs for roadmap items",
          "timestamp": "2026-05-16T14:34:28+01:00",
          "tree_id": "c7dad7a65b193228d298ddf12ff4553048bcf543",
          "url": "https://github.com/cilladev/xlstream/commit/d92af95f8f78676f9b921ef19bd081b4ad0aa8f1"
        },
        "date": 1778939166465,
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
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "value": 212,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 117,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 195,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 174,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 198,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 377,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 313,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 398,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2105,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 452,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1370,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 2",
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
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 197,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 196,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 281,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 113,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 124,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21639,
            "range": "± 310",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 136,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 196,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 804,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 236,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 263,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 265,
            "range": "± 4",
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
          "id": "7b25afe413fecf8f47b914f8c1c056eae6cee2e4",
          "message": "xlstream-eval: deterministic topo sort for cross-column deps\n\nSort deps Vec before passing to topo_sort so column evaluation\norder is consistent across platforms. HashMap/HashSet iteration\norder is non-deterministic, which caused cross-column same-row\nreferences (e.g. RATE(B6,F6,C6) depending on PMT in F6) to\nproduce different results on macOS vs Linux/Windows.\n\nRestores I6 cross-column dependency fixture that was previously\nremoved due to this bug.\n\nCloses #129",
          "timestamp": "2026-05-16T15:23:06+01:00",
          "tree_id": "54763e5a126df4ca7a2e29514ad0d3aecae2c2d6",
          "url": "https://github.com/cilladev/xlstream/commit/7b25afe413fecf8f47b914f8c1c056eae6cee2e4"
        },
        "date": 1778942083723,
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
            "value": 226,
            "range": "± 0",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 123,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 200,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 180,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 210,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 186,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 384,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 335,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 372,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 169,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 164,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2375,
            "range": "± 36",
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
            "value": 1395,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 81,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 77,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/istext",
            "value": 115,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 82,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 53,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 59,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 209,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 209,
            "range": "± 0",
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
            "value": 115,
            "range": "± 5",
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
            "value": 87,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 82,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 126,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 21902,
            "range": "± 505",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 223,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 734,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 255,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 265,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 312,
            "range": "± 11",
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
          "id": "c31889fe8951e79f787ab1ffb87f22c7edca9233",
          "message": "xlstream-eval: add NaN/Infinity guards and missing test shapes\n\nAll 9 rounding functions now reject non-finite inputs with #NUM!.\nTRUNC clamps digits to [-18, 28] to prevent i32 overflow panic.\nAdds 5 missing test shapes (FLOOR.MATH coercion/type, CEILING.PRECISE\ncoercion/type, FLOOR.PRECISE type) and 19 NaN/Infinity guard tests.",
          "timestamp": "2026-05-18T14:56:37+01:00",
          "tree_id": "71b61b4395a163d17603fd9f8f970dddf9fd586d",
          "url": "https://github.com/cilladev/xlstream/commit/c31889fe8951e79f787ab1ffb87f22c7edca9233"
        },
        "date": 1779113284478,
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
            "value": 34,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/power",
            "value": 54,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/concat",
            "value": 214,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 117,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 256,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/switch",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/iferror",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 152,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 208,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 237,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 414,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/networkdays",
            "value": 310,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "date/datedif",
            "value": 496,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 160,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2106,
            "range": "± 6",
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
            "value": 1374,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 70,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/type",
            "value": 76,
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 196,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 196,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 282,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 91,
            "range": "± 0",
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
            "value": 81,
            "range": "± 0",
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
            "value": 22025,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 183,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 267,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 770,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 279,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 317,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 307,
            "range": "± 20",
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
          "id": "2d49f1f88c9ce047b0d576de8107b729bef62230",
          "message": "xlstream-eval: address review suggestions for GCD/LCM/ROMAN/ARABIC",
          "timestamp": "2026-05-18T15:50:13+01:00",
          "tree_id": "1ed4c7a533f8409fb22f5dc7b056d4189eeb6ed5",
          "url": "https://github.com/cilladev/xlstream/commit/2d49f1f88c9ce047b0d576de8107b729bef62230"
        },
        "date": 1779116505384,
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
            "range": "± 1",
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
            "value": 226,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_gt",
            "value": 79,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "arithmetic/compare_eq",
            "value": 79,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/if",
            "value": 124,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/ifs",
            "value": 204,
            "range": "± 10",
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
            "value": 84,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/and",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "conditional/or",
            "value": 215,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "date/year",
            "value": 244,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "date/edate",
            "value": 442,
            "range": "± 8",
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
            "value": 489,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "financial/pmt",
            "value": 210,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "financial/fv",
            "value": 156,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "financial/rate",
            "value": 2361,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "financial/npv",
            "value": 473,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "financial/irr",
            "value": 1406,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "info/isblank",
            "value": 80,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "info/isnumber",
            "value": 68,
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
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/iserror",
            "value": 82,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_ref",
            "value": 62,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_ref",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/row_no_arg",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/column_no_arg",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "info/rows_range",
            "value": 50,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "info/columns_range",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/1000",
            "value": 214,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "lookup/vlookup_exact/10000",
            "value": 213,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "math/round",
            "value": 305,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/mod",
            "value": 118,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "math/sqrt",
            "value": 88,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/abs",
            "value": 87,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "math/int",
            "value": 88,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "math/power",
            "value": 127,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_30_formulas",
            "value": 22213,
            "range": "± 327",
            "unit": "ns/iter"
          },
          {
            "name": "string/left",
            "value": 136,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/concat",
            "value": 222,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "string/text",
            "value": 723,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/substitute",
            "value": 235,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "string/find",
            "value": 262,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "string/textjoin",
            "value": 285,
            "range": "± 5",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}