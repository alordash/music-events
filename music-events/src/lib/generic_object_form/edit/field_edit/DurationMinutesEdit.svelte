<script lang="ts">
	export let fieldName: string;
	export let value: number;
</script>

<label for="inputDuration" class="col col-form-label">{fieldName}:</label>
<div class="col input-group">
	<input type="number" class="form-control w-auto" id="inputDuration" bind:value />
	<span class="input-group-text">m</span>
</div>
