## About

Aker app using Yew Framework with support for React Components.

## Usage

### Install
```
yarn install
```

### Build

```
yarn run build
```

### Serve locally

```
npm run start:dev
```

### Simple graphql operations script:
```
npm run aker-gql-op -- robot_add --name K3_L01_1 --path K3_L01_1

npm run aker-gql-op -- user_by_id --user_id 5
```



### Sanity check

```
rozgo@alien-rozgo:~/aker/aker-app$ npm --version
6.14.8
rozgo@alien-rozgo:~/aker/aker-app$ node --version
v14.9.0
rozgo@alien-rozgo:~/aker/aker-app$ yarn --version
1.22.5
rozgo@alien-rozgo:~/aker/aker-app$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/rozgo/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-2020-04-07-x86_64-unknown-linux-gnu
nightly-2020-04-17-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.46.0 (04488afe3 2020-08-24)

rozgo@alien-rozgo:~/aker/aker-app$ 
```
