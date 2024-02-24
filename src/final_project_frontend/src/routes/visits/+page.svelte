<script>
    import { onMount } from "svelte";
    import { backend } from "$lib/canisters";

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

    onMount(async () => {
        await get_user_data();
        console.log(userData.appointments);
    });
</script>

<main>
    <div>
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
            <button>Make Appointment</button>
        {/if}
    </div>
</main>

<style>
</style>
