<script lang="ts">
	export let id: number;
</script>

<span class="badge text-bg-dark">
	{#if id == 0}
		??
	{:else}
		#{id}
	{/if}
</span>
