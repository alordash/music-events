<script>
	import { Concert } from '$lib/concert/concert';
	import ObjectDisplay from './ObjectDisplay.svelte';
	import { MyObject } from './my_object';
	// let someObject = new MyObject('KAVO', 12333);
    let someObject = new Concert(123, '2023', 999, 'Pushkina St.', 'ENT');
</script>

<ObjectDisplay myObject={someObject} />
