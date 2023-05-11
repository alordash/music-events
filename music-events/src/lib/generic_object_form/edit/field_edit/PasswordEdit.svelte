<script lang="ts">
	export let fieldName: string;
	export let value: string;
</script>

<label for="inputName" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<input type="password" class="form-control view" id="inputName" bind:value />
</div>
