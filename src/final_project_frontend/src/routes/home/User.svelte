<script>
    import { onMount } from "svelte";
    import { backend } from "$lib/canisters";    
    function onLogout() {
        backend.logout();
        window.location.href = "/";
    }
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

    function sendAppointmentPage(){
        window.location.href = "/appointment";
    }


    async function editPersonalData(){
        try {
            console.log(await backend.edit_user_personal_data(
                userData.identity,
                userData.personal_data.name,
                userData.personal_data.surname,
                userData.personal_data.location,
                userData.personal_data.age,
                userData.personal_data.height,
                userData.personal_data.weight,
            ));
        }
        catch (error)
        {
            console.log("Editing user data failed.");
        }
    }

    onMount(async () => {
        await get_user_data();
    });
</script>


<main>
    <!-- <navbar> 
        <div class="logo">
            <img src="../logo2.svg" alt="logo">
        </div>
        <div class="settings">
            <button>Settings</button>
        </div>
    </navbar> -->
    <div class="homeScreen">
        <div class="sections">
            <div class="general">
                <h2>General</h2>
                <a href="/">DMS AI</a>
                <a href="/">Visits</a>
                <a href="/">Prescriptions</a>
                <a href="/">Diseases</a>
                <a href="/">Tests</a>
            </div>
            <div class="explore">
                <h2>Explore</h2>
                <a href="/">Appointment</a>
                <a href="/">Allergies</a>
                <a href="/">Medications</a>
                <a href="/">Documents</a>
                <a href="/">Vaccinations Calendar</a>
            </div>
        </div>
        <div class="detailedContainer">
            <!-- <div class="editPersonalInfo">
                <h2>Enter your name: </h2>
                <input type="text" bind:value={userData.personal_data.name}>
                <h2>Enter your surname: </h2>
                <input type="text" bind:value={userData.personal_data.surname}>
                <h2>Enter your location: </h2>
                <input type="text" bind:value={userData.personal_data.location}>
                <h2>Enter your age: </h2>
                <input type="text" bind:value={userData.personal_data.age}>
                <h2>Enter your height: </h2>
                <input type="text" bind:value={userData.personal_data.height}>
                <h2>Enter your weight: </h2>
                <input type="text" bind:value={userData.personal_data.weight}>
            </div> -->
            <div class="makeAppointment">
                {#if userData.appointments && Object.keys(userData.appointments).length > 0}
                    <h5>Current Appointments</h5>
                    {#each Object.entries(userData.appointments) as [key, value]}
                        <div class="appointmentCard">
                            <h3>Provider ID: {key}</h3>
                            {#each value as appointment}
                                <div>
                                    <p>Department: {appointment.department}</p>
                                    <p>Doctor: {appointment.doctor}</p>
                                    <p>Date: {appointment.date}</p>
                                    <p>Time: {appointment.time}</p>
                                </div>
                            {/each}
                        </div>
                    {/each}
                {:else}
                    <h5>No appointments</h5>
                    <p>You don't have any active appointments.</p>
                    <p>Take an appointment date!</p>
                    <button on:click={sendAppointmentPage}>Make Appointment</button>
                {/if}
            </div>
            <div class="shareSection">
                <h5>Share your health data</h5>
                <p>You can share your health data with the ones only you want.</p>
                <button>Share</button>
            </div>
            <div class="predictor">
                <h5>What may you have?</h5>
                <p>Take a quick survey and share your test results, let DMS AI decide for you !</p>
                <button>Start</button>
            </div>
            <div class="nearProviders">
                <div class="nearHospitals">
                    <h5>Closest DMS Providers</h5>
                    <p>
                        See nearest DMS Providers. Hospitals, health clinics etc.
                    </p>
                    <img src="../hospital.png" alt="">
                </div>
                <div class="nearPharmacy">
                    <h5>Closest DMS Pharmacy</h5>
                    <p>
                        See nearest DMS Providers. Hospitals, health clinics etc.
                    </p>
                    <img src="../pharmacy.png" alt="">
                </div>
            </div>
        </div>
        <div class="lastVisits">
            <h1>Last Visits</h1>
            <div class="lastVisitContainer">
                <div class="visit">
                    <div class="dateSection">
                        <div class="visitDate">3 May</div>
                    </div>
                    <div class="visitDetails">
                        <div class="providerName">Hospital A</div>
                        <div class="departmentName">Radiology</div>
                        <div class="doctorName">John D.</div>
                    </div>
                    <button>Show Details</button>
                </div>
            </div>
        </div>
        <div class="personalInformation">
            <div class="personPic">
                <img src="../../person.jpeg" alt="personPic">
            </div>
            <div class="personalData">
                {#if userData && userData.personal_data && userData.health_data}
                <p class="dfinityID">Dfinity ID: {userData.identity}</p>
                <p class="username">Username: {userData.username}</p>
                <p class="name">Name: {userData.personal_data.name}</p>
                <p class="surname">Surname: {userData.personal_data.surname}</p>
                <p class="age">Age: {userData.personal_data.age}</p>
                <p class="height">Height: {userData.personal_data.height}</p>
                <p class="weight">Weight: {userData.personal_data.weight}</p>
                <p class="location">Location: {userData.personal_data.location}</p>
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
            <br>
            <button on:click|preventDefault={onLogout}>Click for logout..</button>
        </div>
    </div>
    
</main>

<style>
    main {
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
    
    .homeScreen {
        display: flex;
        justify-content: space-between;
        flex-grow: 1;
        padding: 20px;
    }

    .sections {
        display: flex;
        flex-direction: column;
    }

    .general, .explore {
        margin-right: 20px;
    }

    h2 {
        font-size: 24px;
        margin-bottom: 15px;
        color: #333;
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

    .detailedContainer {
        flex-grow: 1;
        margin-right: 20px;
    }

    .shareSection, .predictor, .makeAppointment, .appointmentCard {
        background-color: #f9f9f9;
        border-radius: 10px;
        padding: 20px;
        margin-bottom: 20px;
        box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
    }

    .appointmentCard {
        width: 200px; /* Adjust the width as needed */
        margin-right: 20px; /* Add some space between appointment cards */
    }

    .appointmentCard:nth-child(odd) {
        background-color: #eaeaea;
    }

    .shareSection p, .predictor p, .makeAppointment p, .nearProviders p {
        font-size: 18px;
        color: #333;
    }

    .nearProviders{
        display:flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .nearHospitals, .nearPharmacy{
        flex: 1;
        margin-right: 7px;
        padding: 30px;
        background-color: #fff;
        border-radius: 10px;
        box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
    }

    .personalInformation {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-left: 20px;
    }

    .personPic img {
        width: 150px;
        height: 150px;
        border-radius: 50%;
        margin-bottom: 20px;
        box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
    }

    .personalData p {
        font-size: 16px;
        margin-bottom: 5px;
        color: #555;
    }

    .personalData p:last-child {
        margin-bottom: 20px;
    }

    .lastVisits {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .lastVisits h1 {
        font-size: 28px;
        margin-bottom: 20px;
        color: #333;
    }

    .lastVisitContainer {
        border: 1px solid #ccc;
        border-radius: 10px;
        padding: 20px;
        width: 80%;
        max-width: 400px; /* Limiting the max-width for better readability */
        margin-bottom: 20px;
        box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
        transition: transform 0.3s;
    }

    .visit {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .dateSection {
        background-color: #007bff;
        color: #fff;
        border-radius: 50%;
        padding: 10px;
    }

    .visitDate {
        font-size: 18px;
        margin-left: 10px;
    }

    .visitDetails {
        margin-left: 20px;
    }

    .providerName, .departmentName, .doctorName {
        font-size: 16px;
        margin-bottom: 5px;
        color: #333;
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
</style>
