// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2022
// *
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// *
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// *     http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// *

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[warn(clippy::upper_case_acronyms)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Server(Server),
    Match(Match),
}

/// Start an API server to convert ICD-9, ICD-10, or ICD-11 codes to another coding system
#[derive(Debug, Args)]
pub struct Server {
    /// The port to listen on
    ///
    /// The port to listen on. Defaults to 8080.
    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    /// The address to listen on
    ///
    /// The address to listen on. Defaults to localhost.
    #[clap(short, long, default_value = "localhost")]
    pub address: String,
}

/// Convert an ICD-9, ICD-10, or ICD-11 code to another coding system interactively
#[derive(Debug, Args)]
pub struct Match {
    /// The code to be converted
    ///
    /// This specifies the code to be converted. It must be a valid ICD-10 or ICD-9 code.
    #[clap(short, long)]
    pub code: String,

    /// The coding system of the code to be converted
    ///
    /// The coding system of the code to be converted.
    /// This can be one of icd9, icd10, or icd11.
    #[clap(short, long)]
    pub from: CodingSystem,

    /// The coding system to convert to
    ///
    /// The coding system to convert to. This can be one of icd9, icd10, or icd11.
    #[clap(short, long)]
    pub to: CodingSystem,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum CodingSystem {
    ICD11,
    ICD10,
    ICD9,
}
