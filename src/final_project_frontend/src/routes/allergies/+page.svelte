<script>
    import { onMount } from "svelte";
    import { backend } from "$lib/canisters";    

    let userData = {}
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
    
    onMount(async () => {
        await get_user_data();
        console.log(userData.health_data.allergies);
    });
</script>
<main>
    <h2>Your Allergies</h2>
    <ul class="allergies-list">
        {#if userData.health_data && userData.health_data.allergies && userData.health_data.allergies.length > 0}
            {#each userData.health_data.allergies as allergies}
                <li>{allergies}</li>
            {/each}
        {:else}
            <p>No allergies found.</p>
        {/if}
    </ul>
</main>
<style>
    .allergies-list {
        list-style-type: none;
        padding: 0;
    }

    .allergies-list li {
        background-color: #f4f4f4;
        border-radius: 5px;
        padding: 10px;
        margin-bottom: 5px;
    }
</style>