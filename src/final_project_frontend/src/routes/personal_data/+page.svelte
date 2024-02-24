<script>
    import { onMount } from "svelte";
    import { backend } from "$lib/canisters";
    function onLogout() {
        backend.logout();
        window.location.href = "/";
    }
    let userData = {};
    let name,
        surname,
        location,
        age,
        height,
        weight = "";

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

    async function editPersonalData() {
        try {
            console.log("Name: ", name);
            console.log("User id", userData.identity);
            console.log(
                await backend.edit_user_personal_data(
                    userData.identity,
                    name,
                    surname,
                    location,
                    age,
                    height,
                    weight,
                ),
            );
        } catch (error) {
            console.log("Editing user data failed.");
        }
    }

    onMount(async () => {
        await get_user_data();
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
        <div class="editPersonalInfo">
            <form on:submit={editPersonalData} action="">
                <h3>Enter your name:</h3>
                <input type="text" bind:value={name} />
                <h3>Enter your surname:</h3>
                <input type="text" bind:value={surname} />
                <h3>Enter your location:</h3>
                <input type="text" bind:value={location} />
                <h3>Enter your age:</h3>
                <input type="text" bind:value={age} />
                <h3>Enter your height:</h3>
                <input type="text" bind:value={height} />
                <h3>Enter your weight:</h3>
                <input type="text" bind:value={weight} />
                <button type="submit">Save</button>
            </form>
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

    main {
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
        flex-direction: column;
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

    input[type="text"]{
        width: calc(100% - 44px);
        padding: 8px;
        margin-bottom: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }
    button:hover {
        background-color: #0056b3;
    }
</style>
