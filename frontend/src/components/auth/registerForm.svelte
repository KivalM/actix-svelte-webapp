<script lang="ts">
	import { backendPost } from '$lib/utils/requests';

	let email_element: HTMLInputElement;
	let username_element: HTMLInputElement;
	let name_element: HTMLInputElement;
	let password_element: HTMLInputElement;

	let email: string = '';
	let username: string = '';
	let name: string = '';
	let password: string = '';

	async function submit() {
		// remove all error classes
		email_element.classList.remove('input-error');
		username_element.classList.remove('input-error');
		name_element.classList.remove('input-error');
		password_element.classList.remove('input-error');

		if (email_element.value == '') {
			email_element.classList.add('input-error');
			email_element.focus();
			return;
		}

		if (username_element.value == '') {
			username_element.classList.add('input-error');
			username_element.focus();
			return;
		}

		if (name_element.value == '') {
			name_element.classList.add('input-error');
			name_element.focus();
			return;
		}

		if (password_element.value == '') {
			password_element.classList.add('input-error');
			password_element.focus();
			return;
		}

		let register_info = {
			username: username,
			email: email,
			name: name,
			password: password
		};

		let response = await backendPost('/api/auth/register', register_info);

		if (response.status == 200) {
			// redirect to login page
			window.location.href = '/auth/login';
		} else {
			// show error message
			console.log(response);
		}
	}
</script>

<div class=" p-10">
	<div class="text-center text-3xl">Register Form</div>
	<div class="divider" />

	<!-- email  -->
	<div class="form-control w-full ">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			<span class="label-text">Email</span>
		</label>
		<input
			bind:value={email}
			bind:this={email_element}
			type="email"
			placeholder="Email"
			class="input input-bordered w-full "
		/>
	</div>

	<!-- username  -->
	<div class="form-control w-full ">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			<span class="label-text">Username</span>
		</label>
		<input
			bind:value={username}
			bind:this={username_element}
			type="text"
			placeholder="Username"
			class="input input-bordered w-full "
		/>
	</div>

	<!-- Name  -->
	<div class="form-control w-full ">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			<span class="label-text">Full name</span>
		</label>
		<input
			bind:value={name}
			bind:this={name_element}
			type="text"
			placeholder="Full Name"
			class="input input-bordered w-full "
		/>
	</div>

	<!-- Password -->
	<div class="form-control w-full ">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			<span class="label-text">Password</span>
		</label>
		<input
			type="password"
			bind:this={password_element}
			bind:value={password}
			placeholder="*********"
			class="input input-bordered w-full "
		/>
	</div>

	<!-- Submit button -->
	<button class="btn btn-primary mt-5" on:click={submit}> Create User </button>
</div>
