<script>
    import { backend } from "$lib/canisters";
    import { onMount } from "svelte";

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

    async function editProviderInfo() {
        try {
            console.log(
                await backend.edit_provider_info(
                    userData.identity,
                    providerData.provider_name,
                    providerData.provider_location,
                ),
            );
        } catch (error) {
            console.error("Changing Provider Info failed", error);
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
            <div class="editProfiderInfo">
                <h2>Enter Provider Name:</h2>
                <input type="text" bind:value={providerData.provider_name} />
                <h2>Enter Provider Location</h2>
                <input type="text" bind:value={providerData.provider_location} />
                <button on:click={editProviderInfo}>Change Provider Info</button>
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

    input[type="text"]{
        width: calc(100% - 12px);
        padding: 8px;
        margin-bottom: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
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
