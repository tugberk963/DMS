<script>
    import {backend} from "$lib/canisters";

    export let id;
    let department_name = "Kulak burun boğaza";
    let doctor_name = "Necmyyyyi Kulak";
    let selected_date = "23 February 2024";
    let selected_time = "11.00";

    let departments = [];
    let doctors = [];
    let dates = [];
    let times = [];

    async function getDepartments(){
        try {
             departments = await backend.list_departments(id);
             console.log(departments);
        }
        catch (error){
            console.error("Fetching departments failed.", error);
        }        
    }
    
    async function addDepartment(){
        try {
            console.log(await backend.add_department(id, department_name));
        }
        catch(error){
            console.error("Adding department failed.", error);
        }
    }

    async function getDoctors(){
        try {
            doctors = await backend.list_doctors(id, department_name);
            console.log(doctors);
        }
        catch (error){
            console.error("Fetching doctors failed.", error);
        }
    }

    async function addDoctor(){
        try {
            console.log(await backend.add_doctor(id, department_name, doctor_name));
        }
        catch(error){
            console.error("Adding doctor failed.", error);
        }
    }

    async function getDates(){
        try {
            dates = await backend.list_dates(id, department_name, doctor_name);
            console.log(dates);
        }
        catch (error){
            console.error("Fetching appointment dates failed.", error);
        }
    }

    async function addDate(){
        try {
            console.log(await backend.add_date(id, department_name, doctor_name, selected_date));
        }
        catch(error){
            console.error("Adding date failed.", error);
        }
    }

    async function getTimes(){
        try {
            times = await backend.list_times(id, department_name, doctor_name, selected_date);
            console.log(times);
        }
        catch (error){
            console.error("Fetching appointment times failed.", error);
        }
    }

    async function addTime(){
        try {
            console.log(await backend.add_time(id, department_name, doctor_name, selected_date, selected_time));
        }
        catch(error){
            console.error("Adding time failed.", error);
        }
    }
</script>

<main>
    <div class="departments">
        Current Departments:
        <button on:click={getDepartments}>List Departments</button>
        <button on:click={addDepartment}>Add Departments</button> 
        {#if departments}
            {#each departments as department}
                <div>{department}</div>
            {/each}
        {/if}
        <div>
    </div>
    <div class="doctors">
        Current Doctors: 
        <button on:click={getDoctors}>List Doctors</button>
            <select name="department-name" id="department">
                <option value="">Select Department</option>
                {#if departments}
                {#each departments as department}
                    <option value={department}>{department}</option>
                {/each}
            {/if}
            </select>
        <button on:click={addDoctor}>Add Doctor</button> 
        {#if doctors}
        {#each doctors as doctor}
            <div>{doctor}</div>
        {/each}
        {/if}
    </div>
    <div class="dates">
        Current Appointment Dates For Doctor:
        <button on:click={getDates}>List Appointment Dates</button>
        <button on:click={addDate}>Add Date</button> 
        {#if dates}
        {#each dates as date}
            <div>{date}</div>
        {/each}
        {/if}
    </div>
    <div class="times">
        Current Appointment Times For Selected Dates:
        <button on:click={getTimes}>List Appointment Times</button>
        <button on:click={addTime}>Add Time</button>
        {#if times}
        {#each times as time}
            <div>{time}</div>
        {/each}
        {/if}
    </div>
</main>

<style>

</style>
