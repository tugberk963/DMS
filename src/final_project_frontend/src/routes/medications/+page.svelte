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
        console.log(userData.health_data.medications);
    });
</script>
<main>
    <h2>Your medications</h2>
    <ul class="medications-list">
        {#if userData.health_data && userData.health_data.medications && userData.health_data.medications.length > 0}
            {#each userData.health_data.medications as medications}
                <li>{medications}</li>
            {/each}
        {:else}
            <p>No medications found.</p>
        {/if}
    </ul>
</main>
<style>
    .medications-list {
        list-style-type: none;
        padding: 0;
    }

    .medications-list li {
        background-color: #f4f4f4;
        border-radius: 5px;
        padding: 10px;
        margin-bottom: 5px;
    }
</style>