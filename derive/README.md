![Abscissa](https://www.iqlusion.io/img/github/iqlusioninc/abscissa/abscissa.svg)

# abscissa_derive: custom derive macros for Abscissa

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]

This crate provides the custom derive implementations used by the
[Abscissa] command-line app microframework.

Note that this crate isn't meant to be used directly, and you don't need to
add it to your `Cargo.toml` file. Instead, just import the relevant types
from Abscissa, and the proc macros will be in scope.

## License

The **abscissa_derive** crate is distributed under the terms of the
Apache License (Version 2.0).

Copyright © 2018-2019 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[crate-image]: https://img.shields.io/crates/v/abscissa_derive.svg
[crate-link]: https://crates.io/crates/abscissa_derive
[docs-image]: https://docs.rs/abscissa_derive/badge.svg
[docs-link]: https://docs.rs/abscissa_derive/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/abscissa/blob/develop/LICENSE
[build-image]: https://travis-ci.com/iqlusioninc/abscissa.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/abscissa
[appveyor-image]: https://ci.appveyor.com/api/projects/status/9bgh8je3rsmbyo0y?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/abscissa
[Abscissa]: https://github.com/iqlusioninc/abscissa
