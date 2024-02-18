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
In this final project, project's main functionalities builded on Rust as back-end canisters.

## Project Vision
### Single Interface for Patients and Healthcare Providers:
  DMS aiming to provide a unified platform where both patients and healthcare providers can access and manage all medical service operations. Patients can schedule appointments, search for doctors, and view medical records, while healthcare providers can handle tasks such as doctor assignment, appointment tracking, and updating medical records, all through a single interface which stands on Internet Computer.
### Appointment Scheduling System:
  Patients can schedule appointments with suitable doctors or departments through DMS.
The appointment calendar displays the availability of doctors and allows patients to book appointments at their preferred times.
### Medical Record Security:
  Patients' medical records are securely stored on the Internet Computer.
  Data is protected with end-to-end encryption and robust security protocols.
  Access is restricted to authorized users only.
### Utilization of Data for Predictive Healthcare Services:
The data collected within DMS can be leveraged to train machine learning models for predictive healthcare services.
These machine learning models can analyze patterns and trends within the data to make predictions related to patients' health outcomes, potential diseases, or treatment effectiveness.
### Enhanced Healthcare Services through AI:
By utilizing AI-driven predictive models, DMS can enhance healthcare services by providing proactive and personalized care to patients.
For example, the AI model can identify individuals at higher risk for certain health conditions and recommend preventive measures or early interventions to mitigate those risks.
### Improving Healthcare Accessibility and Efficiency:
AI-driven predictive models can also help in optimizing resource allocation and improving healthcare service efficiency.
By predicting patient needs and allocating resources accordingly, healthcare providers can streamline their operations and ensure better accessibility to healthcare services for all patients.

## Project 


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

