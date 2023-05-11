<div class="container w-75">
	<nav class="navbar navbar-expand-lg bg-body-tertiary">
		<div class="container-fluid">
			<a class="navbar-brand" href="/">Home</a>
			<button
				class="navbar-toggler"
				type="button"
				data-bs-toggle="collapse"
				data-bs-target="#navbarNavAltMarkup"
				aria-controls="navbarNavAltMarkup"
				aria-expanded="false"
				aria-label="Toggle navigation"
			>
				<span class="navbar-toggler-icon" />
			</button>
			<div class="collapse navbar-collapse" id="navbarNavAltMarkup">
				<div class="navbar-nav">
					<a class="nav-link" href="/events/load/">Events</a>
					<a class="nav-link" href="/concerts/load/">Concerts</a>
					<a class="nav-link" href="/groups/load/">Groups</a>
					<a class="nav-link" href="/participants/load/">Participants</a>
					<a class="nav-link" href="/artists/load/">Artists</a>
					<a class="nav-link" href="/group_artists/load/">Group artists</a>
					<a class="nav-link" href="/viewer_seats/load/">Viewer seats</a>
					<a class="nav-link" href="/viewers/load/">Viewers</a>
					<a class="nav-link" href="/persons/load/">Persons</a>
				</div>
			</div>
		</div>
	</nav>

	<slot />
</div>
