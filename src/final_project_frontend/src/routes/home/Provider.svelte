<script>
    import { backend } from "$lib/canisters";

    export let id;
    let department_name = '';
    let doctor_name = '';
    let selected_date = '';
    let selected_time = '';

    let departments = [];
    let doctors = [];
    let dates = [];
    let times = [];

    async function getDepartments() {
        try {
            departments = await backend.list_departments(id);
            console.log(departments);
        } catch (error) {
            console.error("Fetching departments failed.", error);
        }
    }

    async function addDepartment() {
        try {
            console.log(await backend.add_department(id, department_name));
        } catch (error) {
            console.error("Adding department failed.", error);
        }
    }

    async function getDoctors() {
        try {
            doctors = await backend.list_doctors(id, department_name);
            console.log(doctors);
        } catch (error) {
            console.error("Fetching doctors failed.", error);
        }
    }

    async function addDoctor() {
        try {
            console.log(await backend.add_doctor(id, department_name, doctor_name));
        } catch (error) {
            console.error("Adding doctor failed.", error);
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

    async function addDate() {
        try {
            console.log(await backend.add_date(id, department_name, doctor_name, selected_date));
        } catch (error) {
            console.error("Adding date failed.", error);
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

    async function addTime() {
        try {
            console.log(await backend.add_time(id, department_name, doctor_name, selected_date, selected_time));
        } catch (error) {
            console.error("Adding time failed.", error);
        }
    }


</script>

<main>
    <div class="container">
        <div class="section">
            <h2>Departments</h2>
            <button on:click={getDepartments}>List Departments</button>
            {#if departments}
                {#each departments as department}
                    <div class="department-option">{department}</div>
                {/each}
            {/if}
            <input type="text" placeholder="Enter department name" bind:value={department_name}>
            <button on:click={addDepartment}>Add Department</button>
        </div>
        <div class="section">
            <h2>Doctors</h2>
            <button on:click={getDoctors}>List Doctors</button>
            <select name="department-name" id="department" bind:value={department_name}>
                <option value="">Select Department</option>
                {#if departments}
                    {#each departments as department}
                        <option value={department}>{department}</option>
                    {/each}
                {/if}
            </select>
            <input type="text" placeholder="Enter doctor name" bind:value={doctor_name}>
            <button on:click={addDoctor}>Add Doctor</button>
            {#if doctors}
                {#each doctors as doctor}
                    <div class="doctor-option">{doctor}</div>
                {/each}
            {/if}
        </div>
        <div class="section">
            <h2>Appointment Dates</h2>
            <button on:click={getDates}>List Appointment Dates</button>
            <select name="doctor-name" id="doctor" bind:value={doctor_name}>
                <option value="">Select Doctor</option>
                {#if doctors}
                    {#each doctors as doctor}
                        <option value={doctor}>{doctor}</option>
                    {/each}
                {/if}
            </select>
            <input type="date" bind:value={selected_date}>
            <button on:click={addDate}>Add Date</button>
            {#if dates}
                {#each dates as date}
                    <div class="date-option">{date}</div>
                {/each}
            {/if}
        </div>
        <div class="section">
            <h2>Appointment Times</h2>
            <button on:click={getTimes}>List Appointment Times</button>
            <select name="selected-date" id="date" bind:value={selected_date}>
                <option value="">Select Date</option>
                {#if dates}
                    {#each dates as date}
                        <option value={date}>{date}</option>
                    {/each}
                {/if}
            </select>
            <input type="time" bind:value={selected_time}>
            <button on:click={addTime}>Add Time</button>
            {#if times}
                {#each times as time}
                    <div class="time-option">{time}</div>
                {/each}
            {/if}
        </div>
    </div>
</main>

<style>
    .container {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
}

.section {
    flex-basis: 45%;
    margin-bottom: 20px;
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

input[type="text"],
input[type="date"],
input[type="time"],
select {
    width: calc(100% - 12px);
    padding: 8px;
    margin-bottom: 10px;
    border: 1px solid #ccc;
    border-radius: 4px;
}

.date-option,
.time-option,
.department-option,
.doctor-option {
    margin-bottom: 5px;
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background-color: #f9f9f9;
}

</style>