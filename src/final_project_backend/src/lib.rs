mod auth;
mod administration;
mod provider_utils;
mod user_utils;

use candid::{Principal, CandidType};
use std::collections::HashMap;
use std::cell::RefCell;

// Active Sessions contains logged in principal ids.
type ActiveSessions = Vec<Session>;
// User Types
type Users = HashMap<Principal, User>;
type Appointments = HashMap<Principal, Vec<AppointmentDetails>>;
// Admins ??
type Providers = HashMap<Principal, Provider>;

// Provider Subsections
type Departments = HashMap<String, Doctors>;
type Doctors = HashMap<String, Dates>;
type Dates = HashMap<String, Vec<String>>;

thread_local! {
    pub static ACTIVE_SESSIONS: RefCell<ActiveSessions> = RefCell::default();
    pub static USERS: RefCell<Users> = RefCell::default();
    pub static PROVIDERS: RefCell<Providers> = RefCell::default();
}

// Session Struct
#[derive(Clone, Debug)]
struct Session {
    pub user_id: Principal, // Check here
}

// User Struct
#[derive(Clone, Debug)]
struct User {
    pub username: String,
    pub password: String,
    pub appointments: Appointments,
}

#[derive(Clone, Debug, CandidType)]
struct AppointmentDetails {
    pub department: String,
    pub doctor: String,
    pub date: String,
    pub time: String,
}

#[derive(Clone, Debug)]
struct Provider { // a.k.a. Hospitals - Medical Service PROVIDERS
    pub provider_name: String,
    pub provider_pass: String,
    pub departments: Departments, // Department's Name, Department's Doctors
    // pub departments: HashMap<String, Doctors>,
}

// #[derive(Clone, Debug)]
// struct Doctors {
//     doctors: HashMap<String, Dates> // Doctor's name, Doctor's Available Dates
// }

// #[derive(Clone, Debug)]
// struct Dates {
//     times: HashMap<String, Vec<String>> // Date and Available Hours on that day
// }