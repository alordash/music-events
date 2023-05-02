<script lang="ts">
	export let fieldName: string;
	export let value: string;
</script>

<label for="inputName" class="col col-form-label">{fieldName}:</label>
<div class="col">
	<input type="text" class="form-control" id="inputName" bind:value />
</div>
