# Speykious's personal website

> Hello :3

This is the source code of my personal website over at <https://speykious.dev>.

## Tech stack

<table>
  <thead>
    <tr>
      <td>Backend</td>
      <td>Frontend</td>
    </tr>
  </thead>
  <tbody>
    <tr>
<td>

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum/)

</td>
<td>

- [Svelte](https://svelte.dev/) (with [SvelteKit](https://kit.svelte.dev/))
- [TypeScript](https://www.typescriptlang.org/)
- [SCSS](https://sass-lang.com/)

</td>
    </tr>
  </tbody>
</table>

## Installation

### Rust

[Install Rust using the recommended way](https://www.rust-lang.org/tools/install). It will install Rustup, the official Rust toolchain manager, which will keep track of the different Rust toolchains and targets you have installed.

It's better to install Rustup than to install individual Rust tools (such as `rustc` and `cargo`) separately.

If you're on Linux, your distribution may provide a `rustup` package (Arch Linux has one for example).

You'll need to install the stable toolchain. If it isn't already installed, run the following command:

```sh
rustup toolchain install stable
```

### Node.js

Install the latest version of Node.js and NPM.

On Windows and MacOS, you can use [the provided installer](https://nodejs.org/en/download).

On Linux, refer to your distribution's package manager and search for packages like `node` and `npm`.

## Run the project

<table>
  <thead>
    <tr>
      <td>Backend</td>
      <td>Frontend</td>
    </tr>
  </thead>
  <tbody>
    <tr>
<td>

To run the backend server, at the root of the project, simply run:

```sh
cargo run -p server
```

</td>
<td>

To run the frontend app, go to the `client` directory and run:

```sh
npm run dev
```

</td>
    </tr>
  </tbody>
</table>
