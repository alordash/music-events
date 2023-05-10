<script lang="ts">
	export let fieldName: string;
	export let value: number;
</script>

<label for="inputNumber" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<input type="number" class="form-control" id="inputNumber" bind:value />
</div>
