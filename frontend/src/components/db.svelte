<script lang="ts">
	import { backendGet } from '$lib/utils/requests';
	import { onMount } from 'svelte';
	import { insertToastString } from './toasts/toast';
	import type { User } from './user';

	let users: User[] = [];

	async function getAllData() {
		let response = await backendGet('/api/auth/users');

		if (response.status == 200) {
			users = await response.json();
		} else {
			// show error message
			let body: string = await response.text();
			insertToastString(body, 'error');
		}
	}

	onMount(async () => {
		await getAllData();
	});
</script>

<!-- This will be a table with -->
<div>
	<table class="table table-normal table-zebra table-striped">
		<thead>
			<tr>
				<th>Username</th>
				<th>Email</th>
				<th>Name</th>
				<th>Created at (UTC)</th>
			</tr>
		</thead>
		<tbody>
			{#each users as user}
				<tr>
					<td>{user.username}</td>
					<td>{user.email}</td>
					<td>{user.name}</td>
					<td>{user.created_at}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
