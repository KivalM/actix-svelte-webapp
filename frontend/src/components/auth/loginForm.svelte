<script lang="ts">
	import { backendPost } from '$lib/utils/requests';
	import { insertToastString } from '../toasts/toast';

	let email: string = '';
	let password: string = '';

	async function submit() {
		let login_info = {
			email: email,
			password: password
		};

		let response = await backendPost('/api/auth/login', login_info);

		if (response.status == 200) {
			// redirect to login page
			insertToastString('Login successful, redirecting', 'success');
			window.location.href = '/';
		} else {
			// show error message
			console.log(response);
		}
	}
</script>

<div class=" p-10">
	<div class="text-center text-3xl">Login Form</div>
	<div class="divider" />

	<!-- username or email -->
	<div class="form-control w-full ">
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<label class="label">
			<span class="label-text">Email</span>
		</label>
		<input
			bind:value={email}
			type="text"
			placeholder="Email"
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
			placeholder="*********"
			class="input input-bordered w-full "
			bind:value={password}
		/>
	</div>

	<!-- Submit button -->
	<button class="btn btn-primary mt-5" on:click={submit}> Submit </button>
</div>
