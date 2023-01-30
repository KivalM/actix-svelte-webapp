<script lang="ts">
	import { toastStore, type Toast } from './toast';

	let localToast: Toast[] = [];

	toastStore.subscribe((toast) => {
		localToast = toast;
	});
</script>

<div class="toast toast-bottom toast-end">
	{#each localToast as toast}
		<div class="alert alert-{toast.type}">
			<div>
				<span>{toast.message}</span>
			</div>
		</div>
	{/each}
</div>
