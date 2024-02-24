use crate::{ACTIVE_SESSIONS, USERS, User, Session, HealthData, PersonalData};
use ic_cdk::{update};
use std::collections::HashMap;

#[update]
fn sign_up(username: String, password: String) -> Result<(), String> {
    if username.is_empty() || password.is_empty() {
        return Err("Username or password cannot be empty".to_string());
    }

    //One Principal should Create only 1 account on system
    if USERS.with(|users| users.borrow().contains_key(&ic_cdk::caller())) {
        return Err("User already exists".to_string());
    };

    // // Principals can create multiple accounts on system with different user credentials
    // if USERS.with(|users| users.borrow().values().any(|user| user.username == username)) {
    //     return Err("User already exists".to_string());
    // }

    let new_user_id = ic_cdk::caller();
    let new_user_credentials = User {
        identity: new_user_id.to_string(),
        username: username.to_string(),
        password: password.to_string(),
        appointments: HashMap::new(),
        health_data: HealthData {allergies: Vec::new(), diseases: Vec::new()},
        personal_data: PersonalData{name: "".to_string(), surname: "".to_string() , age: "".to_string(), height: "".to_string(), weight: "".to_string(), location: "".to_string()},
    };

    USERS.with(|users| users.borrow_mut().insert(new_user_id, new_user_credentials));
    Ok(())
}

// Login Function
#[update]
fn login(username: String, password: String) -> Result<(), String> {
    if USERS.with(|users| users.borrow().is_empty()) {
        return Err("No users exist".to_string());
    }

    let user_exists = USERS.with(|users| {
        users.borrow().values().any(|user| user.username == username && user.password == password)
    });

    if !user_exists {
        return Err("Wrong user credentials".to_string());
    }

    if ACTIVE_SESSIONS.with(|sessions| sessions.borrow().iter().any(|session| session.user_id == ic_cdk::caller())) {
        return Err("User already logged in".to_string());
    }

    let user_id = ic_cdk::caller();
    ACTIVE_SESSIONS.with(|sessions| {
        sessions.borrow_mut().push(Session { user_id });
    });

    Ok(())
}

// Log Out Function
#[update]
fn logout() {
    let user_id = ic_cdk::caller();
    ACTIVE_SESSIONS.with(|sessions| {
        sessions.borrow_mut().retain(|session| session.user_id != user_id);
    });
}

