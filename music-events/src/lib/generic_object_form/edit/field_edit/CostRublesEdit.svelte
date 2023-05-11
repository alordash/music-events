<script lang="ts">
	export let fieldName: string;
	export let value: number;
</script>

<label for="inputCostRubles" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<div class="input-group">
		<input type="number" class="form-control w-auto" id="inputCostRubles" min=0 bind:value />
		<span class="input-group-text">₽</span>
	</div>
</div>
