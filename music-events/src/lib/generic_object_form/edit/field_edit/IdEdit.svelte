<script lang="ts">
	export let fieldName: string;
	export let id: number;
</script>

<label for="staticId" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<input type="text" readonly class="form-control-plaintext" id="staticId" bind:value={id} />
</div>
