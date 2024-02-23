<script>
    import { backend } from "$lib/canisters";
    import { onMount } from "svelte";
    import User from "../home/User.svelte";

    let providers = [];
    let departments = [];
    let doctors = [];
    let dates = [];
    let times = [];

    let provider_name = '';
    let department_name = '';
    let doctor_name = '';
    let selected_date = '';
    let selected_time = '';

    let userData = {};
    async function get_user_data() {
        try {
            const userDataString = await backend.get_current_user();
            userData = JSON.parse(userDataString[0]); // Parse JSON string to object
            console.log(userData);
        } catch (error) {
            console.error("Error fetching user data:", error);
            alert("User data couldn't be fetched.");
        }
    }

    async function getProviders(){
        try {
            providers = await backend.list_providers();
            console.log(providers);
        }
        catch (error){
            console.error("Fetching providers failed.", error);
        }
    }

    async function getDepartments() {
        try {
            console.log(userData.identity);
            departments = await backend.list_departments(userData.identity);
            console.log(departments);
        } catch (error) {
            console.error("Fetching departments failed.", error);
        }
    }

    async function getDoctors() {
        try {
            doctors = await backend.list_doctors(userData.identity, department_name);
            console.log(doctors);
        } catch (error) {
            console.error("Fetching doctors failed.", error);
        }
    }

    async function getDates() {
        try {
            dates = await backend.list_dates(userData.identity, department_name, doctor_name);
            console.log(dates);
        } catch (error) {
            console.error("Fetching appointment dates failed.", error);
        }
    }
    
    async function getTimes() {
        try {
            times = await backend.list_times(userData.identity, department_name, doctor_name, selected_date);
            console.log(times);
        } catch (error) {
            console.error("Fetching appointment times failed.", error);
        }
    }


   // fn make_appointment(provider_id: String, department_name: String, doctor_name: String, date: String, time: String) -> Result <(), String>{
    async function makeAppointment(){
        if (userData.identity && department_name && doctor_name && selected_date && selected_time)
            try {
                backend.make_appointment(userData.identity, department_name, doctor_name, selected_date, selected_time);
                console.log("Appointment created.");
            }
            catch(error){
                console.error("Making appointment failed.", error);
            }
        else{
            console.error("Provider data can't be empty.");
        }
    };

    onMount(async () => {
        await get_user_data();
        await getProviders();
    });
</script>

<main class="container">
    <div class="selectProvider">
        <h2>Select Provider</h2>
        <select class="select-box" name="select-provider" id="provider" bind:value={provider_name} on:change={getDepartments}>
            <option value="">Select Provider</option>
            {#each providers as provider}
                <option value={provider}>{provider}</option>
            {/each}
        </select>
    </div>
    <div class="selectDepartment">
        <h2>Select Department</h2>
        <select class="select-box" name="select-department" id="department" bind:value={department_name} on:change={getDoctors}>
            <option value="">Select Department</option>
            {#each departments as department}
                <option value={department}>{department}</option>
            {/each}
        </select>
    </div>
    <div class="selectDoctor">
        <h2>Select Doctor</h2>
        <select class="select-box" name="select-doctor" id="doctor" bind:value={doctor_name} on:change={getDates}>
            <option value="">Select Doctor</option>
            {#each doctors as doctor}
                <option value={doctor}>{doctor}</option>
            {/each}
        </select>
    </div>
    <div class="selectDate">
        <h2>Select Date</h2>
        <select class="select-box" name="select-date" id="date" bind:value={selected_date} on:change={getTimes}>
            <option value="">Select Date</option>
            {#each dates as date}
                <option value={date}>{date}</option>
            {/each}
        </select>
    </div>
    <div class="selectTime">
        <h2>Select Time</h2>
        <select class="select-box" name="select-time" id="time" bind:value={selected_time}>
            <option value="">Select Time</option>
            {#each times as time}
                <option value={time}>{time}</option>
            {/each}
        </select>
    </div>
    <button class="createAppointment" on:click={makeAppointment}>Create Appointment !</button>
</main>

<style>
    .container {
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
    }

    .select-box {
        width: 100%;
        padding: 10px;
        margin-bottom: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        font-size: 16px;
    }

    .createAppointment{
        width: 100%;
        padding: 10px;
        margin-top: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        font-size: 16px;
    }

    h2 {
        margin-bottom: 10px;
    }

</style>
