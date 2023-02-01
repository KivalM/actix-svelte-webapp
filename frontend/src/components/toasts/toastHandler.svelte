<script lang="ts">
	import { toastStore, type Toast } from './toast';

	let localToast: Toast[] = [];

	toastStore.subscribe((toast) => {
		localToast = toast;
	});
</script>

<div class="toast toast-top toast-center">
	{#each localToast as toast}
		<div class="alert alert-{toast.type}">
			<div>
				<div>{toast.message}</div>
			</div>
		</div>
	{/each}
</div>
