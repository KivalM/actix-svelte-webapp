<div class="navbar bg-base-100">
	<div class="navbar-start">
		<!-- dropdown -->
		<div class="dropdown">
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<!-- svelte-ignore a11y-label-has-associated-control -->
			<label tabindex="0" class="btn btn-ghost lg:hidden">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 6h16M4 12h8m-8 6h16"
					/></svg
				>
			</label>
			<ul class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
				<li><a href="/">Home</a></li>
				<li><a href="/auth/register">Register</a></li>
				<li><a href="/auth/login">Login</a></li>
				<li><a href="/profile">Profile</a></li>
			</ul>
		</div>
		<!-- /dropdown -->

		<!-- Title -->
		<a class="btn btn-ghost normal-case text-xl" href="/">Example</a>
	</div>

	<!-- /nav start -->

	<!-- nav center -->
	<div class="navbar-center hidden lg:flex">
		<ul class="menu menu-horizontal px-1">
			<li class="border-accent border-2 rounded-lg mx-2"><a href="/">Home</a></li>
			<li class="border-accent border-2 rounded-lg mx-2"><a href="/auth/register">Register</a></li>
			<li class="border-accent border-2 rounded-lg mx-2"><a href="/auth/login">Login</a></li>
			<li class="border-accent border-2 rounded-lg mx-2"><a href="/profile">Profile</a></li>
		</ul>
	</div>

	<!-- nav end -->
	<div class="navbar-end">
		<a
			class="btn btn-secondary"
			href="https://github.com/KivalM/rust-actix-auth-flow"
			target="_blank"
			rel="noreferrer">View the source</a
		>
	</div>
</div>
