<script lang="ts">
	let loadTab: HTMLElement;
	let createTab: HTMLElement;
	let editTab: HTMLElement;

	let tabs: Map<string, HTMLElement>;

	function onTabClick(e: Event) {
		const id = (<HTMLElement>e.target).id;
		const targetTab = tabs.get(id);
		tabs.forEach((v, _) => v?.classList.remove('active'));
		targetTab?.classList.add('active');
	}

	$: {
		tabs = new Map([
			[loadTab?.id, loadTab],
			[createTab?.id, createTab],
			[editTab?.id, editTab]
		]);
	}
</script>

<ul class="nav nav-tabs">
	<li class="nav-item" on:keypress={onTabClick} on:click={onTabClick}>
		<a
			id="loadTab"
			bind:this={loadTab}
			class="nav-link active"
			aria-current="page"
			href="/models/repertoires/load/">Load</a
		>
	</li>
	<li class="nav-item" on:keypress={onTabClick} on:click={onTabClick}>
		<a id="createTab" bind:this={createTab} class="nav-link" href="/models/repertoires/create/">Create</a>
	</li>
	<li class="nav-item" on:keypress={onTabClick} on:click={onTabClick}>
		<a id="editTab" bind:this={editTab} class="nav-link" href="/models/repertoires/edit/">Edit</a>
	</li>
</ul>

<slot />
