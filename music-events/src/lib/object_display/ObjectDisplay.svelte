<script lang="ts">
	export let myObject: Object;
    let objectKeyValues = Object.entries(myObject);
</script>

<div>
    {#each Object.entries(myObject) as [key, value]}
        <p>{key}: {value}</p>
    {/each}
	<!-- <p>id: {concert.id}</p>
	<p>name: <strong>{concert.name}</strong></p>
	<p>date: {concert.date}</p>
	<p>duration (minutes): {concert.durationMinutes}</p>
	<p>address: {concert.address}</p> -->
</div>