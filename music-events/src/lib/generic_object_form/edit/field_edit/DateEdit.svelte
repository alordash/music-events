<script lang="ts">
	export let fieldName: string;
	export let value: string;
</script>

<label for="inputDate" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<input type="datetime-local" class="form-control" id="inputDate" bind:value />
</div>
