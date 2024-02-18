# DMS - Decentralized Medical Services - ICP Internship Bootcamp Final Project



## Content 
- [Project Name](#project-name)
- [Project Team](#project-team---individual)
- [Project Description - Purpose](#project-description---purpose)
- [Project Vision](#project-vision)
- [Project Setup](#project-setup)
  -  [Setting up IC CDK](#setting-up-ic-cdk)
  -  [Setting up Rust](#setting-up-rust)
  -  [Starting Server](#starting-server)
  -  [Deploying Canisters](#deploying-canisters)
- [Project Functionalities](#functions)
- [Testing Process](#tests)

## Project Name
DMS: Medical Service Provider on Internet Computer
<b> <i> DMS stands for Decentralized Medical Services </b> </i> 
## Project Team - Individual
Tuğberk Serçe :D
## Project Description - Purpose 
DMS aims to revolutionize healthcare services by bringing together healthcare service providers (hospitals, health clinics, etc.) and patients on a unified platform. </br> </br> <strong> <i> For final project, some of main functionalities of DMS built on Rust as back-end canisters. </i> </strong>
## Project Vision
The primary vision of the project is to establish DMS as the <b>  <i> central platform for healthcare service providers and patients worldwide. </i> </b>
<br>Through the vast data flow integrated into DMS, we empower its AI capabilities, transforming it into an efficient and proactive predictive healthcare service provider. <br> This strategic approach aims to revolutionize healthcare delivery, offering advanced and personalized services to users globally.
### Single Interface for Patients and Healthcare Providers:
DMS provides a unified platform where both patients and healthcare providers can access and manage all medical service operations. Patients can schedule appointments, search for doctors, and view medical records, while healthcare providers can handle tasks such as doctor assignment, appointment tracking, and updating medical records, all through a single interface standing on <b><a href="https://internetcomputer.org/"> Internet Computer </a></b>.
### Medical Record Security:
Patients' medical records are securely stored on the  <b> <a href="https://internetcomputer.org/"> Internet Computer </a> </b>. Data is protected with end-to-end encryption and security protocols, ensuring that access is restricted to authorized users only.
### Utilization of Data for Predictive Healthcare Services:

DMS collects and leverages data to train machine learning models for predictive healthcare services. These models analyze patterns and trends within the data to make predictions related to patients' health outcomes, potential diseases, or treatment effectiveness.
###  Grant System for Data Sharing:
DMS implements a grant system where individuals providing health data are rewarded with grants. Hospitals, as the institutions approving this data, receive grants based on the number of patients they approve. These grants contribute to financing the privileged healthcare services offered to DMS users.
### Enhanced Healthcare Services through AI:
By utilizing AI-driven predictive models, DMS enhances healthcare services by providing proactive and personalized care to patients. For example, the AI model can identify individuals at higher risk for certain health conditions and recommend preventive measures or early interventions to mitigate those risks.
### Improving Healthcare Accessibility and Efficiency:
AI-driven predictive models optimize resource allocation and improve healthcare service efficiency within DMS. By predicting patient needs and allocating resources accordingly, healthcare providers streamline their operations and ensure better accessibility to healthcare services for all patients.
<br> </br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/GrantSystem.png"> <br> </br>
## Project Setup
### Setting up IC CDK
```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```
For more information or platform specific setup instructions see <a href="https://internetcomputer.org/docs/current/developer-docs/getting-started/install/">ICP Docs : IC_CDK </a>
### Setting up Rust
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
```bash
rustup target add wasm32-unknown-unknown
```
For more information or platform specific setup instructions see <a href="https://internetcomputer.org/docs/current/developer-docs/backend/rust/dev-env" >ICP Docs : Rust</a>
### Starting Server
For starting project on our local. You can simply use the command below.
```bash
cd ICP_Internship_Bootcamp_Final_Project/
```
```bash
dfx start --clean
```
### Deploying Canisters
For deploying canisters and generating our canister interface. You can use the command below.
```bash
cd ICP_Internship_Bootcamp_Final_Project/
```
```bash
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

