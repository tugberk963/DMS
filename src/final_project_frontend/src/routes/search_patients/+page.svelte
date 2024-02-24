<script>
    import { onMount } from "svelte";
    import { backend } from "$lib/canisters";

    let patient_id = "";
    let patientData;
    
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

    async function get_patient_data() {
        try {
            const userDataString = await backend.get_patient_info(patient_id);
            patientData = JSON.parse(userDataString[0]); // Parse JSON string to object
            console.log(patientData);
        } catch (error) {
            console.error("Error fetching user data:", error);
            alert("User data couldn't be fetched.");
        }
    }



    onMount(async () => {
        await get_user_data();
        await get_provider_info();
        console.log(patientData);
    });
</script>

<main>
    <navbar>
        <div class="logo">
            <img src="../logo2.svg" alt="logo" />
        </div>
        <div class="settings">
            <button>Provider Settings</button>
        </div>
    </navbar>
    <div class="providerContent">
        <div class="sideBar">
            <a href="/provider_management">Provider Management</a>
            <a href="/search_patients">Search Patients</a>
            <a href="/manage_patients">Manage Patients</a>
            <a href="/provider_profile">Provider Profile</a>
        </div>
        <h2>Enter Patient ID to See</h2>
        <input type="text" bind:value={patient_id} />
        <button on:click={get_patient_data}>Search Patient</button>
        <div class="patientPersonalData">
            {#if patientData}
                <h2>Name:</h2>
                <p>{patientData.personal_data.name}</p>
                <h2>Surname:</h2>
                <p>{patientData.personal_data.surname}</p>
                <h2>Age:</h2>
                <p>{patientData.personal_data.age}</p>
                <h2>Height:</h2>
                <p>{patientData.personal_data.height}</p>
                <h2>Weight:</h2>
                <p>{patientData.personal_data.weight}</p>
                <h2>Location:</h2>
                <p>{patientData.personal_data.location}</p>
            {:else}
                <p>Search Patient for See the details.</p>
            {/if}
        </div>
        <div class="patientHealthData">
            {#if patientData}
                <h2>{patientData.personal_data.name}'s Diseases</h2>
                <ul class="diseases-list">
                    {#if patientData.health_data && patientData.health_data.diseases && patientData.health_data.diseases.length > 0}
                        {#each patientData.health_data.diseases as disease}
                            <li>{disease}</li>
                        {/each}
                    {:else}
                        <p>No diseases found.</p>
                    {/if}
                </ul>
                <h2>{patientData.personal_data.name}'s Medications</h2>
                <ul class="medications-list">
                    {#if patientData.health_data && patientData.health_data.medications && patientData.health_data.medications.length > 0}
                        {#each patientData.health_data.medications as medications}
                            <li>{medications}</li>
                        {/each}
                    {:else}
                        <p>No medications found.</p>
                    {/if}
                </ul>
                <h2>{patientData.personal_data.name}'s Allergies</h2>
                <ul class="allergies-list">
                    {#if patientData.health_data && patientData.health_data.allergies && patientData.health_data.allergies.length > 0}
                        {#each patientData.health_data.allergies as allergies}
                            <li>{allergies}</li>
                        {/each}
                    {:else}
                        <p>No allergies found.</p>
                    {/if}
                </ul>
            {/if}
        </div>
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
        flex-direction: column;
    }

    .providerProfile {
        display: flex;
        flex-direction: column;
    }

    h2 {
        margin-bottom: 10px;
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
