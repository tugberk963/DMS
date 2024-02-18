# DMS - Decentralized Medical Services - ICP Internship Bootcamp Final Project

# Project Name: DMS: Medical Service Provider on Internet Computer

## Content 


- [Project Purpose](#project-purpose)
- [Project Features](#project-description)
- [Project Vision](#project-vision)
- [Project Setup](#project-setup)
  -  [Setting up IC CDK](#setting-up-ic-cdk)
  -  [Setting up Rust](#setting-up-rust)
  -  [Starting Server](#starting-server)
  -  [Deploying Canisters](#deploying-project)
- [Project Functionalities](#functions)
- [Testing Process](#tests)

## Project Purpose
DMS aims to bring together healthcare service providers (hospitals, health clinics, etc.) and patients, facilitating processes such as appointment scheduling and secure storage of medical records.

## Project Features
### Patient and Healthcare Providers on Single Interface:
  Patients can perform tasks such as scheduling appointments, searching for doctors, and viewing medical records.
  Healthcare providers can handle tasks like doctor assignment, appointment tracking, and updating medical records.
### Appointment Scheduling System:
  Patients can schedule appointments with suitable doctors or departments through DMS.
  The appointment calendar displays the availability of doctors and allows patients to book appointments at their preferred times.
### Medical Record Security:
  Patients' medical records are securely stored on the Internet Computer.
  Data is protected with end-to-end encryption and robust security protocols.
  Access is restricted to authorized users only.
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

