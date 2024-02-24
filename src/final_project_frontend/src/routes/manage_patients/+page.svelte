<script>
    import { backend } from "$lib/canisters";
    import { onMount } from "svelte";

    let department_name, doctor_name, selected_date, selected_time, patient_id, disease_name, medication_name = "";
    let allergy_name = "";
    let departments, doctors, dates, times = [];

    let userData, providerData = {};
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

    async function getProviderData() {
        try {
            const providerDataString = await backend.get_provider_info(
                userData.identity,
            );
            providerData = JSON.parse(providerDataString[0]); // Parse JSON string to object
            console.log(providerData);
        } catch (error) {
            console.error("Error fetching user data:", error);
            alert("User data couldn't be fetched.");
        }
    }

    async function addDisease() {
        try {
            console.log(await backend.add_disease(userData.identity, disease_name));
        }
        catch(e){
            console.log("Error when adding disease", e);
        }
    }

    async function addAllergy() {
        try {
            console.log(await backend.add_allergy(userData.identity, allergy_name));
        }
        catch(e){
            console.log("Error when adding allergy", e);
        }
    }

    async function addMedication() {
        try {
            console.log(await backend.add_medication(userData.identity, medication_name));
        }
        catch(e){
            console.log("Error when adding medication", e);
        }
    }

    onMount(async () => {
        await get_user_data();
        await getProviderData();
    });
</script>

<main>
    <navbar>
        <div class="logo">
            <img src="../logo2.svg" alt="logo">
        </div>
        <div class="settings">
            <button >Provider Settings</button>
        </div>
    </navbar>
    <div class="providerContent">
        <div class="sideBar">
            <a href="/provider_management">Provider Management</a>
            <a href="/search_patients">Search Patients</a>
            <a href="/manage_patients">Manage Patients</a>
            <a href="/provider_profile">Provider Profile</a>
        </div>
        <!-- Middle Container for Detailed Actions -->
        <div class="container">
            <div class="addDisease">
                <h2>Add Disease to Patient</h2>
                <h2>Enter Patient ID:</h2>
                <input type="text" bind:value={patient_id} />
                <h2>Enter Disease Name: </h2>
                <input type="text" bind:value={disease_name} />
                <button on:click={addDisease}>Add Disease</button>
            </div>
            <div class="addAllergy">
                <h2>Add Allergy to Patient</h2>
                <h2>Enter Patient ID:</h2>
                <input type="text" bind:value={patient_id} />
                <h2>Enter Allergy Name: </h2>
                <input type="text" bind:value={allergy_name} />
                <button on:click={addAllergy}>Add Allergy</button>
            </div>
            <div class="addMedication">
                <h2>Add Medication to Patient</h2>
                <h2>Enter Patient ID:</h2>
                <input type="text" bind:value={patient_id} />
                <h2>Enter Medication Name: </h2>
                <input type="text" bind:value={medication_name} />
                <button on:click={addMedication}>Add Medication</button>
            </div>
        </div>
        <!-- Profile Section -->
        <div class="providerProfile">
            <div class="personPic">
                <img src="../../person.png" alt="personPic" />
            </div>
            <div class="personalData">
                {#if userData && providerData}
                    <p class="dfinityID">Dfinity ID: {userData.identity}</p>
                    <p class="username">Username: {userData.username}</p>
                    {#if !providerData.provider_name}
                        <p class="name">Provider Name: Loading..</p>
                    {:else}
                        <p class="name">
                            Provider Name: {providerData.provider_name}
                        </p>
                    {/if}
                    {#if !providerData.provider_location}
                        <p class="location">
                            Provider Location: Please add your location.
                        </p>
                    {:else}
                        <p class="location">
                            Provider Location: {providerData.provider_location}
                        </p>
                    {/if}
                {:else}
                    <p class="dfinityID">Dfinity ID: N/A</p>
                    <p class="username">Username: N/A</p>
                    <p class="name">Provider Name: N/A</p>
                    <p class="surname">Provider Location: N/A</p>
                {/if}
            </div>
        </div>
    </div>

</main>

<style>
    main {
        display: flex;
        flex-direction: column;
    }

    .providerContent {
        display: flex;
        flex-direction: row;
    }

    .sideBar {
        display: flex;
        flex: 1;
        flex-direction: column;
    }
    .container {
        display: flex;
        flex: 5;
        flex-direction: column;
    }
    .providerProfile {
        display: flex;
        flex-direction: column;
        flex: 1;
    }

    .section {
        flex-basis: 45%;
        margin-bottom: 20px;
    }

    h2 {
        margin-bottom: 10px;
    }

    button {
        margin-bottom: 10px;
        padding: 8px 12px;
        border: none;
        border-radius: 4px;
        background-color: #007bff;
        color: #fff;
        cursor: pointer;
    }

    input[type="text"],
    input[type="date"],
    input[type="time"],
    select {
        width: calc(100% - 12px);
        padding: 8px;
        margin-bottom: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .date-option,
    .time-option,
    .department-option,
    .doctor-option {
        margin-bottom: 5px;
        padding: 8px;
        border: 1px solid #ccc;
        border-radius: 4px;
        background-color: #f9f9f9;
    }


    navbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 13px;
        background-color: #007bff;
        color: #fff;
        box-shadow: 0px 2px 5px rgba(0, 0, 0, 0.1);
    }

    .logo img {
        width: 100px;
        height: auto;
    }

    .settings button {
        background-color: transparent;
        border: none;
        color: #fff;
        font-size: 16px;
        cursor: pointer;
        transition: color 0.3s;
    }

    .settings button:hover {
        color: #f0f0f0;
    }
</style>

