<script>
    import { onMount } from "svelte";
    import { backend } from "$lib/canisters";    

    let userData = {}
    let name, surname, location, age, height, weight = '';

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

    async function editPersonalData(){
        try {
            console.log("Name: ", name);
            console.log("User id", userData.identity);
            console.log(await backend.edit_user_personal_data(
                userData.identity,
                name,
                surname,
                location,
                age,
                height,
                weight,
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
    <div class="editPersonalInfo">
        <form on:submit={editPersonalData} action="">
        <h2>Enter your name: </h2>
        <input type="text" bind:value={name}>
        <h2>Enter your surname: </h2>
        <input type="text" bind:value={surname}>
        <h2>Enter your location: </h2>
        <input type="text" bind:value={location}>
        <h2>Enter your age: </h2>
        <input type="text" bind:value={age}>
        <h2>Enter your height: </h2>
        <input type="text" bind:value={height}>
        <h2>Enter your weight: </h2>
        <input type="text" bind:value={weight}>
        <button type="submit">Save</button>
    </form>
    </div> 
</main>
<style>

</style>