# innablr-challenge

[![CI](https://github.com/bradyslot/innablr-challenge/actions/workflows/ci.yml/badge.svg)](https://github.com/bradyslot/innablr-challenge/actions)

## What is this really?
This was my answer to a devops interview challenge.  
Clearly the company doesn't respect peoples time or effort.  
I don't respect them either, so it's free for anyone else to spy on.  
If this firm reaches out to you, they're looking for a standard higher than this, whatever that means ¯\_(ツ)_/¯  

## Objectives
Serve as a base for creating REST API's in Rust using the [Rocket](https://github.com/SergioBenitez/Rocket) web framework.  
Have something that is easily cloned and extended fit for purpose.  
Be an example of a simple API with a "/" and "/status" endpoint.  
Pre-configured Build and Testing CI with Github Actions.  
Pre-configured Docker CI using Github Packages as the image repository.  


## Limitations
This is a minimum viable API example.  
While Rocket supports TLS and tokens, these haven't been configured for this example.  


## Advantages
Why Rust and Rocket?  
Good question!  
Answer: Blazingly fast, memory safety, type safety, simplicity and extensibility.  
Rust is becoming a great language to write all applications in, even web applications.  
Rust's TLS implementation is highly secure.  
It's way faster than scripting languages like Python.  
It's way faster than VM or JIT languages like JavaScript.  
It's slightly faster than Garbage Collected languages like Golang.  
It's as fast as Non-CG'd languages like C/C++.  


## Running and testing the API

### Run with docker
innablr-challenge:latest is built using Github Actions on each push to master.  
Tagged releases are also published with the version tag.  

```bash
docker pull ghcr.io/bradyslot/innablr-challenge:latest
docker run --network host -d --rm --name innablr-challenge-1 innablr-challenge
```

### Run locally
Install cargo and rustup via your preferred package manager, or using the official script for Linux and MacOS.  

```bash
curl https://sh.rustup.rs -sSf | sh
```

Windows binary can be obtained from [win.rustup.rs](https://win.rustup.rs/).

```bash
git clone git@github.com:bradyslot/innablr-challenge.git && cd innablr-challenge
cargo run
```

### Test API
Once the application is running you can test the API endpoints from the CLI using curl.  
You should expect to see similar output to the below.  
```bash
curl -s localhost:8000/
Hello World
```

The variables in the JSON returned from "/status" are retrieved using the crate [vergen](https://github.com/rustyhorde/vergen).  
The program name and description is pulled from Cargo.toml.  
"sha" is pulled from the local environment at build time using the build.rs file
which compiles and runs as a build-depedency of the project.  
```bash
curl -s localhost:8000/status | jq
```
```json
{
  "innablr-challenge": [
    {
      "description": "Innablr API challenge written in rust using rocket framework.",
      "sha": "61109d9d5d305f94f7e4b61109c58dc9fab52e4d",
      "version": "0.1.0"
    }
  ]
}
````

Rocket provides a way for catching requests on unknown endpoints and returning a specific response.  
```bash
curl -s localhost:8000/random | jq
```
```json
{
  "reason": "Resource not found",
  "status": "error"
}
```
