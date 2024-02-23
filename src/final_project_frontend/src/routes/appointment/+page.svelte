<script>
    import {backend} from "$lib/canisters";
    import { onMount } from "svelte";

    let providers = [];
    async function getProviders(){
        try {
            providers = await backend.list_providers();
            console.log(providers);
        }
        catch (error){
            console.error("Fetching providers failed.", error);
        }
    }

    async function getDepartments() {
        try {
            let departments = await backend.list_departments(id);
            console.log(departments);
        } catch (error) {
            console.error("Fetching departments failed.", error);
        }
    }

    async function getDoctors() {
        try {
            let doctors = await backend.list_doctors(id, department_name);
            console.log(doctors);
        } catch (error) {
            console.error("Fetching doctors failed.", error);
        }
    }

    async function getDates() {
        try {
            dates = await backend.list_dates(id, department_name, doctor_name);
            console.log(dates);
        } catch (error) {
            console.error("Fetching appointment dates failed.", error);
        }
    }
    

    async function getTimes() {
        try {
            times = await backend.list_times(id, department_name, doctor_name, selected_date);
            console.log(times);
        } catch (error) {
            console.error("Fetching appointment times failed.", error);
        }
    }

    onMount(getProviders);
</script>

<main>
    {#if providers == ''}
        <div>No Providers Found</div>
    {:else}
        <div>Providers found: {providers}</div> 
    {/if}


</main>

<style>

</style>