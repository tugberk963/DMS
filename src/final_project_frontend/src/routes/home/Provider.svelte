<script>
    import { onMount } from "svelte";
    import {backend} from "$lib/canisters";

    export let id;

    async function getDepartments(identity){
        try {
             console.log(await backend.list_departments(identity));
        }
        catch (error){
            console.error("Fetching departments failed", error);
        }        
    } 
</script>

<main>
    <div class="departments">
        Current Departments:
        <button on:click(getDepartments({identity}))>Add Department</button> 
    </div>
    <div class="doctors">
        Current Doctors: 
        <button>Add Doctor</button>
    </div>
    <div class="dates">
        Current Appointment Dates:
        <button>Add Appointment Date</button>
    </div>
    <div class="times">
        Current Appointment Times:
        <button>Add Appointment Times</button>
    </div>
</main>

<style>

</style>
