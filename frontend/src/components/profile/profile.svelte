<script lang="ts">
	import { backendGet, backendPost } from '$lib/utils/requests';
	import { onMount } from 'svelte';
	import { insertToastString } from '../../components/toasts/toast';

	interface User {
		username: string;
		email: string;
		name: string;
		created_at: string;
	}

	let user: User = {
		username: '',
		email: '',
		name: '',
		created_at: ''
	};

	onMount(async () => {
		let response = await backendPost('/api/auth/user', {});
		if (response.status == 200) {
			user = await response.json();
		} else {
			insertToastString('Error fetching user, You likely need to sign in', 'error');
			return;
		}
	});
</script>

<div>
	<!-- Central box -->
	<div class="text-center">
		<div class="text-3xl">Profile</div>
		<div class="divider" />

		<div class="text-xl">Username: {user.username}</div>
		<div class="text-xl">Email: {user.email}</div>
		<div class="text-xl">Name: {user.name}</div>
		<div class="text-xl">Created at: {user.created_at}</div>
	</div>
</div>
