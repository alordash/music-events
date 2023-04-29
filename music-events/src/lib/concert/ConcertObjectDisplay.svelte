<script>
	import ObjectDisplay from "$lib/generic_object_display/GenericObjectDisplay.svelte";
	import { Concert } from "./Concert";

    const concert = new Concert(90, '2023', 555, 'Kavo', 'NAME');
</script>
<ObjectDisplay displayObject={concert}/>