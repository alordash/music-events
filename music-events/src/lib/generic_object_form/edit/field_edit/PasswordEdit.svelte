<script lang="ts">
	export let fieldName: string;
	export let value: string;
</script>

<label for="inputPassword" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<input type="password" class="form-control view" id="inputPassword" bind:value />
</div>
