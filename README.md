# `tc-yoink`

"Yoinks" task definition and artifacts and populates a `work_dir` for consumption by [`scriptworker`](https://github.com/mozilla-releng/scriptworker/).
I built this so I could run tasks locally and see how they behave with my local changes.

## Usage

```
$ tc-yoink V9qK-ubTSA2WxMh_mOD1Qg
Downloading: public/target.x86.apk
Downloading: public/target.arm.apk
Downloading: public/target.aarch64.apk

$ tree work_dir
work_dir
├── cot
│   └── TaCFyb8aRlOcxB5sgBDe4Q
│       └── public
│           ├── target.aarch64.apk
│           ├── target.arm.apk
│           └── target.x86.apk
└── task.json

```