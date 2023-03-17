
# ICD Code Converter

![GitHub release (latest by date)](https://img.shields.io/github/v/release/CogDisResLab/icd-converter)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/CogDisResLab/icd-converter)
[![Continuous integration](https://github.com/CogDisResLab/icd-converter/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/CogDisResLab/icd-converter/actions/workflows/ci.yaml)
[![Contribute with Gitpod](https://img.shields.io/badge/Contribute%20with-Gitpod-908a85?logo=gitpod)](https://gitpod.io/#CogDisResLab/icd-converter)


[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#CogDisResLab/icd-converter)

This project aims to build a small web-api for converting between various ICD codes. The initial goal is to provide a reliable mapping between ICD-9 and ICD-10 codes. The project is currently in the early stages of development.

## Builds

| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.62.0) | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/ubuntu-stable.json) <br/> ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/ubuntu-beta.json) <br/> ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/ubuntu-nightly.json) <br/> ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.62.0) | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/windows-stable.json) <br/> ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/windows-beta.json) <br/> ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/windows-nightly.json) <br/> ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/windows-msrv.json) |
| macOS    | stable <br/> beta <br/> nightly <br/> MSRV (1.62.0) | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/macos-stable.json) <br/> ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/macos-beta.json) <br/> ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/macos-nightly.json) <br/> ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/CogDisResLab/6e694db1d6b96980c71ea0f13611d6c4/raw/macos-msrv.json) |

## Current Status

The project is currently in the very early stages of development. The goal for v1.0 is to provide a complete mapping between ICD-9 and ICD-10. For v2.0, we will add additional mappings to ICD-11 and other codes.

## Contributing

We welcome contributions from the community. Please see our [contributing guidelines](CONTRIBUTING.md) for more information.

## License

This project is dual-licensed under the Apache 2.0 and MIT licenses. See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for more information.

## Code of Conduct

This project adheres to the Contributor Covenant [code of conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Acknowledgements

This project is under development by the [Cognitive Disorders Research Lab](https://cogdisreslab.org) at the University of Toledo Medical Center, College of Medicine and Life Sciences. The Lab is currently led by Dr. [Robert E. Smith, M.D., Ph.D.](https://www.utoledo.edu/med/depts/neurosciences/smith.html)
