<script lang="ts">
	import { page } from '$app/stores';

	const currentUrl = $page.url.toString();
	const loadUrl = currentUrl.substring(0, currentUrl.lastIndexOf('/')) + '/load';
	const createUrl = currentUrl.substring(0, currentUrl.lastIndexOf('/')) + '/create';
</script>

<ul class="nav nav-tabs">
	<li class="nav-item">
		<a id="loadTab" class="nav-link active" aria-current="page" href={loadUrl}>Load</a>
	</li>
	<li class="nav-item">
		<a id="createTab" class="nav-link active" aria-current="page" href={createUrl}>Create</a>
	</li>
</ul>
