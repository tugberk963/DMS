<script>
    import { backend } from "$lib/canisters";
    import { onMount } from "svelte";
    function onLogout() {
        backend.logout();
        window.location.href = "/";
    }

    let providers = [];
    let departments = [];
    let doctors = [];
    let dates = [];
    let times = [];

    let provider_name = "";
    let department_name = "";
    let doctor_name = "";
    let selected_date = "";
    let selected_time = "";

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

    async function getProviders() {
        try {
            providers = await backend.list_providers();
            console.log(providers);
        } catch (error) {
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
            doctors = await backend.list_doctors(
                userData.identity,
                department_name,
            );
            console.log(doctors);
        } catch (error) {
            console.error("Fetching doctors failed.", error);
        }
    }

    async function getDates() {
        try {
            dates = await backend.list_dates(
                userData.identity,
                department_name,
                doctor_name,
            );
            console.log(dates);
        } catch (error) {
            console.error("Fetching appointment dates failed.", error);
        }
    }

    async function getTimes() {
        try {
            times = await backend.list_times(
                userData.identity,
                department_name,
                doctor_name,
                selected_date,
            );
            console.log(times);
        } catch (error) {
            console.error("Fetching appointment times failed.", error);
        }
    }

    async function makeAppointment() {
        if (
            userData.identity &&
            department_name &&
            doctor_name &&
            selected_date &&
            selected_time
        )
            try {
                backend.make_appointment(
                    userData.identity,
                    department_name,
                    doctor_name,
                    selected_date,
                    selected_time,
                );
                console.log("Appointment created.");
            } catch (error) {
                console.error("Making appointment failed.", error);
            }
        else {
            console.error("Provider data can't be empty.");
        }
    }

    onMount(async () => {
        await get_user_data();
        await getProviders();
    });
</script>

<main>
    <navbar>
        <div class="logo">
            <img src="../logo.png" alt="logo" />
        </div>
    </navbar>
    <div class="generalContainer">
        <div class="sections">
            <div class="general">
                <h2>General</h2>
                <a href="/home">Home</a>
                <a href="/visits">Visits</a>
                <a href="/diseases">Diseases</a>
                <a href="/allergies">Allergies</a>
                <a href="/medications">Medications</a>
                <a href="/personal_data">Personal Data</a>
            </div>
        </div>
        <div class="appointment">
            <div class="selectProvider">
                <h3>Select Provider</h3>
                <select
                    class="select-box"
                    name="select-provider"
                    id="provider"
                    bind:value={provider_name}
                    on:change={getDepartments}
                >
                    <option value="">Select Provider</option>
                    {#each providers as provider}
                        <option value={provider}>{provider}</option>
                    {/each}
                </select>
            </div>
            <div class="selectDepartment">
                <h3>Select Department</h3>
                <select
                    class="select-box"
                    name="select-department"
                    id="department"
                    bind:value={department_name}
                    on:change={getDoctors}
                >
                    <option value="">Select Department</option>
                    {#each departments as department}
                        <option value={department}>{department}</option>
                    {/each}
                </select>
            </div>
            <div class="selectDoctor">
                <h3>Select Doctor</h3>
                <select
                    class="select-box"
                    name="select-doctor"
                    id="doctor"
                    bind:value={doctor_name}
                    on:change={getDates}
                >
                    <option value="">Select Doctor</option>
                    {#each doctors as doctor}
                        <option value={doctor}>{doctor}</option>
                    {/each}
                </select>
            </div>
            <div class="selectDate">
                <h3>Select Date</h3>
                <select
                    class="select-box"
                    name="select-date"
                    id="date"
                    bind:value={selected_date}
                    on:change={getTimes}
                >
                    <option value="">Select Date</option>
                    {#each dates as date}
                        <option value={date}>{date}</option>
                    {/each}
                </select>
            </div>
            <div class="selectTime">
                <h3>Select Time</h3>
                <select
                    class="select-box"
                    name="select-time"
                    id="time"
                    bind:value={selected_time}
                >
                    <option value="">Select Time</option>
                    {#each times as time}
                        <option value={time}>{time}</option>
                    {/each}
                </select>
            </div>
            <button class="createAppointment" on:click={makeAppointment}
                >Create Appointment !</button
            >
        </div>
        <div class="personalInformation">
            <div class="personPic">
                <img src="../../person.png" alt="personPic" />
            </div>
            <div class="personalData">
                {#if userData && userData.personal_data && userData.health_data}
                    <p class="dfinityID">Dfinity ID: {userData.identity}</p>
                    <p class="username">Username: {userData.username}</p>
                    {#if userData.personal_data.name == "" || userData.personal_data.surname == "" || userData.personal_data.height == "" || userData.personal_data.weight == "" || userData.personal_data.age == "" || userData.personal_data.location == ""}
                        <p>Please add your personal data.</p>
                    {/if}
                    {#if userData.personal_data.name}
                        <p class="name">Name: {userData.personal_data.name}</p>
                    {/if}
                    {#if userData.personal_data.surname}
                        <p class="surname">
                            Surname: {userData.personal_data.surname}
                        </p>
                    {/if}
                    {#if userData.personal_data.name}
                        <p class="age">Age: {userData.personal_data.age}</p>
                    {/if}
                    {#if userData.personal_data.height}
                        <p class="height">
                            Height: {userData.personal_data.height}
                        </p>
                    {/if}
                    {#if userData.personal_data.weight}
                        <p class="weight">
                            Weight: {userData.personal_data.weight}
                        </p>
                    {/if}
                    {#if userData.personal_data.location}
                        <p class="location">
                            Location: {userData.personal_data.location}
                        </p>
                    {/if}
                {:else}
                    <p class="dfinityID">Dfinity ID: N/A</p>
                    <p class="username">Username: N/A</p>
                    <p class="name">Name: N/A</p>
                    <p class="surname">Surname: N/A</p>
                    <p class="age">Age: N/A</p>
                    <p class="height">Height: N/A</p>
                    <p class="weight">Weight: N/A</p>
                    <p class="location">Location: N/A</p>
                {/if}
            </div>
            <br />
            <button on:click|preventDefault={onLogout}
                >Click for logout..</button
            >
        </div>
    </div>
</main>

<style>
    navbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px;
        background-color: #fcfcfc;
        color: #fff;
        box-shadow: 0px 2px 5px rgba(0, 0, 0, 0.1);
    }

    .logo img {
        width: 100px;
        height: auto;
    }

    .personalInformation {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-left: 20px;
    }

    .personPic img {
        width: 100px;
        height: 100px;
        border-radius: 50%;
        margin-bottom: 20px;
        box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
    }

    a {
        color: #333;
        text-decoration: none;
        font-size: 18px;
        margin-bottom: 10px;
        display: block;
        transition: color 0.3s;
    }

    a:hover {
        color: #007bff;
    }

    main{
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        display: flex;
        flex-direction: column;
    }

    .generalContainer {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        padding: 20px;
    }

    .general {
        display: flex;
        flex-direction:  column;
    }

    button {
        background-color: #007bff;
        color: #fff;
        border: none;
        border-radius: 5px;
        padding: 10px 20px;
        cursor: pointer;
        font-size: 16px;
        transition: background-color 0.3s;
    }

    button:hover {
        background-color: #0056b3;
    }

    .select-box {
        width: 100%;
        padding: 10px;
        margin-bottom: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        font-size: 16px;
    }

    .createAppointment {
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
