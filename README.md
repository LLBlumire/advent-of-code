# Lucy's Advent of Code Hub

Welcome to the central repository for my advent of code solutions!

I have engineered this repository to be the central location of my advent of code task solutions and
secondary tooling: such as an input downloader, and a code skeleton generator.

## Branches

Two branches are available, `master`, which contains all of my solutions, and `public`, which
contains all of the tooling but without any pre-completed solutions. `master` is the default as
I expect it to be more likely that anyone browsing to this page wants to see my solutions, than to
use my tooling.

## Downloading

Assuming you have access to git (https://git-scm.org).

**if you want to browse and test my solutions**

```sh
git clone -b master --single-branch https://github.com/LLBlumire/advent-of-code.git
```

**if you want to use my advent of code tooling**

```sh
git clone -b tooling --single-branch https://github.com/LLBlumire/advent-of-code.git
```

## Tooling Usage

To build my tooling, you must be able to run `cargo` and `rustc` (https://www.rust-lang.org/).

To use my advent of code tooling, you will need to provide your session token to the tooling. This
is done by creating a file, `Advent.toml`, and filling it with the following:

```toml
[config]
session = "YOUR_ADVENT_OF_CODE_SESSION_TOKEN_HERE"
```

To get your session token, and fill it in, browse to https://adventofcode.com, log in, and browse
to your cookies (on firefox this is in "Storage" in developer tools). One of your cookies should be
called "session", and should have a value that looks like a long string of hexadecimal characters.
This value is your session token, and should be placed into the configuration, such that it
resembles the following:

```toml
[config]
session = "4f8a8398b9bcb9b43cf3065b0cc19d04be04e4288bb42895838009ad6a024f0affbfe4ce680184d0a4523efca7e3bedd"
```

Once you have got your `Advent.toml` set up, the rest of the tooling becomes available.

**NOTE**

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

### Downloading An Input

To run my solutions, or create your own, you will need to download the input from advent of code.

The creator has not licensed these inputs for reproduciton, so they are not shared here.

Instead I have provided tooling to make downloading them easy. The following command will download
the input for year ####, day ##, and save them into `./inputs/y####d##.txt`.

```
cargo run --bin util -- scaffold --download-only --year ##### --day ## --path ./inputs/y#####d###.txt --config ./Advent.toml
```

Note: if you do not provide the year, it will assume the current year, or previous year if advent
of code has not yet begun in the current calendar year. If you do not provide a day, it will assume
the current day, or the most recent day an advent of code took place on if it is not currently an
advent of code day. If you do not provide a file path, it will assume `./inputs/y#####d###.txt`
relative to the crate root directory. If you do not provide a config, it will assume it is
`Advent.toml` in the crate root directory.

### Running one of my Solutions

```
cargo run --bin util -- run --year ##### --day ### --path ./inputs/y#####d###.txt
```

It will use the same default year, day, and path as the file downloader.

### Creating your own solution

If you are on the `public` branch and wish to create your own solution, you can fully scaffold a
solution file with the following syntax

```
cargo run --bin util -- scaffold --year ##### --day ## --path ./inputs/y#####d###.txt --bin y#####d###
```

It will use the same defaults as the base scaffold, with the default binary name being `y#####d###`.

Meaning if you are on a new day of an advent of code, and want to solve that days solution, you need
only run:

```sh
cargo run --bin util -- scaffold
```

To create the code and download your input, and

```sh
cargo run --bin util -- run
```

To run your solution

#### Adding dependencies

For the base environment, I have included a very minimal set of dependencies. You may wish to add
more dependencies over time, this is trivially done by modifying the `Cargo.toml` file provided.
Please do not remove any of the following dependencies, as they are required for the utility tools
to function. Please note that this file is regenerated programatically if you scaffold a new
binary in order to add it to the list of compilable binaries, and as such your comments will not
be preserved (though I will not remove any extra dependencies).

## Question: Why are you not using workspaces

Setting up multiple binaries and dynamically editing the `Cargo.toml` allows the code to have a
much easier to browse structure, with each of my solutions being a simple to follow rust file rather
than an entire project workspace with their own dependencies and configuration. This is not a large
enterprise project prioritising correctness, but a quick way for me to make solving advents of code
fast and easy for myself. If this were the former, I'd likely restructure this into a workspace.
