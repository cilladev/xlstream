window.BENCHMARK_DATA = {
  "lastUpdate": 1776910269341,
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
      }
    ]
  }
}