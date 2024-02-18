# DMS - Decentralized Medical Services - ICP Internship Bootcamp Final Project

Welcome to your new final_project project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

## Content 

- [Project Description](#project-description)
- [Project Purpose](#project-purpose)
- [Project Vision](#project-vision)
- [Project Setup](#project-setup)
  -  [Setting up IC CDK](#setting-up-ic-cdk)
  -  [Setting up Rust](#setting-up-rust)
  -  [Starting Server](#starting-server)
  -  [Deploying Canisters](#deploying-project)
- [Project Functionalities](#functions)
- [Testing Process](#tests)

## Project Description
Project Description

## Project Purpose
Project Purpose

## Project Vision
Project Vision

## Project Setup
### Setting up IC CDK
Setting up IC_CDK
### Setting up Rust
Setting up Rust
### Starting Server
For starting project on our local. You can simply use the command below.
```bash
cd ICP_Internship_Bootcamp_Final_Project/
dfx start --clean
```
### Deploying Canisters
For deploying canisters and generating our canister interface. You can use the command below.
```bash
cd ICP_Internship_Bootcamp_Final_Project/
dfx deploy
```
Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with
```bash
npm run generate
```
at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.
If you are making frontend changes, you can start a development server with
```bash
npm start
```
Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

